use abci::Header;
use crate::auth::{Condition, Address};
use blake2::{Blake2b, Digest};
use crate::config::Config;
use bech32::{ToBase32, FromBase32};
use std::error::Error;
use crate::store::{ReadonlyKVStore, KVStore};
use bech32::Error::InvalidChecksum;
use im::HashSet;

pub struct StdContext {
    pub config: Config,
    pub header: Header,
    pub conditions: HashSet<Condition>,
}

pub struct StoreKey(Box<[u8]>);

pub trait Context {
    fn readonly_kv_store(&self, key: StoreKey) -> Result<Box<dyn ReadonlyKVStore>, Box<dyn Error>>;
    fn kv_store(&self, key: StoreKey) -> Result<Box<dyn KVStore>, Box<dyn Error>>;
    fn block_header(&self) -> &Header;
    fn address_string(&self, addr: &Address) -> Result<String, Box<dyn Error>>;
    fn parse_address(&self, str: &String) -> Result<Address, Box<dyn Error>>;
    fn condition_address(&self, condition: &Condition) -> Address;
    fn get_conditions(&self) -> &HashSet<Condition>;
    fn with_conditions(&self, conditions: &[Condition]) -> Box<dyn Context>;
}

impl Context for StdContext {
    fn readonly_kv_store(&self, key: StoreKey) -> Result<Box<dyn ReadonlyKVStore>, Box<dyn Error>> {
        unimplemented!()
    }

    fn kv_store(&self, key: StoreKey) -> Result<Box<dyn KVStore>, Box<dyn Error>> {
        unimplemented!()
    }

    fn block_header(&self) -> &Header {
        &self.header
    }

    fn address_string(&self, addr: &Address) -> Result<String, Box<dyn Error>> {
        match bech32::encode(&self.config.address_prefix, addr.0.to_base32()) {
            Ok(x) => Ok(x),
            Err(e) => Err(Box::from(e))
        }
    }

    fn parse_address(&self, str: &String) -> Result<Address, Box<dyn Error>> {
        let res = bech32::decode(str);
        match res {
            Err(e) => Err(Box::from(e)),
            Ok((hrp, data)) => {
                if !(hrp.eq(&self.config.address_prefix)) {
                    Err(Box::from(InvalidChecksum))
                } else {
                    let res = Vec::<u8>::from_base32(&data);
                    match res {
                        Err(e) => Err(Box::from(e)),
                        Ok(v) => Ok(Address(Box::from(v)))
                    }
                }
            }
        }
    }

    fn condition_address(&self, cond: &Condition) -> Address {
        let mut hasher = Blake2b::new();
        hasher.input(cond.to_string());
        Address(Box::from(hasher.result().as_slice()))
    }

    fn get_conditions(&self) -> &HashSet<Condition> {
        &self.conditions
    }

    fn with_conditions(&self, conditions: &[Condition]) -> Box<dyn Context> {
//        let new_conds = conditions.iter().fold(
//            self._conditions,
//            |conds, cond| conds.update(cond.clone()),
//        );
//        Box::from(ContextImpl {
//            _config: self._config.clone(),
//            _header: self._header.clone(),
//            _conditions: new_conds.clone(),
//        })
        unimplemented!()
    }
}

