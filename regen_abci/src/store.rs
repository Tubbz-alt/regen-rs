use regen_store::{MutableOrderedMap, CommitKVStore, Batch, ReadonlyKVStore};
use crate::handler::{Decorator, Handler};
use regen_context::{SimpleContext, ContextKey};
use abci;
use crate::result::Res;
use std::marker::PhantomData;
use std::rc::Weak;

struct StoreMiddleware<'a> {
    app_store: Box<dyn CommitKVStore<'a, Vec<u8>, Vec<u8>>>,
    block_store: Box<dyn Batch<'a, Vec<u8>, Vec<u8>>>
}

struct ReadonlyKVStoreAccessor(Weak<dyn ReadonlyKVStore<Vec<u8>, Vec<u8>>>);

pub struct StoreKey(Vec<u8>);

impl ReadonlyKVStoreAccessor {
    fn readonly_kv_store(&self, key: StoreKey) -> Res<&dyn ReadonlyKVStore> {
        unimplemented!()
    }
}

const READONLY_KV_STORE_ACCESSOR: ContextKey<ReadonlyKVStoreAccessor> = ContextKey("readonly_kv_store_accessor", PhantomData);

impl <'a, T, Q, CheckRes, DeliverRes, QueryRes> Decorator<T, Q, CheckRes, DeliverRes, QueryRes> for StoreMiddleware<'a> {
    fn on_info(&self, ctx: &SimpleContext, req: &abci::RequestInfo, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseInfo {
        next.info(&ctx, req)
    }

    fn on_init_chain(&self, ctx: &SimpleContext, req: &abci::RequestInitChain, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseInitChain {
        next.init_chain(ctx, req)
    }

    fn on_begin_block(&self, ctx: &SimpleContext, req: &abci::RequestBeginBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseBeginBlock {
        next.begin_block(ctx, req)
    }

    fn on_check(&self, ctx: &SimpleContext, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> CheckRes {
        next.check(ctx, tx)
    }
    fn on_deliver(&self, ctx: &SimpleContext, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> DeliverRes {
        next.deliver(ctx, tx)
    }

    fn on_end_block(&self, ctx: &SimpleContext, req: &abci::RequestEndBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseEndBlock {
        next.end_block(ctx, req)
    }

    fn on_commit(&mut self, ctx: &SimpleContext, req: &abci::RequestCommit, next: &mut dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseCommit {
        next.commit(ctx, req)
    }

    fn on_query(&self, ctx: &SimpleContext, req: &Q, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> QueryRes {
        next.query(ctx, req)
    }
}