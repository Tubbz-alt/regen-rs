pub mod ed25519;

#[derive(Clone)]
pub struct Condition {
    ext: String,
    typ: String,
    data: Box<[u8]>
}

impl ToString for Condition {
    fn to_string(&self) -> String {
//        format!("{}/{}/{}", &self.ext, &self.typ, &self.data)
        panic!()
    }
}

pub struct Address(pub Box<[u8]>);

pub trait HasCondition {
    fn condition(&self) -> Condition;
}

pub trait PubKey: HasCondition {
    fn verify(&self, message: &[u8], sig: &[u8]) -> bool;
}

pub trait Signer {
    fn sign(&self, message: &[u8]) -> Box<[u8]>;
    fn pub_key(&self) -> Box<dyn PubKey>;
}

const EXT_SIG: &str = "sig";
