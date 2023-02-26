use wasm_bindgen::prelude::wasm_bindgen;

use crate::{farkle_solver::FarkleSolver, defs::ScoreType, dice_set};

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

    pub fn decide_action_ext(&mut self, held_score: ScoreType, dice_left: usize, scores: Vec<ScoreType>) -> String {
        let (prob, action) = self.solver.decide_action_ext(held_score, dice_left, [scores[0], scores[1]]);
        action.to_string()
    }

    pub fn decide_held_dice_ext(&mut self, held_score: ScoreType, roll: String, scores: Vec<ScoreType>) -> String {
        let (prob, held_dice) = self.solver.decide_held_dice_ext(held_score, dice_set::from_string(&roll), [scores[0], scores[1]]);
        dice_set::to_sorted_string(held_dice)
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