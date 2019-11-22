use err_derive::Error;
use std::error::Error;
use regen_store::{Entry, Iterator, ReadonlyKVStore, KVStore};
use regen_context::Context;
use crate::TableError::{Other, Wrap};
use regen_context::ContextError::NotFound;

#[derive(Debug, Error)]
pub enum TableError {
    #[error(display="unexpected state")]
    UnexpectedState,
    #[error(display="not found")]
    NotFound,
    #[error(display="{:?}", _0)]
    Other(String),
    #[error(display="{:?}", _0)]
    Wrap(Box<dyn Error>)
}

pub trait StoreContext: Context {
    fn readonly_kv_store(&self, key: StoreKey) -> Result<&dyn ReadonlyKVStore<Vec<u8>, Vec<u8>>, TableError>;
    fn kv_store(&self, key: StoreKey) -> Result<&mut dyn KVStore<Vec<u8>, Vec<u8>>, TableError>;
}

pub struct StoreKey(Vec<u8>);

pub trait Index<K, V> {
    fn has(&self, ctx: &dyn StoreContext, key: &K) -> Result<bool, TableError>;
    fn get(&self, ctx: &dyn StoreContext, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
    fn prefix_scan(&self, ctx: &dyn StoreContext, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
    fn reverse_prefix_scan(&self, ctx: &dyn StoreContext, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
}

pub trait UniqueIndex<K, V>: Index<K, V> {
    fn get_one(&self, ctx: &dyn StoreContext, key: &K) -> Result<&dyn Entry<K, V>, TableError>;
}

pub trait Table<K, V>: UniqueIndex<K, V> {
    fn delete(&self, ctx: &dyn StoreContext, k: &K) -> Option<TableError>;
    fn save(&self, ctx: &dyn StoreContext, v: &V) -> Result<Option<&V>, TableError>;
}

pub trait TableInterceptor<K, V> {
    fn on_read(&self, ctx: &dyn StoreContext, value: &V) -> Result<Option<&V>, TableError>;
    fn before_save(&self, ctx: &dyn StoreContext, row_id: u64, value: &mut V) -> Result<(), TableError>;
    fn after_save(&self, ctx: &dyn StoreContext, row_id: u64, value: &V) -> Result<(), TableError>;
    fn before_delete(&self, ctx: &dyn StoreContext, row_id: u64, key: &K) -> Result<(), TableError>;
    fn after_delete(&self, ctx: &dyn StoreContext, row_id: u64, key: &K) -> Result<(), TableError>;
}

pub trait Sequence {
    fn next_val(&self, ctx: &dyn StoreContext) -> u64;
    fn cur_val(&self, ctx: &dyn StoreContext) -> u64;
}

struct SequenceImpl(StoreKey);

const SEQ_KEY: Vec<u8> = Vec::from([0; 0]);

impl Sequence for SequenceImpl {
    fn next_val(&self, ctx: &dyn StoreContext) -> Result<u64, TableError> {
        let cur = self.cur_val(ctx)?;
        let store = ctx.kv_store(self.0)?;
        let next = cur + 1;
        let mut buf = unsigned_varint::encode::u64_buffer();
        let res = unsigned_varint::encode::u64(next, &mut buf);
        match store.set(&SEQ_KEY, &Vec::from(res)) {
            Err(e) => Err(Wrap(Box::from(e))),
            _ => Ok(next)
        }
    }

    fn cur_val(&self, ctx: &dyn StoreContext) -> Result<u64, TableError> {
        let store = ctx.readonly_kv_store(self.0)?;
        match store.get(&SEQ_KEY) {
            Err(e) => Err(Wrap(Box::from(e))),
            Ok(None) => Ok(0),
            Ok(Some(r)) =>{
                match unsigned_varint::decode::u64(r) {
                    Err(e) => Err(Wrap(Box::from(e))),
                    x => x
                }
            }
        }
    }
}
