use ed25519_dalek::{Keypair, PublicKey};
use ed25519_dalek::Signature;
use crate::auth::{PubKey, HasCondition, Condition, EXT_SIG};

pub struct Ed25519PubKey(PublicKey);

pub fn from_bytes(bytes: &[u8]) -> Ed25519PubKey {
//    match PublicKey::from_bytes(bytes) {
//
//    }
    unimplemented!()
}

impl HasCondition for Ed25519PubKey {
    fn condition(&self) -> Condition {
        return Condition {
            ext: String::from(EXT_SIG),
            typ: String::from("ed25519"),
            data: panic!() //self.0.to_bytes()
        }
    }
}

impl PubKey for Ed25519PubKey {
    fn verify(&self, message: &[u8], sig: &[u8]) -> bool {
        match Signature::from_bytes(sig) {
            Err(e) => false,
            Ok(sig) =>
                match self.0.verify(message,&sig) {
                    Err(e) => false,
                    Ok(_) => true
                }
        }
    }
}
