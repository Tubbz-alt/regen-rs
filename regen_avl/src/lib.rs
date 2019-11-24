use std::sync::Arc;
use std::cmp::Ordering;
use blake2::{Blake2b, Digest};
use std::error::Error;
use regen_store::BasicKVStore;
use protobuf::Message;

mod codec;

pub trait Marshaller<T> {
    fn read(&self, buf: &[u8]) -> Result<T, Box<dyn Error>>;
    fn write(&self, k: &T) -> Vec<u8>;
}

pub trait Hasher {
    fn input(&self, bytes: &[u8]);
    fn result(&self) -> Vec<u8>;
    fn output_size() -> usize;
}

pub struct TreeContext<K, V> {
    pub key_marshaller: Box<dyn Marshaller<K>>,
    pub value_marshaller: Box<dyn Marshaller<V>>,
    pub store: BasicKVStore<Vec<u8>, Vec<u8>>,
    pub new_digest: fn() -> Box<Hasher>,
}

struct Node<K, V> {
    key: K,
    value: V,
    left: NodeRef<K, V>,
    right: NodeRef<K, V>,
    height: i32,
    rank: i64,
    hash: Vec<u8>,
}

enum NodeRef<K, V> {
    HashRef(Vec<u8>),
    MemRef(Option<Arc<Node<K, V>>>),
}

struct EditNode<K, V> {
    key: K,
    value: V,
    left: EditNodeRef<K, V>,
    right: EditNodeRef<K, V>,
    height: i32,
    rank: i64,
}

enum EditNodeRef<K, V> {
    PersistentNodeRef(NodeRef<K, V>),
    EditNodeRef(EditNode<K, V>),
}

impl<K, V> NodeRef<K, V> {
    fn hash(&self) -> &Vec<u8> {
        match self {
            NodeRef::HashRef(hash) => hash,
            NodeRef::MemRef(Some(r)) => &r.hash,
            NodeRef::MemRef(None) => &Vec::new()
        }
    }
}

