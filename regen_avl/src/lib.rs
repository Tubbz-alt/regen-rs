use std::sync::Arc;
use std::cmp::{Ordering, max};
use regen_store::{MutableMap, Map, Result, StoreError};
use protobuf::Message;
use std::sync::atomic::AtomicPtr;
use crate::NodeRef::{HashRef, MemRef, NoRef};

mod lru;
mod codec;

pub trait Reader<T> {
    fn read(&self, buf: &[u8]) -> Result<T>;
}

pub trait Writer<T> {
    fn write(&self, k: &T) -> Vec<u8>;
}

pub trait Marshaller<T>: Reader<T> + Writer<T> {}

pub trait Hasher {
    fn input(&self, bytes: &[u8]);
    fn result(&self) -> Vec<u8>;
    fn output_size(&self) -> usize;
}

pub struct TreeContext<K, V> {
    key_to_canonical_bytes: Box<dyn Writer<K>>,
    value_to_canonical_bytes: Box<dyn Writer<V>>,
    new_digest: fn() -> Box<dyn Hasher>,
    store: Box<dyn MutableMap<Vec<u8>, Node<K, V>>>,
    comparator: fn(K) -> Ordering,
}

#[derive(Clone)]
struct NodeData<K, V> {
    key: K,
    value: V,
    height: u32,
    rank: u64,
}

#[derive(Clone)]
struct Node<K, V> {
    data: Arc<NodeData<K, V>>,
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
    hash: Option<Vec<u8>>,
}

enum NodeRef<K, V> {
    HashRef(Vec<u8>),
    MemRef(Arc<Node<K, V>>),
    NoRef,
}

#[derive(Clone)]
struct EditNode<K, V> {
    key: K,
    value: V,
    left: EditNodeRef<K, V>,
    right: EditNodeRef<K, V>,
    height: i32,
    rank: i64,
}

#[derive(Clone)]
enum EditNodeRef<K, V> {
    PersistentNodeRef(NodeRef<K, V>),
    EditNodeRef(Box<EditNode<K, V>>),
}

impl<K, V> Node<K, V> {
    fn calc_hash(&self, ctx: &mut TreeContext<K, V>, serialize: bool) -> Result<(Option<Self>)> {
        match &self.hash {
            // hash is already calculated
            Some(h) => {
                if serialize {
                    if ctx.store.has(h)? {
                        return Ok((None));
                    } else {
                        let new_left = self.left.calc_hash(ctx, serialize)?;
                        let new_right = self.right.calc_hash(ctx, serialize)?;
                        if new_left.is_some() || new_right.is_some() {
                            let new_node = Node {
                                data: self.data.clone(),
                                left: new_left.unwrap_or_else(|| self.left.clone()),
                                right: new_right.unwrap_or_else(|| self.right.clone()),
                                hash: Some(h.clone()),
                            };
                            ctx.store.set(&h, &new_node)?;
                            return Ok(Some(new_node));
                        } else {
                            ctx.store.set(&h, self)?;
                            return Ok(None);
                        }
                    }
                } else {
                    return Ok(None);
                }
            }
            // no hash yet, fall through
            None => {}
        }
        let data = &self.data;
        let key_bytes = ctx.key_to_canonical_bytes.write(&data.key);
        let value_bytes = ctx.value_to_canonical_bytes.write(&data.value);
        let mut hasher = (ctx.new_digest)();
        hasher.input(key_bytes.as_slice());
        hasher.input(value_bytes.as_slice());
        let new_left = self.left.calc_hash(ctx, serialize)?.unwrap_or_else(|| self.left.clone());
        let new_right = self.right.calc_hash(ctx, serialize)?.unwrap_or_else(|| self.right.clone());
        hasher.input(&new_left.get_hash());
        hasher.input(&new_right.get_hash());
        let hash = hasher.result();
        let new_node = Node {
            data: data.clone(),
            left: new_left,
            right: new_right,
            hash: Some(hash.clone()),
        };
        if serialize {
            ctx.store.set(&hash, &new_node)?;
        }
        Ok(Some(new_node))
    }
}

