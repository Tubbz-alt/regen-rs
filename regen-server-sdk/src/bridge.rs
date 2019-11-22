use regen_client_sdk::auth::Condition;
use crate::handler::{Handler, RawHandler};
use abci::{RequestQuery, Application};
use protobuf::{parse_from_bytes, Message};
use crate::context::Context;

#[repr(C)]
pub struct Bridge{
    app: dyn Application
}

#[repr(C)]
pub struct App;

//pub fn init(app: &App, call_out: fn(conditions: &[u8], msg: &[u8]) -> &[u8]) -> Box<Bridge>{
//    unimplemented!();
//}
//
//pub fn call_into(b: &mut Bridge, conditions: &[u8], msg: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn info(b: &mut Bridge, req: &[u8]) -> Vec<u8> {
//    wrap(req, |req| b.app.info(req))
//}
//
//pub fn init_chain(b: &Bridge, req: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn begin_block(b: &Bridge, req: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn check(b: &Bridge, tx: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn deliver(b: &Bridge, tx: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn end_block(b: &Bridge, req: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn commit(b: &Bridge, req: &[u8]) -> &[u8] {
//    unimplemented!();
//}
//
//pub fn query(b: &mut Bridge, req: &[u8]) -> Vec<u8> {
//    wrap(req, |req| b.app.query(req))
//}
//
//fn wrap<Req, Res>(req: &[u8], f: fn(Req) -> Res) -> Vec<u8> {
//    match parse_from_bytes(req) {
//        Err(e) => panic!(),
//        Ok(req) => {
//            let res = f(req);
//            match res.write_to_bytes() {
//                Err(e) => panic!(),
//                Ok(bz) => bz
//            }
//        }
//    }
//}
//
