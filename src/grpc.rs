use crate::handler::{Handler, Checker, Deliverer};
use crate::context::Context;
use crate::tx::Tx;
use crate::result::{CheckResult, DeliverResult};
use std::error::Error;
use std::collections::HashMap;
use grpc::rt::ServerMethod;

struct GrpcHandler {
    method_resolver: dyn MethodResolver,
    methods: HashMap<String, ServerMethod>
}

pub struct MethodCall {
    name: string,
    body: Box<[u8]>
}

pub trait MethodResolver {
    fn resolve(&self, msg: &[u8]) -> MethodCall;
}

impl Handler for GrpcHandler {}

impl Checker for GrpcHandler {
    fn check(&self, ctx: &dyn Context, tx: &dyn Tx) -> Result<CheckResult, Box<dyn Error>> {
        unimplemented!()
    }
}

impl Deliverer for GrpcHandler {
    fn deliver(&self, ctx: &dyn Context, tx: &dyn Tx) -> Result<DeliverResult, Box<dyn Error>> {
        unimplemented!()
    }
}
