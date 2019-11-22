use crate::handler::{Handler, RawHandler};
use abci::{RequestCheckTx, ResponseCheckTx, RequestBeginBlock, RequestDeliverTx, ResponseDeliverTx, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseCommit, RequestCommit, RequestInfo, ResponseInfo, RequestInitChain, ResponseInitChain, RequestQuery, ResponseQuery};
use crate::tx::Tx;
use std::error::Error;
use crate::context::{ABCIPhase, ABCI_PHASE, BLOCK_HEADER};
use crate::context::ABCIPhase::{BeginBlock, Check, InitChain, Query, Deliver, EndBlock, Commit, Info};
use crate::config::Config;
use regen_context::SimpleContext;

struct ABCIBaseApp {
    base_context: SimpleContext,
    block_context: SimpleContext,
    handler: RawHandler,
}

impl ABCIBaseApp {
    pub fn new(handler: RawHandler) -> Self {
        let ctx = SimpleContext::new()
            .with(&ABCI_PHASE, ABCIPhase::Query);
        ABCIBaseApp {
            base_context: ctx.clone(),
            block_context: ctx,
            handler,
        }
    }
}

impl abci::Application for ABCIBaseApp {
    fn info(&mut self, req: &RequestInfo) -> ResponseInfo {
        self.handler.info(
            &self.base_context.with(&ABCI_PHASE, Info),
            req,
        )
    }

    fn init_chain(&mut self, req: &RequestInitChain) -> ResponseInitChain {
        self.handler.init_chain(
            &self.base_context.with(&ABCI_PHASE, InitChain),
            req,
        )
    }

    fn begin_block(&mut self, req: &RequestBeginBlock) -> ResponseBeginBlock {
        let ctx = &self.base_context
            .with(&ABCI_PHASE, BeginBlock)
            .with(&BLOCK_HEADER, req.get_header().clone());
        self.block_context = ctx.clone();
        self.handler.begin_block(&ctx, req)
    }

    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        self.handler.check(
            &self.block_context.with(&ABCI_PHASE, Check),
            &Box::from(req.get_tx()),
        )
    }

    fn deliver_tx(&mut self, req: &RequestDeliverTx) -> ResponseDeliverTx {
        self.handler.deliver(
            &self.block_context.with(&ABCI_PHASE, Deliver),
            &Box::from(req.get_tx()),
        )
    }

    fn end_block(&mut self, req: &RequestEndBlock) -> ResponseEndBlock {
        self.handler.end_block(
            &self.block_context.with(&ABCI_PHASE, EndBlock),
            req
        )
    }

    fn commit(&mut self, req: &RequestCommit) -> ResponseCommit {
        self.handler.commit(
            &self.block_context.with(&ABCI_PHASE, Commit),
            req
        )
    }

    fn query(&mut self, req: &RequestQuery) -> ResponseQuery {
        self.handler.query(
            &self.block_context.with(&ABCI_PHASE, Query),
            req
        )
    }
}

impl ABCIBaseApp {
    fn load_tx(&self, tx_bytes: &[u8]) -> Result<Box<dyn Tx>, Box<dyn Error>> {
        unimplemented!()
    }
}