use std::sync::Arc;
use regen_store::{Result, StoreError};
use crate::api::NodeRef::{HashRef, MemRef, NoRef};
use crate::api::{NodeRef, TreeContext, Node, NodeData, NodeStore};
use std::cmp::{Ordering, max};

impl<K, V> TreeContext<K, V> {
    pub(crate) fn get_store(&self) -> Result<&NodeStore<K, V>> {
        match &self.store {
            None => Err(Box::from(StoreError::Other(String::from("have a HashRef but no backing store!")))),
            Some(store) => {
                Ok(store.as_ref())
            }
        }
    }
    pub(crate) fn get_store_mut(&mut self) -> Result<&mut NodeStore<K, V>> {
        match &mut self.store {
            None => Err(Box::from(StoreError::Other(String::from("have a HashRef but no backing store!")))),
            Some(store) => {
                Ok(store.as_mut())
            }
        }
    }
}

impl<K, V> NodeRef<K, V> {
    pub(crate) fn get_node(&self, ctx: &TreeContext<K, V>) -> Result<Option<Arc<Node<K, V>>>> {
        match self {
            HashRef(h) => {
                let store = ctx.get_store()?;
                match store.get(&h)? {
                    None => Ok(None),
                    Some(node) => Ok(Some(Arc::new(node))),
                }
            }
            MemRef(node) => Ok(Some(node.clone())),
            NoRef => Ok(None),
        }
    }
}

fn node_height<K, V>(node: &Option<Arc<Node<K, V>>>) -> i32 {
    match node {
        None => 0,
        Some(node) => node.data.height as i32,
    }
}

fn node_rank<K, V>(node: &Option<Arc<Node<K, V>>>) -> u64 {
    match node {
        None => 0,
        Some(node) => node.data.rank
    }
}

fn make_node_ref<K, V>(node: Option<Arc<Node<K, V>>>) -> NodeRef<K, V> {
    match node {
        None => NoRef,
        Some(node) => MemRef(node)
    }
}

fn make_node<K: Clone, V: Clone>(key: &K, value: &V, left: Option<Arc<Node<K, V>>>, right: Option<Arc<Node<K, V>>>) -> Arc<Node<K, V>> {
    Arc::new(Node {
        data: Arc::new(NodeData {
            key: key.clone(),
            value: value.clone(),
            height: (max(node_height(&left), node_height(&right))) as u32,
            rank: node_rank(&left) + node_rank(&left) + 1,
        }),
        left: make_node_ref(left),
        right: make_node_ref(right),
        hash: None,
    })
}

impl<K: Clone, V: Clone> Node<K, V> {
    fn balance_factor(&self, ctx: &TreeContext<K, V>) -> Result<i32> {
        Ok(node_height(&self.left.get_node(ctx)?) - node_height(&self.right.get_node(ctx)?))
    }

    fn balance(&self, ctx: &TreeContext<K, V>) -> Result<Arc<Self>> {
        let left = self.left.get_node(ctx)?;
        let right = self.right.get_node(ctx)?;
        let left_height = node_height(&left);
        let right_height = node_height(&right);
        let diff = left_height - right_height;
        let data = &self.data;
        let key = &data.key;
        let value = &data.value;
        // Left Big
        if diff == 2 {
            Ok(match left {
                None => panic!("illegal"),
                Some(l) => {
                    let bal_factor = l.balance_factor(ctx)?;
                    // Left Heavy
                    let l_data = &l.data;
                    let ll = l.left.get_node(ctx)?;
                    let lr = l.right.get_node(ctx)?;
                    if bal_factor >= 0 {
                        make_node(
                            &l_data.key,
                            &l_data.value,
                            ll,
                            Some(make_node(
                                &key,
                                &value,
                                lr,
                                right,
                            )),
                        )
                    }
                    // Right Heavy
                    else {
                        match l.right.get_node(ctx)? {
                            None => panic!("illegal"),
                            Some(lr) => {
                                let lr_data = &lr.data;
                                let ll = l.left.get_node(ctx)?;
                                let lrr = lr.right.get_node(ctx)?;
                                let lrl = lr.left.get_node(ctx)?;
                                make_node(
                                    &lr_data.key,
                                    &lr_data.value,
                                    Some(make_node(
                                        &l_data.key,
                                        &l_data.value,
                                        ll,
                                        lrl,
                                    )),
                                    Some(make_node(
                                        key,
                                        value,
                                        lrr,
                                        right,
                                    )))
                            }
                        }
                    }
                }
            })
        } else if diff == -2 {
            Ok(match right {
                None => panic!("illegal"),
                Some(r) => {
                    let bal_factor = r.balance_factor(ctx)?;
                    let r_data = &r.data;
                    if bal_factor > 0 {
                        match r.left.get_node(ctx)? {
                            None => panic!("illegal"),
                            Some(rl) => {
                                let rl_data = &rl.data;
                                let rll = rl.left.get_node(ctx)?;
                                let rlr = rl.right.get_node(ctx)?;
                                let rr = r.right.get_node(ctx)?;
                                make_node(
                                    &rl_data.key,
                                    &rl_data.value,
                                    Some(make_node(
                                        key,
                                        value,
                                        left,
                                        rll,
                                    )),
                                    Some(make_node(
                                        &r_data.key,
                                        &r_data.value,
                                        rlr,
                                        rr,
                                    )))
                            }
                        }
                    } else {
                        let rl = r.left.get_node(ctx)?;
                        let rr = r.right.get_node(ctx)?;
                        make_node(
                            &r_data.key,
                            &r_data.value,
                            Some(make_node(
                                key,
                                value,
                                left,
                                rl,
                            )),
                            rr,
                        )
                    }
                }
            })
        } else {
            Ok(make_node(key, value, left, right))
        }
    }

    fn update_value(&self, v: &V) -> Self {
        let data = self.data.as_ref();
        Node {
            data: Arc::new(NodeData {
                key: data.key.clone(),
                value: data.value.clone(),
                height: data.height,
                rank: data.rank,
            }),
            left: self.left.clone(),
            right: self.right.clone(),
            hash: None,
        }
    }
}

pub(crate) fn insert<K: Clone, V: Clone>(node: Option<Arc<Node<K, V>>>, ctx: &TreeContext<K, V>, key: &K, value: &V) -> Result<Arc<Node<K, V>>> {
    match node {
        None => {
            Ok(make_node(key, value, None, None))
        }
        Some(node) => {
            let data = &node.data;
            let nkey = &data.key;
            let nvalue = &data.value;
            match (ctx.comparator)(key, nkey) {
                Ordering::Less => {
                    let left = node.left.get_node(ctx)?;
                    let right = node.right.get_node(ctx)?;
                    make_node(nkey, nvalue, Some(insert(left, ctx, key, value)?), right).balance(ctx)
                }
                Ordering::Greater => {
                    let left = node.left.get_node(ctx)?;
                    let right = node.right.get_node(ctx)?;
                    make_node(nkey, nvalue, left, Some(insert(right, ctx, key, value)?)).balance(ctx)
                }
                Ordering::Equal => Ok(Arc::new(node.update_value(&value)))
            }
        }
    }
}


