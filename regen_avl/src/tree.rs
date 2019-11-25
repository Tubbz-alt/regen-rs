use crate::api::{TreeContext, NodeRef};
use regen_store::{Map, Result, PersistentMap};
use std::error::Error;
use crate::find::find_node;
use std::sync::Arc;
use crate::balance::insert;
use crate::api::NodeRef::{MemRef, NoRef};

pub struct Tree<K, V> {
    ctx: Arc<TreeContext<K, V>>,
    root: NodeRef<K, V>,
}

impl<K, V> Tree<K, V> {
    pub fn new(ctx: Arc<TreeContext<K, V>>) -> Self {
        Tree { ctx, root: NoRef }
    }
}

impl<K, V: Clone> Map<K, V> for Tree<K, V> {
    fn get(&self, key: &K) -> Result<Option<V>> {
        Ok(match self.root.get_node(&self.ctx)? {
            None => None,
            Some(node) =>
                find_node(&node, &self.ctx, key)?
                    .map(|n| n.data.value.clone())
        })
    }

    fn has(&self, key: &K) -> Result<bool> {
        Ok(match self.root.get_node(&self.ctx)? {
            None => false,
            Some(node) => find_node(&node, &self.ctx, key)?.is_some(),
        })
    }
}

impl<K: 'static + Clone, V: 'static + Clone> PersistentMap<K, V> for Tree<K, V> {
    fn with(&self, key: &K, value: &V) -> Result<Box<dyn PersistentMap<K, V>>> {
        Ok(Box::from(Tree {
            ctx: self.ctx.clone(),
            root: MemRef(insert(self.root.get_node(&self.ctx)?, &self.ctx, key, value)?),
        }))
    }

    fn without(&self, key: &K) -> Result<Box<dyn PersistentMap<K, V>>> {
        unimplemented!()
    }
}
