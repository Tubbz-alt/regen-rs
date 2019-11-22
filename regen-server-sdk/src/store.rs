use std::fmt::{Display, Formatter, Error, Debug};

pub enum StoreError {
    UnknownError,
    Unauthorized,
    NotFound,
}

impl std::error::Error for StoreError {

}

impl Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Ok(())
    }
}

impl Debug for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Ok(())
    }
}

pub trait Entry<K, V> {
    fn key(&self) -> K;
    fn value(&self) -> V;
}

pub trait Iterator<K, V> {
    fn next(&self) -> Result<Box<dyn Entry<K, V>>, StoreError>;
    fn release(&self);
}

type RawIterator = Box<dyn Iterator<Box<[u8]>, Box<[u8]>>>;

pub trait ReadonlyKVStore {
    fn get(&self, key: &[u8]) -> Result<Box<[u8]>, StoreError>;
    fn has(&self, key: &[u8]) -> Result<bool, StoreError>;
    fn iterator(&self, start: &[u8], end: &[u8]) -> Result<RawIterator, StoreError>;
    fn reverse_iterator(&self, start: &[u8], end: &[u8]) -> Result<RawIterator, StoreError>;
}

pub trait SetDeleter {
    fn set(&self, key: &[u8], value: &[u8]) -> Option<StoreError>;
    fn delete(&self, key: &[u8]) -> Option<StoreError>;
}

pub trait KVStore: ReadonlyKVStore + SetDeleter {
    fn new_batch(&self) -> Box<dyn Batch>;
}

pub trait Batch: SetDeleter {
    fn write(&self) -> Option<StoreError>;
}
