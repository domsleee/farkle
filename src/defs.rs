use std::fmt::{self};

use serde_repr::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub const SCORE_WIN: ScoreType = 10000;
pub const NUM_DICE: usize = 6;
pub type ProbType = f64;
pub type HumanReadableDiceSet = Vec<String>;
pub type ScoreType = i32;

pub const PROB_IMPOSSIBLE: ProbType = get_val(0);
pub const PROB_CERTAIN: ProbType = get_val(1);

pub const fn get_val(v: i64) -> ProbType {
    v as f64
}

#[wasm_bindgen]
#[derive(Default, Copy, Clone, Debug, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum Action {
    #[default]
    Stay = 0,
    Roll = 1,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Stay => write!(f, "Stay"),
            Action::Roll => write!(f, "Roll"),
        }
    }
}
