use wasm_bindgen::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;
use std::any::Any;
use std::fmt::{Display, Formatter, Debug};
use crate::Error::{NotFound, UnknownError};
use wasm_bindgen::__rt::core::any::TypeId;

#[macro_use]
extern crate derive_error;

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn greet() -> Test {
    Test{a: String::from("test")}
}

#[wasm_bindgen]
#[repr(C)]
pub struct Test {
    a: String
}

#[wasm_bindgen]
#[no_mangle]
pub extern "C" fn get_a(x: &Test) -> String {
    x.a.clone()
}

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;

pub trait ContextKey {
    type T;

    fn get<'a>(&'static self, ctx: &'a Context) -> Res<&'a Self::T> {
        match ctx.0.get(&self.type_id()) {
            None => Err(Box::from(NotFound)),
            Some(v) => match v.downcast_ref::<Self::T>() {
                None => Err(Box::from(UnknownError)),
                Some(x) => Ok(x)
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Context(im::HashMap<TypeId, Arc<dyn Any>>);

struct Version;

impl ContextKey for Version {
    type T = String;
}

impl Context {
    pub fn new() -> Context {
        Context(im::HashMap::new())
    }

//    pub fn get<T: 'static>(&self, key: &str) -> Res<&T> {
//        match self.0.get(key) {
//            None => Err(Box::from(NotFound)),
//            Some(v) => match v.downcast_ref::<T>() {
//                None => Err(Box::from(UnknownError)),
//                Some(x) => Ok(x)
//            }
//        }
//    }

//    pub fn with<T: Any>(&self, key: &ContextKey<T>, value: T) -> Self {
//        Context(self.0.update(key.0.clone(), Arc::from(value)))
//    }
//
//    pub fn without<T>(&self, key: &ContextKey<T>) -> Self {
//        Context(self.0.without("x"))
//    }

//    /// Gets the key from the context returning the associated value, if any
//    /// and returns a context without that key
//    pub fn take<T: 'static>(&self, key: &ContextKey<T>) -> (Res<&T>, &Self) {
//        match self.get(key) {
//            Err(e) => (Err(e), self),
//            Ok(x) => (Ok(x), &self.without(key))
//        }
//    }
}


fn test(ctx: &Context) -> Res<&String> {
    let res = Version.get(ctx)?;
    Ok(res)
}

//pub const VERSION: ContextKey<String> = new_context_key("version");

#[derive(Debug, Error)]
pub enum Error {
    UnknownError,
    Unauthorized,
    NotFound,
}

