use crate::context::Context;
use crate::result::{CheckResult, DeliverResult};
use crate::tx::Tx;
use crate::result::Res;
use abci::{RequestBeginBlock, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseInitChain, RequestInitChain, RequestQuery, ResponseQuery};

pub trait Checker {
    fn check(&self, ctx: &dyn Context, tx: &dyn Tx) -> Res<CheckResult> {
        Ok(CheckResult{})
    }
}

pub trait Deliverer {
    fn deliver(&self, ctx: &dyn Context, tx: &dyn Tx) -> Res<DeliverResult> {
        Ok(DeliverResult{})
    }
}

pub trait Querier {
    fn query(&self, ctx: &dyn Context, tx: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }
}

pub type InitChainer = fn(ctx: &dyn Context, req: &RequestInitChain) -> ResponseInitChain;

pub type BeginBlocker = fn(ctx: &dyn Context, req: &RequestBeginBlock) -> ResponseBeginBlock;

pub type EndBlocker = fn(ctx: &dyn Context, req: &RequestEndBlock) -> ResponseEndBlock;

pub trait Handler: Checker + Deliverer + Querier {
    fn init_chainer(&self) -> Option<InitChainer> {
        None
    }

    fn begin_blocker(&self) -> Option<BeginBlocker> {
        None
    }

    fn end_blocker(&self) -> Option<EndBlocker> {
        None
    }
}

pub trait Decorator {
    fn decorate_check(&self, ctx: &dyn Context, tx: &dyn Tx, next: &dyn Checker) -> Res<CheckResult> {
        next.check(ctx, tx)
    }
    fn decorate_deliver(&self, ctx: &dyn Context, tx: &dyn Tx, next: &dyn Deliverer) -> Res<DeliverResult> {
        next.deliver(ctx, tx)
    }

    fn decorate_query(&self, ctx: &dyn Context, req: &RequestQuery, next: &dyn Querier) -> ResponseQuery {
        next.query(ctx, req)
    }
}

