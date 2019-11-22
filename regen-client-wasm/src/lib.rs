mod utils;
//pub use regen_client_sdk::auth::ed25519;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> Test {
//    ed25519::from_bytes(&[0;1]);
    alert("Hello, regen-client-wasm!");
    Test{a: String::from("test")}
}

#[wasm_bindgen]
#[repr(C)]
pub struct Test {
    a: String
}

#[wasm_bindgen]
pub fn get_a(x: &Test) -> String {
    x.a.clone()
}