impl<K, V> Clone for NodeRef<K, V> {
    fn clone(&self) -> Self {
        match self {
            HashRef(h) => HashRef(h.clone()),
            MemRef(n) => MemRef(n.clone()),
            NoRef => NoRef
        }
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}

const EMPTY: Vec<u8> = Vec::new();

impl<K, V> NodeRef<K, V> {
    fn get_hash(&self) -> Vec<u8> {
        match self {
            NodeRef::HashRef(h) => h.clone(),
            NodeRef::MemRef(node) => node.hash.clone().unwrap_or(EMPTY),
            NodeRef::NoRef => EMPTY,
        }
    }

    fn calc_hash(&self, ctx: &mut TreeContext<K, V>, serialize: bool) -> Result<Option<NodeRef<K, V>>> {
        match self {
            NodeRef::HashRef(_) => Ok(None),
            NodeRef::MemRef(node) => {
                match node.calc_hash(ctx, serialize)? {
                    Some(new_node) => {
                        if serialize {
                            let hash = new_node.hash.unwrap_or_else(|| Vec::new());
                            Ok(Some(HashRef(hash.clone())))
                        } else {
                            Ok(Some(MemRef(Arc::new(new_node))))
                        }
                    }
                    None => {
                        let hash = node.hash.clone().unwrap_or(EMPTY);
                        if serialize {
                            Ok(Some(HashRef(hash.clone())))
                        } else {
                            Ok(None)
                        }
                    }
                }
            }
            NoRef => Ok(None)
        }
    }
}

struct NodeStore<K, V> {
    store: Box<dyn MutableMap<Vec<u8>, Vec<u8>>>,
    key_marshaller: Box<dyn Marshaller<K>>,
    value_marshaller: Box<dyn Marshaller<V>>,
}

const NODE_HASH_NODE_PREFIX: u8 = 0;

fn node_hash__node__key(node_hash: &[u8]) -> Vec<u8> {
    let mut res = Vec::with_capacity(node_hash.len() + 1);
    res.push(NODE_HASH_NODE_PREFIX);
    res.extend_from_slice(node_hash);
    res
}

fn read_node_ref<K, V>(hash: Vec<u8>) -> NodeRef<K, V> {
    if hash.len() == 0 {
        return NodeRef::NoRef;
    }
    NodeRef::HashRef(hash)
}

impl<K, V> Map<Vec<u8>, Node<K, V>> for NodeStore<K, V> {
    fn get(&self, hash: &Vec<u8>) -> Result<Option<Node<K, V>>> {
        let res = self.store.get(&node_hash__node__key(hash))?;
        match res {
            None => Ok(None),
            Some(bytes) => {
                let mut proto_node: codec::Node = protobuf::parse_from_bytes(&bytes)?;
                let key = self.key_marshaller.read(proto_node.get_key())?;
                let value = self.value_marshaller.read(proto_node.get_value())?;
                let mut left: NodeRef<K, V> = read_node_ref(proto_node.take_left());
                let right = read_node_ref(proto_node.take_right());
                let opt_hash = if hash.len() == 0 {
                    None
                } else {
                    Some(hash.clone())
                };
                Ok(Some(Node {
                    data: Arc::new(NodeData {
                        key,
                        value,
                        height: proto_node.height,
                        rank: proto_node.rank,
                    }),
                    left,
                    right,
                    hash: opt_hash,
                }))
            }
        }
    }

