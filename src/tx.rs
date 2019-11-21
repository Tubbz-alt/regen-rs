use std::error::Error;
use crate::auth::PubKey;

pub trait Msg {
    fn get_route(&self) -> u64;
    fn get_bytes(&self) -> &[u8];
}

pub trait Tx {
    fn get_msg(&self) -> Result<Box<dyn Msg>, Box<dyn Error>>;
    fn get_sign_bytes(&self) -> &[u8];
    fn get_signatures(&self) -> &[Box<dyn StdSignature>];
}

pub trait StdSignature {
    fn get_sequence(&self) -> u64;
    fn get_pub_key(&self) -> Box<dyn PubKey>;
    fn get_signature(&self) -> &[u8];
}