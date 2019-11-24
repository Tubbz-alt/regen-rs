use std::marker::PhantomData;
use std::sync::Arc;
use std::any::Any;
use err_derive::Error;
use crate::ContextError::TypeConversionFailed;

#[derive(Debug, Error)]
pub enum ContextError {
    #[error(display="not found")]
    NotFound,
    #[error(display="type conversion failed for key {:?}", key)]
    TypeConversionFailed{key: String},
}

pub struct ContextKey<T>(pub &'static str, pub PhantomData<T>);

impl <T: 'static> ContextKey<T> {
    fn get(&self, ctx: &dyn Context2) -> Result<&T, ContextError> {
        let any = ctx.get_raw(self.0)?;
        match any.downcast_ref::<T>() {
            None => Err(TypeConversionFailed{key: String::from(self.0)}),
            Some(x) => Ok(x)
        }
    }

    fn set(&self, ctx: &dyn Context2, value: T) -> &dyn Context2 {
        unimplemented!()
    }

    fn unset(&self, ctx: &dyn Context2) -> &dyn Context2 {
        unimplemented!()
    }
}

#[derive(Default, Clone)]
pub struct SimpleContext(im::HashMap<String, Arc<dyn Any>>);

pub trait Context {
    fn get<T: 'static>(&self, key: &ContextKey<T>) -> Result<&T, ContextError>;
    fn with<T: Any>(&self, key: &ContextKey<T>, value: T) -> Self;
    fn without<T>(&self, key: &ContextKey<T>) -> Self;
}

pub trait Context2 {
    fn get_raw(&self, key: &str) -> Result<&Arc<dyn Any>, ContextError>;
    fn with_raw(&self, key: &str, value: Box<dyn Any>) -> Box<dyn Context2>;
    fn without_raw(&self, key: &str) -> Box<Context2>;
}

impl SimpleContext {
    pub fn new() -> SimpleContext {
        SimpleContext(im::HashMap::new())
    }
}

impl Context for SimpleContext {
    fn get<T: 'static>(&self, key: &ContextKey<T>) -> Result<&T, ContextError> {
        match self.0.get(key.0) {
            None => Err(ContextError::NotFound),
            Some(v) => match v.downcast_ref::<T>() {
                None => Err(TypeConversionFailed{key: String::from(key.0)}),
                Some(x) => Ok(x)
            }
        }
    }

    fn with<T: Any>(&self, key: &ContextKey<T>, value: T) -> Self {
        SimpleContext(self.0.update(String::from(key.0), Arc::from(value)))
    }

    fn without<T>(&self, key: &ContextKey<T>) -> Self {
        SimpleContext(self.0.without(key.0))
    }
}