    fn has(&self, hash: &Vec<u8>) -> Result<bool> {
        self.has(&node_hash__node__key(hash))
    }
}

impl<K, V> MutableMap<Vec<u8>, Node<K, V>> for NodeStore<K, V> {
    fn set(&mut self, key: &Vec<u8>, value: &Node<K, V>) -> Result<()> {
        let data = &value.data;
        let key_bytes = self.key_marshaller.write(&data.key);
        let value_bytes = self.value_marshaller.write(&data.value);
        let proto_node = codec::Node {
            key: key_bytes,
            value: value_bytes,
            left: value.left.get_hash().clone(),
            right: value.right.get_hash().clone(),
            height: data.height,
            rank: data.rank,
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        };
        let proto_bytes = proto_node.write_to_bytes()?;
        self.store.set(&node_hash__node__key(key), &proto_bytes)
    }

    fn delete(&mut self, key: &Vec<u8>) -> Result<()> {
        self.store.delete(&node_hash__node__key(key))
    }
}


//trait NodeContext<K: Ord, V> {
//}
//
//trait Node<K: Ord, V> {
//    fn key(&self) -> &K;
//    fn value(&self) -> &V;
//    fn left(&self) -> &Option<Arc<dyn Node<K, V>>>;
//    fn right(&self) -> &Option<Arc<dyn Node<K, V>>>;
//    fn height(&self) -> i32;
//    fn rank(&self) -> i64;
//    fn update_value(&self, v: &V) -> Arc<dyn Node<K, V>>;
//}
//
//struct NodeImpl<K: Ord, V> {
//    key: K,
//    value: V,
//    left: Option<Arc<dyn Node<K, V>>>,
//    right: Option<Arc<dyn Node<K, V>>>,
//    height: i32,
//    rank: i64,
//    hash: Vec<u8>,
//}
//
//impl<K: Ord + Clone, V: Clone> Node<K, V> for NodeImpl<K, V> {
//    fn key(&self) -> &K {
//        return &self.key;
//    }
//
//    fn value(&self) -> &V {
//        return &self.value;
//    }
//
//    fn left(&self) -> &Option<Arc<dyn Node<K, V>>> {
//        return &self.left;
//    }
//
//    fn right(&self) -> &Option<Arc<dyn Node<K, V>>> {
//        return &self.right;
//    }
//
//    fn height(&self) -> i32 {
//        return self.height;
//    }
//
//    fn rank(&self) -> i64 {
//        return self.rank;
//    }
//
//    fn update_value(&self, v: &V) -> Arc<dyn Node<K, V>> {
//        Arc::from(NodeImpl {
//            key: self.key.clone(),
//            value: v.clone(),
//            left: self.left.clone(),
//            right: self.right.clone(),
//            height: self.height,
//            rank: self.rank,
//            hash: vec![]
//        })
//    }
//}
//
//
//type NodeFactory<K, V> = fn(key: &K, value: &V, left: &Option<Arc<dyn Node<K, V>>>, right: &Option<Arc<dyn Node<K, V>>>) -> Arc<dyn Node<K, V>>;
//
impl<K, V> NodeRef<K, V> {
    fn get_node(&self, ctx: &TreeContext<K, V>) -> Result<Option<Arc<Node<K, V>>>> {
        match self {
            HashRef(h) =>
                match ctx.store.get(&h)? {
                    None => Ok(None),
                    Some(node) => Ok(Some(Arc::new(node))),
                }
            MemRef(node) => Ok(Some(node.clone())),
            NoRef => Ok(None),
        }
    }
}

fn node_height<K, V>(node: &Option<Arc<Node<K, V>>>) -> i32 {
    match node {
        None => 0,
        Some(node) => node.data.height as i32,
    }
}

fn node_rank<K, V>(node: &Option<Arc<Node<K, V>>>) -> u64 {
    match node {
        None => 0,
        Some(node) => node.data.rank
    }
}

fn make_node<K: Clone, V: Clone>(ctx: &TreeContext<K, V>, key: &K, value: &V, left: NodeRef<K, V>, right: NodeRef<K, V>) -> Result<Node<K, V>> {
    let l = left.get_node(ctx)?;
    let r = right.get_node(ctx)?;
    Ok(Node {
        data: Arc::new(NodeData {
            key: key.clone(),
            value: value.clone(),
            height: (max(node_height(&l), node_height(&r))) as u32,
            rank: node_rank(&l) + node_rank(&r),
        }),
        left,
        right,
        hash: None
    })
}

impl<K: Clone, V: Clone> Node<K, V> {
    fn balance_factor(&self, ctx: &TreeContext<K, V>) -> Result<i32> {
        Ok(node_height(&self.left.get_node(ctx)?) - node_height(&self.right.get_node(ctx)?))
    }

