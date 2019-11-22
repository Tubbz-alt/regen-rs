use err_derive::Error;
use std::error::Error;

#[derive(Debug, Error)]
pub enum ABCIError {
    #[error(display="not found")]
    NotFound,
    #[error(display="{:?}", _0)]
    Other(String),
    #[error(display="{:?}", _0)]
    Wrap(Box<dyn Error>)
}
