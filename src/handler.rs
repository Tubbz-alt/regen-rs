use crate::context::Context;
use crate::result::{CheckResult, DeliverResult};
use crate::tx::Tx;
use crate::result::Res;
use abci::{RequestBeginBlock, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseInitChain, RequestInitChain, RequestQuery, ResponseQuery};

pub trait Checker {
    fn check(&self, ctx: &dyn Context, tx: &dyn Tx) -> Res<CheckResult>;
}

pub trait Deliverer {
    fn deliver(&self, ctx: &dyn Context, tx: &dyn Tx) -> Res<DeliverResult>;
}

pub trait Handler: Checker + Deliverer {
}

pub trait AppHandler: Handler {
    fn init_chain(&self, ctx: &dyn Context, req: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn begin_block(&self, ctx: &dyn Context, req: &RequestBeginBlock) -> Option<ResponseBeginBlock> {
        None
    }

    fn end_block(&self, ctx: &dyn Context, req: &RequestEndBlock) -> Option<ResponseEndBlock> {
        None
    }

    fn query(&self, ctx: &dyn Context, req: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }
}

pub trait Decorator {
    fn check(&self, ctx: &dyn Context, tx: &dyn Tx, next: &dyn Checker) -> Res<CheckResult>;
    fn deliver(&self, ctx: &dyn Context, tx: &dyn Tx, next: &dyn Deliverer) -> Res<DeliverResult>;
}

pub trait InitChain {
}