impl<K, V> Node<K, V> {
  fn calc_hash(&mut self, ctx: &mut TreeContext<K, V>, persist: bool) -> Result<&Vec<u8>, Box<dyn Error>> {
//        let have_hash = self.hash.len() > 0;
//        if have_hash  && persist {
//            return Ok(&self.hash)
//        }
//        let mut key_bytes = Vec::new();
//        let mut value_bytes = Vec::new();
//        let mut left_hash = Vec::new();
//        let mut right_hash = Vec::new();
//        if !have_hash {
//            key_bytes = ctx.key_marshaller.write(&self.key);
//            value_bytes = ctx.value_marshaller.write(&self.value);
//            let mut hasher = Blake2b::new();
//            hasher.input(key_bytes.as_slice());
//            hasher.input(value_bytes.as_slice());
//            left_hash = self.left.calc_hash(ctx, persist);
//            right_hash = self.right.calc_hash(ctx, persist);
//            hasher.input(left_hash);
//            hasher.input(right_hash);
//            self.hash = hasher.result;
//        }
//        if persist {
//            let mut kv_key = codec::KVKey::new();
//            kv_key.set_node_hash__node(hash.clone());
//            let key_proto = kv_key.write_to_bytes()?;
//            if ctx.store.has(&key_proto) {
//                return Ok(&self.hash)
//            }
//            if key_bytes.len() == 0 {
//                key_bytes = ctx.key_marshaller.write(&self.key);
//                value_bytes = ctx.value_marshaller.write(&self.value);
//                left_hash = self.left.calc_hash(ctx, persist);
//                right_hash = self.right.calc_hash(ctx, persist);
//            }
//            let kv_val = codec::Node {
//                key: key_bytes,
//                value: value_bytes,
//                left: left_hash,
//                right: right_hash,
//                height: 0,
//                rank: 0,
//                unknown_fields: Default::default(),
//                cached_size: Default::default()
//            };
//            let val_proto = kv_val.write_to_bytes()?;
//            ctx.store.set(key_proto.as_ref(), val_proto.as_ref())?;
//        }
//        Ok(&self.hash)
  }
//    fn calc_hash(&mut self, ctx: &mut TreeContext<K, V>, persist: bool) -> Result<&Vec<u8>, Box<dyn Error>> {
//        let have_hash = self.hash.len() > 0;
//        if have_hash  && persist {
//            return Ok(&self.hash)
//        }
//        let mut key_bytes = Vec::new();
//        let mut value_bytes = Vec::new();
//        let mut left_hash = Vec::new();
//        let mut right_hash = Vec::new();
//        if !have_hash {
//            key_bytes = ctx.key_marshaller.write(&self.key);
//            value_bytes = ctx.value_marshaller.write(&self.value);
//            let mut hasher = Blake2b::new();
//            hasher.input(key_bytes.as_slice());
//            hasher.input(value_bytes.as_slice());
//            left_hash = self.left.calc_hash(ctx, persist);
//            right_hash = self.right.calc_hash(ctx, persist);
//            hasher.input(left_hash);
//            hasher.input(right_hash);
//            self.hash = hasher.result;
//        }
//        if persist {
//            let mut kv_key = codec::KVKey::new();
//            kv_key.set_node_hash__node(hash.clone());
//            let key_proto = kv_key.write_to_bytes()?;
//            if ctx.store.has(&key_proto) {
//                return Ok(&self.hash)
//            }
//            if key_bytes.len() == 0 {
//                key_bytes = ctx.key_marshaller.write(&self.key);
//                value_bytes = ctx.value_marshaller.write(&self.value);
//                left_hash = self.left.calc_hash(ctx, persist);
//                right_hash = self.right.calc_hash(ctx, persist);
//            }
//            let kv_val = codec::Node {
//                key: key_bytes,
//                value: value_bytes,
//                left: left_hash,
//                right: right_hash,
//                height: 0,
//                rank: 0,
//                unknown_fields: Default::default(),
//                cached_size: Default::default()
//            };
//            let val_proto = kv_val.write_to_bytes()?;
//            ctx.store.set(key_proto.as_ref(), val_proto.as_ref())?;
//        }
//        Ok(&self.hash)
//    }
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
//fn node_height<K: Ord, V>(node: &Option<Arc<dyn Node<K, V>>>) -> i32 {
//    match node {
//        None => 0,
//        Some(n) => n.height()
//    }
//}
//
//fn balance_factor<K: Ord, V>(node: &Arc<dyn Node<K, V>>) -> i32 {
//    node_height(&node.left()) - node_height(&node.right())
//}
//
//type NodeFactory<K, V> = fn(key: &K, value: &V, left: &Option<Arc<dyn Node<K, V>>>, right: &Option<Arc<dyn Node<K, V>>>) -> Arc<dyn Node<K, V>>;
//
//fn balance<K: Ord + Clone, V: Clone>(key: &K, value: &V, left: &Option<Arc<dyn Node<K, V>>>, right: &Option<Arc<dyn Node<K, V>>>, make_node: &NodeFactory<K, V>) -> Arc<dyn Node<K, V>> {
//    let diff = node_height(&left) - node_height(&right);
//    // Left Big
//    if diff == 2 {
//        match left {
//            None => panic!("unexpected"),
//            Some(l) => {
//                let bal_factor = balance_factor(&l);
//                // Left Heavy
//                if bal_factor >= 0 {
//                    make_node(l.key(), l.value(), l.left(), &Some(make_node(key, value, l.right(), right)))
//                }
//                // Right Heavy
//                else {
//                    match l.right() {
//                        None => panic!("illegal"),
//                        Some(lr) =>
//                            make_node(lr.key(), lr.value(),
//                                      &Some(make_node(l.key(), l.value(), l.left(), lr.left())),
//                                      &Some(make_node(key, value, lr.right(), right)))
//                    }
//                }
//            }
//        }
//    } else if diff == -2 {
//        match right {
//            None => panic!("illegal"),
//            Some(r) => {
//                let bal_factor = balance_factor(&r);
//                if bal_factor > 0 {
//                    match r.left() {
//                        None => panic!("illegal"),
//                        Some(rl) =>
//                            make_node(
//                                rl.key(), rl.value(),
//                                &Some(make_node(key, value, left, rl.left())),
//                                &Some(make_node(r.key(), r.value(), rl.right(), r.right())),
//                            )
//                    }
//                } else {
//                    return make_node(r.key(), r.value(), &Some(make_node(key, value, left, r.left())), r.right());
//                }
//            }
//        }
//    } else {
//        return make_node(key, value, left, right);
//    }
//}
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
