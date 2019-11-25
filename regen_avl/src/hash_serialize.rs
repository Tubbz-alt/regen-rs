use crate::api::{Node, TreeContext, NodeRef, NodeData, NodeStore};
use regen_store::{MutableMap, Map, Result};
use std::sync::Arc;
use crate::api::NodeRef::{HashRef, MemRef, NoRef};
use crate::codec;
use protobuf::Message;

impl<K, V> Node<K, V> {
    fn calc_hash_serialize(&self, ctx: &mut TreeContext<K, V>, serialize: bool) -> Result<Option<Self>> {
        match &self.hash {
            // hash is already calculated
            Some(h) => {
                if serialize {
                    if ctx.get_store()?.has(h)? {
                        return Ok(None);
                    } else {
                        let new_left = self.left.calc_hash_serialize(ctx, serialize)?;
                        let new_right = self.right.calc_hash_serialize(ctx, serialize)?;
                        if new_left.is_some() || new_right.is_some() {
                            let new_node = Node {
                                data: self.data.clone(),
                                left: new_left.unwrap_or_else(|| self.left.clone()),
                                right: new_right.unwrap_or_else(|| self.right.clone()),
                                hash: Some(h.clone()),
                            };
                            ctx.get_store_mut()?.set(&h, &new_node)?;
                            return Ok(Some(new_node));
                        } else {
                            ctx.get_store_mut()?.set(&h, self)?;
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
        let new_left = self.left.calc_hash_serialize(ctx, serialize)?.unwrap_or_else(|| self.left.clone());
        let new_right = self.right.calc_hash_serialize(ctx, serialize)?.unwrap_or_else(|| self.right.clone());
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
            ctx.get_store_mut()?.set(&hash, &new_node)?;
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

    fn calc_hash_serialize(&self, ctx: &mut TreeContext<K, V>, serialize: bool) -> Result<Option<NodeRef<K, V>>> {
        match self {
            NodeRef::HashRef(_) => Ok(None),
            NodeRef::MemRef(node) => {
                match node.calc_hash_serialize(ctx, serialize)? {
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

impl<K, V> NodeStore<K, V> {
    pub fn get(&self, hash: &Vec<u8>) -> Result<Option<Node<K, V>>> {
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

    pub fn has(&self, hash: &Vec<u8>) -> Result<bool> {
        self.has(&node_hash__node__key(hash))
    }
}

impl<K, V> NodeStore<K, V> {
    pub fn set(&mut self, key: &Vec<u8>, value: &Node<K, V>) -> Result<()> {
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

    pub fn delete(&mut self, key: &Vec<u8>) -> Result<()> {
        self.store.delete(&node_hash__node__key(key))
    }
}


