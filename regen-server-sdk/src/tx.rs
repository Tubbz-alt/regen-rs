use std::error::Error;
use regen_client_sdk::auth::PubKey;

pub trait Tx {
    fn get_msg(&self) -> &[u8];
    fn get_sign_bytes(&self) -> &[u8];
    fn get_signatures(&self) -> &[Box<dyn StdSignature>];
}

pub trait StdSignature {
    fn get_sequence(&self) -> u64;
    fn get_pub_key(&self) -> Box<dyn PubKey>;
    fn get_signature(&self) -> &[u8];
}