use crate::store::{Iterator, StoreError, Entry};
use crate::context::{Context};
use crate::result::Res;

pub trait Index<K, V> {
    fn has(&self, ctx: &Context, key: &K) -> Result<bool, StoreError>;
    fn get(&self, ctx: &Context, key: &K) -> Result<Box<dyn Iterator<K, V>>, StoreError>;
    fn prefix_scan(&self, ctx: &Context, key: &K) -> Result<Box<dyn Iterator<K, V>>, StoreError>;
    fn reverse_prefix_scan(&self, ctx: &Context, key: &K) -> Result<Box<dyn Iterator<K, V>>, StoreError>;
}

pub trait UniqueIndex<K, V>: Index<K, V> {
    fn get_one(&self, ctx: &Context, key: &K) -> Result<Box<dyn Entry<K, V>>, StoreError>;
}

pub trait Table<K, V>: UniqueIndex<K, V> {
    fn delete(&self, ctx: &Context, k: &K) -> Option<StoreError>;
    fn save(&self, ctx: &Context, v: &V) -> Result<Option<V>, StoreError>;
}

pub trait TableInterceptor<K, V> {
    fn on_read(&self, ctx: &Context, value: &V) -> Res<&V>;
    fn before_save(&self, ctx: &Context, row_id: u64, value: &mut V) -> Res<()>;
    fn after_save(&self, ctx: &Context, row_id: u64, value: &V) -> Res<()>;
    fn before_delete(&self, ctx: &Context, row_id: u64, key: &K) -> Res<()>;
    fn after_delete(&self, ctx: & Context, row_id: u64, key: &K) -> Res<()>;
}