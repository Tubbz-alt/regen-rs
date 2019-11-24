use crate::store::StoreKey;
use regen_context::Context;
use regen_table::{Sequence, Table};

struct StoreManager {
    name_table: Box<dyn Table<u64, String>>
}

impl StoreManager {
    fn reserve_store_key(&self, ctx: &dyn Context, name: &str) -> StoreKey {
    }
}