    fn balance(&self, ctx: &TreeContext<K, V>) -> Result<Self> {
        let left = self.left.get_node(ctx)?;
        let right = self.right.get_node(ctx)?;
        let left_height = node_height(&left);
        let right_height = node_height(&right);
        let diff = left_height - right_height;
        let data = &self.data;
        let key = &data.key;
        let value = &data.value;
        // Left Big
        if diff == 2 {
            match left {
                None => panic!("illegal"),
                Some(l) => {
                    let bal_factor = l.balance_factor(ctx)?;
                    // Left Heavy
                    let l_data = &l.data;
                    if bal_factor >= 0 {
                        Ok(make_node(
                            ctx,
                            &l_data.key,
                            &l_data.value,
                            l.left.clone(),
                            MemRef(Arc::from(make_node(
                                ctx,
                                &key,
                                &value,
                                l.right.clone(),
                                self.right.clone()
                            )?)),
                        )?)
                    }
                    // Right Heavy
                    else {
                        match l.right.get_node(ctx)? {
                            None => panic!("illegal"),
                            Some(lr) => {
                                let lr_data = &lr.data;
                                Ok(make_node(
                                    ctx,
                                    &lr_data.key,
                                    &lr_data.value,
                                    MemRef(Arc::from(make_node(
                                        ctx,
                                        &l_data.key,
                                        &l_data.value,
                                        l.left.clone(),
                                        lr.left.clone()
                                    )?)),
                                    MemRef(Arc::from(make_node(
                                        ctx,
                                        key,
                                        value,
                                        lr.right.clone(),
                                        self.right.clone()
                                    )?))
                                )?)
                            }
                        }
                    }
                }
            }
        } else if diff == -2 {
            match right {
                None => panic!("illegal"),
                Some(r) => {
                    let bal_factor = r.balance_factor(ctx)?;
                    let r_data = &r.data;
                    if bal_factor > 0 {
                        match r.left.get_node(ctx)? {
                            None => panic!("illegal"),
                            Some(rl) => {
                                let rl_data = &rl.data;
                                Ok(make_node(
                                    ctx,
                                    &rl_data.key,
                                    &rl_data.value,
                                    MemRef(Arc::from(make_node(
                                        ctx,
                                        key,
                                        value,
                                        self.left.clone(),
                                        rl.left.clone()
                                    )?)),
                                    MemRef(Arc::from(make_node(
                                        ctx,
                                        &r_data.key,
                                        &r_data.value,
                                        rl.right.clone(),
                                        r.right.clone()
                                    )?)),
                                )?)
                            }
                        }
                    } else {
                        Ok(make_node(
                            ctx,
                            &r_data.key,
                            &r_data.value,
                            MemRef(Arc::from(make_node(
                                ctx,
                                key,
                                value,
                                self.left.clone(),
                                r.left.clone()
                            )?)),
                            r.right.clone()
                        )?)
                    }
                }
            }
        } else {
            Ok(make_node(ctx, key, value, self.left.clone(), self.right.clone())?)
        }
    }
}
//
//fn find_node<'a, K: Ord, V>(node: &'a Arc<dyn Node<K, V>>, key: &K) -> Option<&'a Arc<dyn Node<K, V>>> {
//    match key.cmp(node.key()) {
//        Ordering::Less => match node.left() {
//            None => None,
//            Some(l) => find_node(l, key)
//        },
//        Ordering::Greater => match node.right() {
//            None => None,
//            Some(r) => find_node(r, key)
//        },
//        Ordering::Equal => Some(node)
//    }
//}
//
//fn insert<K: Ord + Clone, V: Clone>(node: &Option<Arc<dyn Node<K, V>>>, key: &K, value: &V, make_node: &NodeFactory<K, V>) -> Arc<dyn Node<K, V>> {
//    match node {
//        None => make_node(key, value, &None, &None),
//        Some(n) => {
//            let nkey = n.key();
//            match key.cmp(nkey) {
//                Ordering::Less => balance(nkey, n.value(), &Some(insert(n.left(), key, value, make_node)), n.right(), make_node),
//                Ordering::Greater => balance(nkey, n.value(), n.left(), &Some(insert(n.right(), key, value, make_node)), make_node),
//                Ordering::Equal => n.update_value(&value)
//            }
//        }
//    }
//}


// //         // Right Big
// //         diff == -2 -> {
// //             if (right == null) throw IllegalStateException()
// //             val bal_factor = right.balance_factor
// //             when {
// //                 // Left Heavy
// //                 bal_factor > 0 -> {
// //                     val rl = right.left
// //                     if (rl == null) throw IllegalStateException()
// //                     make_node(
// //                         rl.key, rl.value,
// //                         make_node(key, value, left, rl.left),
// //                         make_node(right.key, right.value, rl.right, right.right)
// //                     )
// //                 }
// //                 // Right Heavy
// //                 else -> make_node(right.key, right.value, make_node(key, value, left, right.left), right.right)
// //             }
// //         }
// //         else -> make_node(key, value, left, right)
// //     }
// // }


// fn main() {}

// trait KVStore {
//     fn get(&self, key: &[u8]) -> &[u8];
//     fn set(&self, key: &[u8], value: &[u8]);
//     fn delete(&self, key: &[u8]);
//     fn prefix_iterator(&self, prefix: &[u8]) -> Option<&dyn Iterator<&[u8], &[u8]>>;
// }

// trait Iterator<K, V> {
//     fn key(&self) -> K;
//     fn value(&self) -> V;
//     fn previous(&self) -> Option<&dyn Iterator<K, V>>;
//     fn next(&self) -> Option<&dyn Iterator<K, V>>;
// }

// trait Lookup<K, V> {
//    fn has(&self, key: K) -> bool;
//    fn get(&self, key: K) -> Option<V>;
// }

// trait Tree<K:Ord, V>: Lookup<K, V> {
//    fn set(&self, key: K, value: V) -> &dyn Tree<K, V>;
//    fn delete(&self, key: K, value: V) -> &dyn Tree<K, V>;
//    fn transient(&self) -> &dyn TransientTree<K, V>;
//    // fn iterator(k: K) -> Option<&dyn Iterator<K, V>>;
// }

// trait TransientTree<K:Ord, V>: Lookup<K, V> {
//    fn set(&self, key: K, value: V) -> &dyn TransientTree<K, V>;
//    fn delete(&self, key: K, value: V) -> &dyn TransientTree<K, V>;
//    fn persistent(&self) -> &dyn Tree<K, V>;
// }

// trait TreeStore<K:Ord, V> {
//    fn getByHeight(&self, height: u64) -> Option<&dyn Tree<K, V>>;
//    fn getByHash(&self, hash: &[u8]) -> Option<&dyn Tree<K, V>>;
//    fn store(&self, tree: impl Tree<K, V>) -> &[u8];
//    fn storeAsHead(&self, tree: &dyn Tree<K, V>) -> u64;
//    fn swap(&self, f: fn(tree: &dyn Tree<K, V>) -> &dyn Tree<K, V>);
//    // TODO add pruning support
// }

// trait Triple<E, A, V> {
//    fn e(&self) -> E;
//    fn a(&self) -> A;
//    fn v(&self) -> V;
// }

// trait Quad<E, A, V, G>: Triple<E, A, V> {
//    fn g(&self) -> G;
// }

// trait TripleStore<E, A, V> {

// }
