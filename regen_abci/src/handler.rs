use crate::result::{CheckResult, DeliverResult};
use crate::tx::Tx;
use crate::result::Res;
use abci;
use std::ops::{Deref, Shr};
use regen_context::SimpleContext;

//pub type EndBlocker = fn(ctx: &dyn Context, req: &RequestEndBlock) -> ResponseEndBlock;
//
//pub type EndBlocker = fn(ctx: &dyn Context, req: &RequestEndBlock) -> ResponseEndBlock;

pub trait Handler<T = Box<dyn Tx>, Q = abci::RequestQuery, CheckRes: Default = CheckResult, DeliverRes: Default = DeliverResult, QueryRes: Default = abci::ResponseQuery> {
    fn info(&self, ctx: &SimpleContext, req: &abci::RequestInfo) -> abci::ResponseInfo {
        abci::ResponseInfo::new()
    }

    fn init_chain(&self, ctx: &SimpleContext, req: &abci::RequestInitChain) -> abci::ResponseInitChain {
        abci::ResponseInitChain::new()
    }

    fn begin_block(&self, ctx: &SimpleContext, req: &abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        abci::ResponseBeginBlock::new()
    }

    fn check(&self, ctx: &SimpleContext, tx: &T) -> CheckRes {
        CheckRes::default()
    }

    fn deliver(&self, ctx: &SimpleContext, tx: &T) -> DeliverRes {
        DeliverRes::default()
    }

    fn end_block(&self, ctx: &SimpleContext, req: &abci::RequestEndBlock) -> abci::ResponseEndBlock {
        abci::ResponseEndBlock::new()
    }

    fn commit(&mut self, ctx: &SimpleContext, req: &abci::RequestCommit) -> abci::ResponseCommit {
        abci::ResponseCommit::new()
    }

    fn query(&self, ctx: &SimpleContext, query: &Q) -> QueryRes {
        QueryRes::default()
    }
}

pub type RawHandler = Box<dyn Handler<Box<[u8]>, abci::RequestQuery, abci::ResponseCheckTx, abci::ResponseDeliverTx, abci::ResponseQuery>>;
pub type TxHandler = Box<dyn Handler<Box<dyn Tx>>>;

pub trait Decorator<
    T=Box<dyn Tx>,
    Q = abci::RequestQuery,
    CheckRes: Default = CheckResult,
    DeliverRes: Default = DeliverResult,
    QueryRes: Default = abci::ResponseQuery,
> {
    fn on_info(&self, ctx: &SimpleContext, req: &abci::RequestInfo, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseInfo {
        next.info(ctx, req)
    }

    fn on_init_chain(&self, ctx: &SimpleContext, req: &abci::RequestInitChain, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseInitChain {
        next.init_chain(ctx, req)
    }

    fn on_begin_block(&self, ctx: &SimpleContext, req: &abci::RequestBeginBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseBeginBlock {
        next.begin_block(ctx, req)
    }

    fn on_check(&self, ctx: &SimpleContext, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> CheckRes {
        next.check(ctx, tx)
    }
    fn on_deliver(&self, ctx: &SimpleContext, tx: &T, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> DeliverRes {
        next.deliver(ctx, tx)
    }

    fn on_end_block(&self, ctx: &SimpleContext, req: &abci::RequestEndBlock, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseEndBlock {
        next.end_block(ctx, req)
    }

    fn on_commit(&mut self, ctx: &SimpleContext, req: &abci::RequestCommit, next: &mut dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> abci::ResponseCommit {
        next.commit(ctx, req)
    }

    fn on_query(&self, ctx: &SimpleContext, req: &Q, next: &dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>) -> QueryRes {
        next.query(ctx, req)
    }
}

struct Chain<T, Q, CheckRes, DeliverRes, QueryRes>(Box<dyn Decorator<T, Q, CheckRes, DeliverRes, QueryRes>>, Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>);

impl <T, Q, CheckRes: Default, DeliverRes: Default, QueryRes: Default> Handler<T, Q, CheckRes, DeliverRes, QueryRes> for Chain<T, Q, CheckRes, DeliverRes, QueryRes> {
    fn info(&self, ctx: &SimpleContext, req: &abci::RequestInfo) -> abci::ResponseInfo {
        self.0.on_info(ctx, req, self.1.as_ref())
    }

    fn init_chain(&self, ctx: &SimpleContext, req: &abci::RequestInitChain) -> abci::ResponseInitChain {
        self.0.on_init_chain(ctx, req, self.1.as_ref())
    }

    fn begin_block(&self, ctx: &SimpleContext, req: &abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        self.0.on_begin_block(ctx, req, self.1.as_ref())
    }

    fn check(&self, ctx: &SimpleContext, tx: &T) -> CheckRes {
        self.0.on_check(ctx, tx, self.1.as_ref())
    }

    fn deliver(&self, ctx: &SimpleContext, tx: &T) -> DeliverRes {
        self.0.on_deliver(ctx, tx, self.1.as_ref())
    }

    fn end_block(&self, ctx: &SimpleContext, req: &abci::RequestEndBlock) -> abci::ResponseEndBlock {
        self.0.on_end_block(ctx, req, self.1.as_ref())
    }

    fn commit(&mut self, ctx: &SimpleContext, req: &abci::RequestCommit) -> abci::ResponseCommit {
        self.0.on_commit(ctx, req, self.1.as_ref())
    }

    fn query(&self, ctx: &SimpleContext, query: &Q) -> QueryRes {
        self.0.on_query(ctx, query, self.1.as_ref())
    }
}

impl <T, Q, CheckRes: Default, DeliverRes: Default, QueryRes: Default> Shr<Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>> for Box<dyn Decorator<T, Q, CheckRes, DeliverRes, QueryRes>> {
    type Output = Box<dyn Handler<T, Q, CheckRes, DeliverRes, QueryRes>>;

    fn shr(self, rhs: Self::Output) -> Self::Output {
        Box::from(Chain(self, rhs))
    }
}
