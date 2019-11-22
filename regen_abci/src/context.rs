use abci::Header;
use blake2::{Blake2b, Digest};
use crate::config::Config;
use bech32::{ToBase32, FromBase32};
use std::error::Error;
use bech32::Error::InvalidChecksum;
use crate::context::ABCIPhase::Query;
use regen_client_sdk::auth::{Condition, Address};
use std::any::Any;
use im::*;
use im::hashmap::*;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::result::Res;
use regen_context::{ContextKey, Context};

pub struct StoreKey(Box<[u8]>);

#[derive(Clone)]
pub enum ABCIPhase {
    Query,
    Info,
    InitChain,
    BeginBlock,
    Check,
    Deliver,
    EndBlock,
    Commit,
}

impl Default for ABCIPhase {
    fn default() -> Self {
        Query
    }
}


pub static mut VERSION: ContextKey<String> = ContextKey("version", PhantomData);

pub const ABCI_PHASE: ContextKey<ABCIPhase> = ContextKey("abci_phase", PhantomData);

pub const CONFIG: ContextKey<Config> = ContextKey("config", PhantomData);

pub const BLOCK_HEADER: ContextKey<Header> = ContextKey("config", PhantomData);

//pub trait Context {
//    fn readonly_kv_store(&self, key: StoreKey) -> Result<Box<dyn ReadonlyKVStore>, Box<dyn Error>>;
//    fn kv_store(&self, key: StoreKey) -> Result<Box<dyn KVStore>, Box<dyn Error>>;
//    fn block_header(&self) -> &Header;
//    fn address_string(&self, addr: &Address) -> Result<String, Box<dyn Error>>;
//    fn parse_address(&self, str: &String) -> Result<Address, Box<dyn Error>>;
//    fn condition_address(&self, condition: &Condition) -> Address;
//    fn get_conditions(&self) -> &HashSet<Condition>;
//    fn with_conditions(&self, conditions: &[Condition]) -> Box<dyn Context>;
//    fn abci_phase(&self) -> &ABCIPhase;
//    fn with_value(&self, key: &[u8], value: &[u8]) -> Box<dyn Context>;
//    fn get_value(&self, key: &[u8]) -> Option<&[u8]>;
//    fn with_version(&self, version: u64) -> Box<dyn Context>;
//    fn get_version(&self) -> u64;
//}
//
//impl Context for StdContext {
//    fn readonly_kv_store(&self, key: StoreKey) -> Result<Box<dyn ReadonlyKVStore>, Box<dyn Error>> {
//        unimplemented!()
//    }
//
//    fn kv_store(&self, key: StoreKey) -> Result<Box<dyn KVStore>, Box<dyn Error>> {
//        unimplemented!()
//    }
//
//    fn block_header(&self) -> &Header {
//        &self.header
//    }
//
pub fn address_string(ctx: &Context, addr: &Address) -> Res<String> {
    let cfg = ctx.get(&CONFIG)?;
    let x = bech32::encode(&cfg.address_prefix, addr.0.to_base32())?;
    Ok(x)
}

pub fn parse_address(ctx: &Context, str: &String) -> Res<Address> {
    let cfg = ctx.get(&CONFIG)?;
    let (hrp, data) = bech32::decode(str)?;
    if !(hrp.eq(&cfg.address_prefix)) {
        Err(Box::from(InvalidChecksum))
    } else {
        let res = Vec::<u8>::from_base32(&data)?;
        Ok(Address(Box::from(res)))
    }
}

pub fn condition_address(ctx: &Context, cond: &Condition) -> Address {
    let mut hasher = Blake2b::new();
    hasher.input(cond.to_string());
    Address(Box::from(hasher.result().as_slice()))
}
//
//    fn get_conditions(&self) -> &HashSet<Condition> {
//        &self.conditions
//    }
//
//    fn with_conditions(&self, conditions: &[Condition]) -> Box<dyn Context> {
////        let new_conds = conditions.iter().fold(
////            self._conditions,
////            |conds, cond| conds.update(cond.clone()),
////        );
////        Box::from(ContextImpl {
////            _config: self._config.clone(),
////            _header: self._header.clone(),
////            _conditions: new_conds.clone(),
////        })
//        unimplemented!()
//    }
//
//    fn abci_phase(&self) -> &ABCIPhase {
//        &self.phase
//    }
//
//    fn with_value(&self, key: &[u8], value: &[u8]) -> Box<dyn Context> {
//        unimplemented!()
//    }
//
//    fn get_value(&self, key: &[u8]) -> Option<&[u8]> {
//        unimplemented!()
//    }
//
//    fn with_version(&self, version: u64) -> Box<dyn Context> {
//        unimplemented!()
//    }
//
//    fn get_version(&self) -> u64 {
//        unimplemented!()
//    }
//}

