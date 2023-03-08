use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{farkle_solver::FarkleSolver, defs::{ScoreType, Action, get_val, NUM_DICE}, dice_set};

#[wasm_bindgen]
#[derive(Default)]
pub struct FarkleSolverWasm {
    solver: FarkleSolver<2>
}

#[wasm_bindgen]
impl FarkleSolverWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FarkleSolverWasm {
        FarkleSolverWasm::default()
    }

    pub fn decide_action_ext(&mut self, held_score: ScoreType, dice_left: usize, scores: Vec<ScoreType>) -> JsValue {
        let res = self.solver.decide_action_ext(held_score, dice_left, [scores[0], scores[1]]);
        let val = (res.0, if res.1 == Action::Roll { "Roll" } else { "Stay" });
        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn decide_held_dice_ext(&mut self, held_score: ScoreType, roll: String, scores: Vec<ScoreType>) -> JsValue {
        let roll_diceset = dice_set::from_string(&roll);
        let (mut prob, held_dice) = self.solver.decide_held_dice_ext(held_score, roll_diceset, [scores[0], scores[1]]);
        if prob == get_val(-1) {
            prob = get_val(1) - self.solver.decide_action_ext(0, NUM_DICE, [scores[1], scores[0]]).0;
        }
        let held_score = self.solver.farkle_solver_internal.precomputed.calc_score(held_dice);
        let val = (prob, held_score, dice_set::to_sorted_string(held_dice));
        serde_wasm_bindgen::to_value(&val).unwrap()
    }

    pub fn get_is_approx(&self) -> bool {
        self.solver.farkle_solver_internal.is_approx
    }
}

impl FarkleSolverWasm {
    pub fn set_cache(&mut self, cache: &crate::farkle_solver::DecideActionCache) {
        self.solver.set_cache(cache);
    }
}