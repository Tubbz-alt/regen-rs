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

#[derive(Default, Clone)]
pub struct Context(im::HashMap<String, Arc<dyn Any>>);

impl Context {
    pub fn new() -> Context {
        Context(im::HashMap::new())
    }

    pub fn get<T: 'static>(&self, key: &ContextKey<T>) -> Result<&T, ContextError> {
        match self.0.get(key.0) {
            None => Err(ContextError::NotFound),
            Some(v) => match v.downcast_ref::<T>() {
                None => Err(TypeConversionFailed{key: String::from(key.0)}),
                Some(x) => Ok(x)
            }
        }
    }

    pub fn with<T: Any>(&self, key: &ContextKey<T>, value: T) -> Self {
        Context(self.0.update(String::from(key.0), Arc::from(value)))
    }

    pub fn without<T>(&self, key: &ContextKey<T>) -> Self {
        Context(self.0.without(key.0))
    }
}
