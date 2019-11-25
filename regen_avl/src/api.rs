use std::sync::Arc;
use std::cmp::{Ordering};
use regen_store::{MutableMap, Result};

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
    pub key_to_canonical_bytes: Box<dyn Writer<K>>,
    pub value_to_canonical_bytes: Box<dyn Writer<V>>,
    pub new_digest: fn() -> Box<dyn Hasher>,
    pub store: Option<Box<NodeStore<K, V>>>,
    pub comparator: fn(&K, &K) -> Ordering,
}

pub struct NodeStore<K, V> {
    pub store: Box<dyn MutableMap<Vec<u8>, Vec<u8>>>,
    pub key_marshaller: Box<dyn Marshaller<K>>,
    pub value_marshaller: Box<dyn Marshaller<V>>,
}

#[derive(Clone)]
pub struct NodeData<K, V> {
    pub key: K,
    pub value: V,
    pub height: u32,
    pub rank: u64,
}

#[derive(Clone)]
pub struct Node<K, V> {
    pub data: Arc<NodeData<K, V>>,
    pub left: NodeRef<K, V>,
    pub right: NodeRef<K, V>,
    pub hash: Option<Vec<u8>>,
}

pub enum NodeRef<K, V> {
    HashRef(Vec<u8>),
    MemRef(Arc<Node<K, V>>),
    NoRef,
}

#[derive(Clone)]
pub struct EditNode<K, V> {
    pub key: K,
    pub value: V,
    pub left: EditNodeRef<K, V>,
    pub right: EditNodeRef<K, V>,
    pub height: i32,
    pub rank: i64,
}

#[derive(Clone)]
pub enum EditNodeRef<K, V> {
    PersistentNodeRef(NodeRef<K, V>),
    EditNodeRef(Box<EditNode<K, V>>),
}

