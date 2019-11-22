use crate::context::Context;
use crate::result::{CheckResult, DeliverResult};
use crate::tx::Tx;
use crate::result::Res;
use abci::{RequestEndBlock, ResponseEndBlock, ResponseInitChain, RequestInitChain, RequestQuery, ResponseQuery, RequestCommit, ResponseCommit, RequestInfo, ResponseInfo, ResponseCheckTx, ResponseDeliverTx, ResponseBeginBlock, RequestBeginBlock};
use std::ops::{Deref, Shr};

//pub type EndBlocker = fn(ctx: &dyn Context, req: &RequestEndBlock) -> ResponseEndBlock;
//
//pub type EndBlocker = fn(ctx: &dyn Context, req: &RequestEndBlock) -> ResponseEndBlock;

pub trait Handler<T = Box<dyn Tx>, Q = RequestQuery, CheckRes: Default = CheckResult, DeliverRes: Default = DeliverResult, QueryRes: Default = ResponseQuery> {
    fn info(&self, ctx: &Context, req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }

    fn init_chain(&self, ctx: &Context, req: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn begin_block(&self, ctx: &Context, req: &RequestBeginBlock) -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn check(&self, ctx: &Context, tx: &T) -> CheckRes {
        CheckRes::default()
    }

    fn deliver(&self, ctx: &Context, tx: &T) -> DeliverRes {
        DeliverRes::default()
    }

    fn end_block(&self, ctx: &Context, req: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn commit(&self, ctx: &Context, req: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }

    fn query(&self, ctx: &Context, query: &Q) -> QueryRes {
        QueryRes::default()
    }
}

pub type RawHandler = Box<dyn Handler<Box<[u8]>, RequestQuery, ResponseCheckTx, ResponseDeliverTx, ResponseQuery>>;
pub type TxHandler = Box<dyn Handler<Box<dyn Tx>>>;

pub trait Decorator<
    T=Box<dyn Tx>,
    Q = RequestQuery,
    CheckRes: Default = CheckResult,
    DeliverRes: Default = DeliverResult,
    QueryRes: Default = ResponseQuery,
> {
    fn on_info(&self, ctx: &Context, req: &RequestInfo, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseInfo {
        next.info(ctx, req)
    }

    fn on_init_chain(&self, ctx: &Context, req: &RequestInitChain, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseInitChain {
        next.init_chain(ctx, req)
    }

    fn on_begin_block(&self, ctx: &Context, req: &RequestBeginBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseBeginBlock {
        next.begin_block(ctx, req)
    }

    fn on_check(&self, ctx: &Context, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> CheckRes {
        next.check(ctx, tx)
    }
    fn on_deliver(&self, ctx: &Context, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> DeliverRes {
        next.deliver(ctx, tx)
    }

    fn on_end_block(&self, ctx: &Context, req: &RequestEndBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseEndBlock {
        next.end_block(ctx, req)
    }

    fn on_commit(&self, ctx: &Context, req: &RequestCommit, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> ResponseCommit {
        next.commit(ctx, req)
    }

    fn on_query(&self, ctx: &Context, req: &Q, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> QueryRes {
        next.query(ctx, req)
    }
}

struct Chain<T, Q, CheckRes, DeliverRes, QueryRes>(Box<dyn Decorator<T, Q, CheckRes, DeliverRes, QueryRes>>, Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>);

impl <T, Q, CheckRes: Default, DeliverRes: Default, QueryRes: Default> Handler<T, Q, CheckRes, DeliverRes, QueryRes> for Chain<T, Q, CheckRes, DeliverRes, QueryRes> {
    fn info(&self, ctx: &Context, req: &RequestInfo) -> ResponseInfo {
        self.0.on_info(ctx, req, self.1.as_ref())
    }

    fn init_chain(&self, ctx: &Context, req: &RequestInitChain) -> ResponseInitChain {
        self.0.on_init_chain(ctx, req, self.1.as_ref())
    }

    fn begin_block(&self, ctx: &Context, req: &RequestBeginBlock) -> ResponseBeginBlock {
        self.0.on_begin_block(ctx, req, self.1.as_ref())
    }

    fn check(&self, ctx: &Context, tx: &T) -> CheckRes {
        self.0.on_check(ctx, tx, self.1.as_ref())
    }

    fn deliver(&self, ctx: &Context, tx: &T) -> DeliverRes {
        self.0.on_deliver(ctx, tx, self.1.as_ref())
    }

    fn end_block(&self, ctx: &Context, req: &RequestEndBlock) -> ResponseEndBlock {
        self.0.on_end_block(ctx, req, self.1.as_ref())
    }

    fn commit(&self, ctx: &Context, req: &RequestCommit) -> ResponseCommit {
        self.0.on_commit(ctx, req, self.1.as_ref())
    }

    fn query(&self, ctx: &Context, query: &Q) -> QueryRes {
        self.0.on_query(ctx, query, self.1.as_ref())
    }
}

impl <T, Q, CheckRes: Default, DeliverRes: Default, QueryRes: Default> Shr<Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>> for Box<dyn Decorator<T, Q, CheckRes, DeliverRes, QueryRes>> {
    type Output = Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>;

    fn shr(self, rhs: Self::Output) -> Self::Output {
        Box::from(Chain(self, rhs))
    }
}
