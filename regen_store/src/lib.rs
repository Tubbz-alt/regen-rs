use err_derive::Error;
use std::error::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error(display="{:?}", _0)]
    Other(String),
    #[error(display="{:?}", _0)]
    Wrap(Box<dyn Error>)
}

pub trait Entry<K, V> {
    fn key(&self) -> &K;
    fn value(&self) -> &V;
}

pub trait Iterator<K, V> {
    fn next(&self) -> Result<&dyn Entry<K, V>, StoreError>;
    fn release(&self);
}

pub trait ReadonlyKVStore<K, V> {
    fn get(&self, key: &K) -> Result<Option<&V>, StoreError>;
    fn has(&self, key: &K) -> Result<bool, StoreError>;
    fn iterator(&self, start: &K, end: &K) -> Result<Box<dyn Iterator<K, V>>, StoreError>;
    fn reverse_iterator(&self, start: &K, end: &K) -> Result<Box<dyn Iterator<K, V>>, StoreError>;
}

pub trait KVStore<K, V>: ReadonlyKVStore<K, V> {
    fn set(&mut self, key: &K, value: &V) -> Result<(), StoreError>;
    fn delete(&mut self, key: &K) -> Result<(), StoreError>;
}

pub trait Batch<'a, K, V>: KVStore<K, V> {
    fn new_batch(&'a mut self) -> &mut dyn Batch<'a, K, V>;
    fn write(&mut self) -> Result<(), StoreError>;
}

pub trait CommitKVStore<'a, K, V, Commit>: Batch<'a, K, V> {
    fn commit(&mut self) -> Result<Commit, StoreError>;
}


