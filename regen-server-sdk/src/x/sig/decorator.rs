use crate::context::{Context, BLOCK_HEADER};
use crate::result::{CheckResult, DeliverResult, Res};
use crate::tx::{Tx, StdSignature};
use crate::table::Table;
use crate::x::sig::codec::Account;
use crate::x::sig::codec;
use crate::handler::{Decorator, Handler, TxHandler};
use regen_client_sdk::auth::{Address, Condition, PubKey};
use crate::x::sig::codec::PubKey_oneof_sum::ed25519;

pub struct Keeper {
    auth_table: Box<dyn Table<Address, Account>>
}

pub fn new_keeper() -> Box<Keeper> {
    Box::from(Keeper { auth_table: unimplemented!() })
}

impl Decorator for Keeper {
//    fn on_check(&self, ctx: &dyn Context, tx: &Box<dyn Tx>, next: &TxHandler) -> CheckResult {
//        let chain_id: String = ctx.block_header().chain_id.clone();
//        let conds = self.verify_tx_signatures(ctx, tx)?;
//        let new_ctx = ctx.with_conditions(conds.as_ref());
//        next.check(new_ctx.as_ref(), tx)
//    }
//
//    fn on_deliver(&self, ctx: &dyn Context, tx: &Box<dyn Tx>, next: &TxHandler) -> DeliverResult {
//        let hain_id: String = ctx.block_header().chain_id.clone();
//        let conds = self.verify_tx_signatures(ctx, tx)?;
//        let new_ctx = ctx.with_conditions(conds.as_ref());
//        next.deliver(new_ctx.as_ref(), tx)
//    }
}

impl Keeper {
    fn verify_tx_signatures(&self, ctx: &Context, tx: &dyn Tx) -> Res<Box<[Condition]>> {
        let header = ctx.get(&BLOCK_HEADER)?;
        let chain_id = &header.chain_id;
        let sign_bytes = tx.get_sign_bytes();
        let sigs = tx.get_signatures();
        let mut signers = Vec::new();
        for sig in sigs.iter() {
            let cond = self.verify_signature(ctx, sig.as_ref(), sign_bytes, chain_id)?;
            signers.push(cond);
        }
        Ok(Box::from(signers))
    }

    fn verify_signature(&self, ctx: &Context, sig: &dyn StdSignature, sign_bytes: &[u8], chain_id: &str) -> Res<Condition> {
        let cond = sig.get_pub_key().condition();
        let addr = ctx.condition_address(&cond);
        let acc = self.get_or_create_account(ctx, &addr);
        let seq = sig.get_sequence();
        let to_sign = build_sign_bytes(sign_bytes, chain_id, seq);
        let pk = wrap_pub_key(acc.get_pubkey())?;
        if !pk.verify(to_sign.as_ref(), sig.get_signature()) {
            panic!()
        }
        let new_acc = acc.check_and_increment_sequence(seq)?;
        self.auth_table.save(ctx, &new_acc);
        Ok(cond)
    }

    fn get_or_create_account(&self, ctx: &Context, addr: &Address) -> Account {
        match self.auth_table.get_one(ctx, addr) {
            Err(e) => Account {
                address: Vec::from(addr.0.clone()),
                pubkey: Default::default(),
                sequence: 0,
                metadata: vec![],
                unknown_fields: Default::default(),
                cached_size: Default::default(),
            },
            Ok(acct) => acct.value()
        }
    }
}

fn build_sign_bytes(sign_bytes: &[u8], chain_id: &str, sequence: u64) -> Box<[u8]> {
    unimplemented!()
}

fn wrap_pub_key(pk: &codec::PubKey) -> Res<Box<dyn PubKey>> {
//    if pk.has_ed25519() {
//        return Ok(Box::from(ed25519::from_bytes(pk.get_ed25519())));
//    }
    unimplemented!()
}

impl Account {
    fn check_and_increment_sequence(&self, seq: u64) -> Res<Account> {
        if self.get_sequence() != seq {
            panic!()
        }
        let mut res = self.clone();
        res.set_sequence(seq + 1);
        Ok(res)
    }
}
