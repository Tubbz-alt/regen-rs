use crate::handler::Handler;
use crate::x::sig::decorator::{Keeper, new_keeper};

pub fn test1(h: Box<Handler>) -> Box<Handler> {
    h >> new_keeper()
}