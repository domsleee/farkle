#![allow(dead_code, unused_variables)]
mod utils;
mod farkle_solver;
mod defs;
mod dice_set;
mod precompute;
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
pub fn greet() -> String {
    //alert("Hello, farkle!");
    "55".to_string()
}
