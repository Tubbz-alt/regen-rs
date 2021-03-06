use crate::handler::{Handler, RawHandler};
use abci::{RequestQuery, ResponseCheckTx, ResponseDeliverTx, ResponseQuery, RequestBeginBlock, ResponseBeginBlock, RequestInfo, ResponseInfo, RequestInitChain, ResponseInitChain, RequestEndBlock, ResponseEndBlock, RequestCommit, ResponseCommit};
use crate::result::Res;
use std::collections::HashMap;
use regen_context::SimpleContext;
use crate::store::StoreKey;

struct VersionInfo {
    current_version: String,
    upgrade_plan: Option<UpgradePlan>
}

struct UpgradePlan {
    height: u64,
    name: String,
    info: Box<[u8]>
}

struct VersionManager {
    versions: HashMap<String, Box<dyn VersionHandler>>,
    current_version: Option<Box<dyn Handler>>,
    store_key: StoreKey,
}

impl VersionManager {
    fn cur(&self, ctx: &SimpleContext) -> Box<dyn Handler> {
        unimplemented!()
    }
}

impl <T, Q, RC: Default, RD: Default, RQ: Default> Handler<T, Q, RC, RD, RQ> for VersionManager {
    fn info(&self, ctx: &SimpleContext, req: &RequestInfo) -> ResponseInfo {
        self.cur(ctx).info(ctx, req)
    }

    fn init_chain(&self, ctx: &SimpleContext, req: &RequestInitChain) -> ResponseInitChain {
        self.cur(ctx).init_chain(ctx, req)
    }

    fn begin_block(&self, ctx: &SimpleContext, req: &RequestBeginBlock) -> ResponseBeginBlock {
        self.cur(ctx).begin_block(ctx, req)
    }

    fn check(&self, ctx: &SimpleContext, tx: &T) -> RC {
        self.cur(ctx).check(ctx, tx)
    }

    fn deliver(&self, ctx: &SimpleContext, tx: &T) -> RD {
        self.cur(ctx).deliver(ctx, tx)
    }

    fn end_block(&self, ctx: &SimpleContext, req: &RequestEndBlock) -> ResponseEndBlock {
        self.cur(ctx).end_block(ctx, req)
    }

    fn commit(&self, ctx: &SimpleContext, req: &RequestCommit) -> ResponseCommit {
        self.cur(ctx).commit(ctx, req)
    }

    fn query(&self, ctx: &SimpleContext, query: &Q) -> RQ {
        self.cur(ctx).query(ctx, query)
    }
}

pub trait VersionHandler: Handler<Box<[u8]>, RequestQuery, ResponseCheckTx, ResponseDeliverTx, ResponseQuery> {
    fn migrate(&self, ctx: &SimpleContext, from_version: &str) -> Res<()>;
}
