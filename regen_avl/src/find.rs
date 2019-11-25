use std::cmp::Ordering;
use std::sync::Arc;
use crate::api::{Node, TreeContext};
use regen_store::{Result};

pub fn find_node<K, V>(node: &Arc<Node<K, V>>, ctx: &TreeContext<K, V>, key: &K) -> Result<Option<Arc<Node<K, V>>>> {
    let data = &node.data;
    Ok(match (ctx.comparator)(key, &data.key) {
        Ordering::Less => match node.left.get_node(ctx)? {
            None => None,
            Some(l) => find_node(&l, ctx, key)?
        },
        Ordering::Greater => match node.right.get_node(ctx)? {
            None => None,
            Some(r) => find_node(&r, ctx, key)?
        },
        Ordering::Equal => Some(node.clone())
    })
}
