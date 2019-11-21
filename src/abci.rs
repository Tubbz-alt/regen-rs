use crate::handler::{Handler, RawHandler};
use abci::{RequestCheckTx, ResponseCheckTx, RequestBeginBlock, RequestDeliverTx, ResponseDeliverTx, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseCommit, RequestCommit, RequestInfo, ResponseInfo, RequestInitChain, ResponseInitChain, RequestQuery, ResponseQuery};
use crate::tx::Tx;
use std::error::Error;
use crate::context::{StdContext, ABCIPhase};
use crate::context::ABCIPhase::{BeginBlock, Check, InitChain, Query, Deliver, EndBlock, Commit};
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
    handler: RawHandler,
    //    ticker: Ticker,
}

impl ABCIBaseApp {
    pub fn new(chain_id: String, handler: RawHandler, config: Config) -> Self {
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
            handler: handler,
        }
    }
}

impl abci::Application for ABCIBaseApp {
    fn info(&mut self, _req: &RequestInfo) -> ResponseInfo {
        ResponseInfo {
            data: self.chain_id.to_string(),
            version: "".to_string(),
            app_version: 0,
            last_block_height: 0,
            last_block_app_hash: vec![],
            unknown_fields: Default::default(),
            cached_size: Default::default(),
        }
    }

    fn init_chain(&mut self, req: &RequestInitChain) -> ResponseInitChain {
        let mut ctx = self.base_context.clone();
        ctx.phase = InitChain;
        self.handler.init_chain(&ctx, req)
    }

    fn begin_block(&mut self, req: &RequestBeginBlock) -> ResponseBeginBlock {
        let mut ctx = self.base_context.clone();
        ctx.phase = BeginBlock;
        ctx.header = req.get_header().clone();
        self.block_context = ctx.clone();
        self.handler.begin_block(&ctx, req)
    }

    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        let mut ctx = self.base_context.clone();
        ctx.phase = Check;
        self.handler.check(&ctx, &Box::from(req.get_tx()))
    }

    fn deliver_tx(&mut self, req: &RequestDeliverTx) -> ResponseDeliverTx {
        let mut ctx = self.base_context.clone();
        ctx.phase = Deliver;
        self.handler.deliver(&ctx, &Box::from(req.get_tx()))
    }

    fn end_block(&mut self, req: &RequestEndBlock) -> ResponseEndBlock {
        let mut ctx = self.base_context.clone();
        ctx.phase = EndBlock;
        self.handler.end_block(&ctx, req)
    }

    fn commit(&mut self, req: &RequestCommit) -> ResponseCommit {
        let mut ctx = self.base_context.clone();
        ctx.phase = Commit;
        self.handler.commit(&ctx, req)
    }

    fn query(&mut self, req: &RequestQuery) -> ResponseQuery {
        let mut ctx = self.base_context.clone();
        ctx.phase = Query;
        self.handler.query(&ctx, req)
    }
}

impl ABCIBaseApp {
    fn load_tx(&self, tx_bytes: &[u8]) -> Result<Box<dyn Tx>, Box<dyn Error>> {
        unimplemented!()
    }
}