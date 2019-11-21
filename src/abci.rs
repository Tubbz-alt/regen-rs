use crate::handler::{Handler, AppHandler};
use abci::{RequestCheckTx, ResponseCheckTx, RequestBeginBlock, RequestDeliverTx, ResponseDeliverTx, ResponseBeginBlock, RequestEndBlock, ResponseEndBlock, ResponseCommit, RequestCommit, Header};
use crate::tx::Tx;
use std::error::Error;
use crate::context::StdContext;

struct ABCIBaseApp {
//    logger log.Logger

    // name is what is returned from abci.Info
    name: String,

    // Database state (committed, check, deliver....)
//    store: *CommitStore

    // Code to initialize from a genesis file
//    initializer weave.Initializer

    // How to handle queries
//    queryRouter weave.QueryRouter

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
    debug: bool,
}

impl abci::Application for ABCIBaseApp {
    fn begin_block(&mut self, req: &RequestBeginBlock) -> ResponseBeginBlock {
        let ctx = StdContext {
            config: self.base_context.config.clone(),
            header: req.get_header().clone(),
            conditions: Default::default(),
        };
        self.block_context = ctx;
        ResponseBeginBlock::from(self.handler.begin_block(&ctx, req))
    }

    fn check_tx(&mut self, req: &RequestCheckTx) -> ResponseCheckTx {
        let mut res = ResponseCheckTx::new();
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
}

impl ABCIBaseApp {
    fn load_tx(&self, tx_bytes: &[u8]) -> Result<Box<dyn Tx>, Box<dyn Error>> {
        unimplemented!()
    }
}