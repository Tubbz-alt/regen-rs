use std::error::Error;

pub type Res<T> = Result<T, Box<dyn Error>>;

#[derive(Default)]
pub struct CheckResult {

}

#[derive(Default)]
pub struct DeliverResult {

}


