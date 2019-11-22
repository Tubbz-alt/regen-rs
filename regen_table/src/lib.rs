use err_derive::Error;
use std::error::Error;
use regen_context::Context;
use regen_store::{Entry, Iterator};

#[derive(Debug, Error)]
pub enum TableError {
    #[error(display="{:?}", _0)]
    Other(String),
    #[error(display="{:?}", _0)]
    Wrap(Box<dyn Error>)
}

pub trait Index<K, V> {
    fn has(&self, ctx: &Context, key: &K) -> Result<bool, TableError>;
    fn get(&self, ctx: &Context, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
    fn prefix_scan(&self, ctx: &Context, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
    fn reverse_prefix_scan(&self, ctx: &Context, key: &K) -> Result<&dyn Iterator<K, V>, TableError>;
}

pub trait UniqueIndex<K, V>: Index<K, V> {
    fn get_one(&self, ctx: &Context, key: &K) -> Result<&dyn Entry<K, V>, TableError>;
}

pub trait Table<K, V>: UniqueIndex<K, V> {
    fn delete(&self, ctx: &Context, k: &K) -> Option<TableError>;
    fn save(&self, ctx: &Context, v: &mut V) -> Result<Option<&V>, TableError>;
}

pub trait TableInterceptor<K, V> {
    fn on_read(&self, ctx: &Context, value: &V) -> Result<Option<&V>, TableError>;
    fn before_save(&self, ctx: &Context, row_id: u64, value: &mut V) -> Result<(), TableError>;
    fn after_save(&self, ctx: &Context, row_id: u64, value: &V) -> Result<(), TableError>;
    fn before_delete(&self, ctx: &Context, row_id: u64, key: &K) -> Result<(), TableError>;
    fn after_delete(&self, ctx: & Context, row_id: u64, key: &K) -> Result<(), TableError>;
}
