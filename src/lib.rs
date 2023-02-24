#![allow(dead_code, unused_variables)]
pub mod utils;
pub mod farkle_serialiser;
pub mod farkle_solver;
pub mod farkle_solver_wasm;
pub mod defs;
pub mod dice_set;
pub mod precompute;
pub mod solver_trait;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

