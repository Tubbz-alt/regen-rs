use crate::context::Context;
use crate::tx::Tx;
use crate::result::{CheckResult, DeliverResult, Res};
use std::error::Error;
use std::collections::HashMap;
use grpc::rt::{ServerMethod, MethodHandler};
use std::ops::Deref;
use crate::handler::Handler;

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

impl Handler for GrpcHandler {
    fn check(&self, ctx: &Context, tx: &Box<dyn Tx>) -> CheckResult {
//        match self.method_resolver.resolve(tx.get_msg()) {
//            Err(e) => Err(e),
//            Ok(mc) => {
//                match self.methods.get(&mc.name) {
//                    None => panic!(),
//                    Some(method) => {
////                        method.handle()
//                        panic!()
//                    }
//                }
//            }
//        }
        unimplemented!()
    }

    fn deliver(&self, ctx: &Context, tx: &Box<dyn Tx>) -> DeliverResult {
        unimplemented!()
    }
}
