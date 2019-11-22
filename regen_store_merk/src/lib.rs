use regen_store::{ReadonlyKVStore, StoreError, Iterator, KVStore, Batch, CommitKVStore, Commit};
use merk::{Merk, Op};
use std::collections::BTreeMap;
use std::sync::mpsc::channel;
use std::marker::PhantomData;

struct MerkReadonlyKVStore(Merk);

impl ReadonlyKVStore for MerkReadonlyKVStore {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        self.0.get(key)
            .map_err(|e| StoreError::Wrap(Box::from(e)))
    }

    fn has(&self, key: &[u8]) -> Result<bool, StoreError> {
        let res = self.get(key)?;
        Ok(res.is_some())
    }

    fn iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<dyn Iterator>, StoreError> {
        Err(StoreError::Other(String::from("not implemented")))
    }

    fn reverse_iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<dyn Iterator>, StoreError> {
        Err(StoreError::Other(String::from("not implemented")))
    }
}

struct MerkKVStoreBatch<'a> {
    store: &'a mut dyn MerkKVStoreBatchWriter,
    changes: BTreeMap<Vec<u8>, Op>
}

trait MerkKVStoreBatchWriter: ReadonlyKVStore {
    fn write_changes(&mut self, changes: &BTreeMap<Vec<u8>, Op>);
}

impl <'a> ReadonlyKVStore for MerkKVStoreBatch<'a> {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        unimplemented!()
    }

    fn has(&self, key: &[u8]) -> Result<bool, StoreError> {
        unimplemented!()
    }

    fn iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<dyn Iterator>, StoreError> {
        unimplemented!()
    }

    fn reverse_iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<dyn Iterator>, StoreError> {
        unimplemented!()
    }
}

impl <'a> KVStore for MerkKVStoreBatch<'a> {
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), StoreError> {
        self.changes.insert(Vec::from(key), Op::Put(Vec::from(value)));
        Ok(())
    }

    fn delete(&mut self, key: &[u8]) -> Result<(), StoreError> {
        self.changes.insert(Vec::from(key), Op::Delete);
        Ok(())
    }
}

impl <'a> MerkKVStoreBatchWriter for MerkKVStoreBatch<'a> {
    fn write_changes(&mut self, changes: &BTreeMap<Vec<u8>, Op>) {
        for (k, v) in changes.iter() {
            self.changes.insert(k.clone(), *v.clone());
        }
        unimplemented!()
    }
}

impl <'a> Batch<'a> for MerkKVStoreBatch<'a> {
    fn new_batch(&'a mut self) -> &mut dyn Batch<'a> {
//        &mut MerkKVStoreBatch {
//            store: self,
//            changes: Default::default()
//        }
        unimplemented!()
    }

    fn write(&mut self) -> Result<(), StoreError> {
        self.store.write_changes(&self.changes);
        self.changes.clear();
        Ok(())
    }
}

struct MerkCommitKVStore {
    merk: Merk,
    changes: BTreeMap<Vec<u8>, Op>
}

impl ReadonlyKVStore for MerkCommitKVStore {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, StoreError> {
        unimplemented!()
    }

    fn has(&self, key: &[u8]) -> Result<bool, StoreError> {
        unimplemented!()
    }

    fn iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<Iterator>, StoreError> {
        unimplemented!()
    }

    fn reverse_iterator(&self, start: &[u8], end: &[u8]) -> Result<Box<Iterator>, StoreError> {
        unimplemented!()
    }
}

impl KVStore for MerkCommitKVStore {
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), StoreError> {
        unimplemented!()
    }

    fn delete(&mut self, key: &[u8]) -> Result<(), StoreError> {
        unimplemented!()
    }
}

impl <'a> Batch<'a> for MerkCommitKVStore {
    fn new_batch(&'a mut self) -> &mut dyn Batch<'a> {
        unimplemented!()
    }

    fn write(&mut self) -> Result<(), StoreError> {
        unimplemented!()
    }
}

impl <'a> CommitKVStore<'a> for MerkCommitKVStore {
    fn commit(&mut self) -> Result<Commit, StoreError> {
        let mut batch = Vec::with_capacity(self.changes.len());
        for (k, v) in self.changes.iter() {
            batch.push((k.clone(), *v.clone()))
        }
        self.merk.apply(batch.as_slice())
            .map_err(|e| StoreError::Wrap(Box::from(e)))?;
        Ok(Commit{hash: self.merk.root_hash().to_vec() })
    }
}
