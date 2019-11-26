use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Relaxed, AcqRel};
use im::hashmap;
use std::sync::Arc;
use std::hash::Hash;

struct LRUData<K, V> {
    entries: Arc<im::HashMap<K, V>>,
    priority: Arc<im::OrdMap<usize, K>>,
    tick: usize,
}

struct LRU<K, V> {
    data: AtomicPtr<LRUData<K, V>>,
    capacity: usize,
}

impl<K: Clone + Eq + Hash, V: Clone> LRU<K, V> {
    unsafe fn get(&self, key: &K) -> Option<&V> {
        let data = self.data.load(AcqRel);
        let entries = &(*data).entries;
        match entries.get(key) {
            Some(value) => {
                let next_tick = (*data).tick + 1;
                let mut new_data = &mut LRUData {
                    entries: entries.clone(),
                    priority: Arc::new((*data).priority.update(next_tick, key.clone())),
                    tick: next_tick,
                };
                self.data.store(new_data, AcqRel);
                Some(value)
            }
            r => None
        }
    }

    unsafe fn put(&self, key: K, value: V) {
        let data = self.data.load(AcqRel);
        let mut new_entries = (*data).entries.update(key.clone(), value);
        let next_tick = (*data).tick + 1;
        let mut new_priority = (*data).priority.update(next_tick, key);
        if new_priority.len() > self.capacity {
            let to_evict = new_priority.get_min();
            match to_evict {
                None => {},
                Some(min) => {
                    new_entries = new_entries.without(&min.1);
                    new_priority = new_priority.without(&min.0);
                }
            }
        }
        let mut new_data = &mut LRUData {
            entries: Arc::new(new_entries),
            priority: Arc::new(new_priority),
            tick: next_tick,
        };
        self.data.store(new_data, AcqRel);
    }
}