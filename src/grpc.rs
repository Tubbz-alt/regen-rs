use crate::handler::{Handler, Checker, Deliverer, Querier};
use crate::context::Context;
use crate::tx::Tx;
use crate::result::{CheckResult, DeliverResult, Res};
use std::error::Error;
use std::collections::HashMap;
use grpc::rt::{ServerMethod, MethodHandler};
use std::ops::Deref;

struct GrpcHandler {
    method_resolver: Box<dyn MethodResolver>,
    methods: HashMap<String, ServerMethod>
}

pub struct MethodCall {
    name: String,
    body: Box<[u8]>
}

pub trait MethodResolver {
    fn resolve(&self, msg: &[u8]) -> Res<MethodCall>;
}

impl Handler for GrpcHandler {}

impl Querier for GrpcHandler {}

impl Checker for GrpcHandler {
    fn check(&self, ctx: &dyn Context, tx: &dyn Tx) -> Result<CheckResult, Box<dyn Error>> {
        match self.method_resolver.resolve(tx.get_msg()) {
            Err(e) => Err(e),
            Ok(mc) => {
                match self.methods.get(&mc.name) {
                    None => panic!(),
                    Some(method) => {
//                        method.handle()
                        panic!()
                    }
                }
            }
        }
    }
}

impl Deliverer for GrpcHandler {
    fn deliver(&self, ctx: &dyn Context, tx: &dyn Tx) -> Result<DeliverResult, Box<dyn Error>> {
        unimplemented!()
    }
}
