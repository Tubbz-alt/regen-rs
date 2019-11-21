use crate::handler::AppHandler;
use abci::{RequestCheckTx, ResponseCheckTx, RequestBeginBlock, RequestDeliverTx, ResponseDeliverTx, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseCommit, RequestCommit, RequestInfo, ResponseInfo, RequestInitChain, ResponseInitChain, RequestQuery, ResponseQuery};
use crate::tx::Tx;
use std::error::Error;
use crate::context::{StdContext, ABCIPhase};
use crate::context::ABCIPhase::{BeginBlock, Check, InitChain, Query};
use crate::config::Config;

struct ABCIBaseApp {
//    logger log.Logger

    // Database state (committed, check, deliver....)
//    store: *CommitStore

    // chainID is loaded from db in initialization
    // saved once in parseGenesis
    chain_id: String,

    // cached validator changes from DeliverTx
    //    pending weave.ValidatorUpdates

    // baseContext contains context info that is valid for
    // lifetime of this app (eg. chainID)
    base_context: StdContext,
    block_context: StdContext,
    //    decoder: TxDecoder,
    handler: Box<dyn AppHandler>,
    //    ticker: Ticker,
}

impl ABCIBaseApp {
    pub fn new(chain_id: String, handler: Box<dyn AppHandler>, config: Config) -> Self {
        let ctx = StdContext{
            config,
            header: Default::default(),
            conditions: Default::default(),
            phase: ABCIPhase::Query,
        };
        ABCIBaseApp {
            chain_id,
            base_context: ctx.clone(),
            block_context: ctx,
            handler,
        }
    }
}

impl abci::Application for ABCIBaseApp {
    fn info(&mut self, _req: &RequestInfo) -> ResponseInfo {
        ResponseInfo {
            data: self.name.to_string(),
            version: "".to_string(),
            app_version: 0,
            last_block_height: 0,
            last_block_app_hash: vec![],
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }

    fn init_chain(&mut self, req: &RequestInitChain) -> ResponseInitChain {
        let ctx = StdContext {
            phase: InitChain,
            ..self.base_context
        };
        self.handler.init_chain(ctx, req)
    }

    fn begin_block(&mut self, req: &RequestBeginBlock) -> ResponseBeginBlock {
        let ctx = StdContext {
            phase: BeginBlock,
            config: self.base_context.config.clone(),
            header: req.get_header().clone(),
            conditions: Default::default(),
        };
        self.block_context = ctx;
        ResponseBeginBlock::from(self.handler.begin_block(&ctx, req))
    }

    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        let mut res = ResponseCheckTx::new();
        let ctx = StdContext {
            phase: Check,
            ..self.block_context
        };
        match self.load_tx(req.get_tx()) {
            Err(e) => {
                res.code = 1;
                res
            }
            Ok(tx) => {
                let hres =
                    self.handler.check(&self.block_context, tx.as_ref());
                res.code = 0;
                res
            }
        }
    }

    fn deliver_tx(&mut self, req: &RequestDeliverTx) -> ResponseDeliverTx {
        let mut res = ResponseDeliverTx::new();
        match self.load_tx(req.get_tx()) {
            Err(e) => {
                res.code = 1;
                res
            }
            Ok(tx) => {
                let hres =
                    self.handler.deliver(&self.block_context, tx.as_ref());
                res.code = 0;
                res
            }
        }
    }

    fn end_block(&mut self, req: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::from(self.handler.end_block(&self.block_context, req))
    }

    fn commit(&mut self, _req: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }

    fn query(&mut self, req: &RequestQuery) -> ResponseQuery {
        let ctx = StdContext {
            phase: Query,
            ..self.base_context
        };
        self.handler.query(ctx, req)
    }
}

impl ABCIBaseApp {
    fn load_tx(&self, tx_bytes: &[u8]) -> Result<Box<dyn Tx>, Box<dyn Error>> {
        unimplemented!()
    }
}