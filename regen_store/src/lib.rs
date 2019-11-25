use err_derive;

#[derive(Debug, err_derive::Error)]
pub enum StoreError {
    #[error(display="{:?}", _0)]
    Other(String),
    #[error(display="{:?}", _0)]
    Wrap(Box<dyn std::error::Error>)
}

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub trait Entry<K, V> {
    fn key(&self) -> &K;
    fn value(&self) -> &V;
}

pub trait Iterator<K, V> {
    fn next(&self) -> Result<&dyn Entry<K, V>>;
    fn release(&self);
}

pub trait Map<K, V> {
    fn get(&self, key: &K) -> Result<Option<V>>;
    fn has(&self, key: &K) -> Result<bool>;
}

pub trait OrderedMap<K, V>: Map<K, V> {
    fn iterator(&self, start: &K, end: &K) -> Result<Box<dyn Iterator<K, V>>>;
    fn reverse_iterator(&self, start: &K, end: &K) -> Result<Box<dyn Iterator<K, V>>>;
}

pub trait MutableMap<K, V>: Map<K, V> {
    fn set(&mut self, key: &K, value: &V) -> Result<()>;
    fn delete(&mut self, key: &K) -> Result<()>;
}

pub trait MutableOrderedMap<K, V>: MutableMap<K, V> + OrderedMap<K, V> {
}

pub trait PersistentMap<K, V> {
    fn with(&self, key: &K, value: &V) -> Result<Box<dyn PersistentMap<K, V>>>;
    fn without(&self, key: &K) -> Result<Box<dyn PersistentMap<K, V>>>;
}

pub trait Batch<'a, K, V>: MutableOrderedMap<K, V> {
    fn new_batch(&'a mut self) -> &mut dyn Batch<'a, K, V>;
    fn write(&mut self) -> Result<()>;
}

pub trait CommitKVStore<'a, K, V, Commit>: Batch<'a, K, V> {
    fn commit(&mut self) -> Result<Commit>;
}


