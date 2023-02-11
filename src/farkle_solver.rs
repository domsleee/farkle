use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::dice_set;
use crate::defs::*;

use crate::precompute::Precomputed;

type MutableCache = HashMap<u64, (ProbType, Action)>;

const DEBUG: bool = false;
#[wasm_bindgen]
#[derive(Default)]
pub struct FarkleSolver {
    farkle_solver_internal: FarkleSolverInternal,
    cache_decide_action: MutableCache
}

#[derive(Default)]
struct FarkleSolverInternal {
    precomputed: Precomputed
}

#[wasm_bindgen]
impl FarkleSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> FarkleSolver {
        FarkleSolver::default()
    }

    pub fn decide_action_ext(&mut self, held_score: ScoreType, dice_left: usize, scores: Vec<ScoreType>) -> String {
        let (prob, action) = self.farkle_solver_internal.decide_action(&mut self.cache_decide_action, held_score, dice_left, &scores);
        action.to_string()
    }

    pub fn decide_held_dice_ext(&mut self, held_score: ScoreType, roll: String, scores: Vec<ScoreType>) -> String {
        let (prob, held_dice) = self.farkle_solver_internal.decide_held_dice(&mut self.cache_decide_action, held_score, dice_set::from_string(&roll), &scores);
        dice_set::to_sorted_string(held_dice)
    }
}

impl FarkleSolver {
    pub fn decide_action_ext2(&mut self, held_score: ScoreType, dice_left: usize, scores: Vec<ScoreType>) -> (ProbType, Action) {
        self.farkle_solver_internal.decide_action(&mut self.cache_decide_action, held_score, dice_left, &scores)
    }
}

impl FarkleSolverInternal {
    fn decide_action(&self, cache_decide_action: &mut MutableCache, held_score: ScoreType, dice_left: usize, scores: &Vec<ScoreType>) -> (ProbType, Action) {
        if held_score + scores[0] >= SCORE_WIN {
            return (get_val(1), Action::Stay);
        }
        
        let cache_key = Self::get_cache_key(held_score, dice_left, scores);
        if cache_decide_action.contains_key(&cache_key) {
            return *cache_decide_action.get(&cache_key).unwrap();
        }
        if DEBUG { println!("decide_action({held_score}, {dice_left}, {scores:?})"); }

        let mut rotated_scores = {
            let mut new_scores = scores.clone();
            new_scores.rotate_left(1);
            new_scores
        };

        // you can stay
        let prob_win_stay = {
            let mut pstay = get_val(0);
            if held_score > 0 {
                pstay = {
                    let mut new_scores = rotated_scores.clone();
                    *new_scores.last_mut().unwrap() += held_score;
                    get_val(1) - self.decide_action(cache_decide_action, 0, NUM_DICE, &new_scores).0
                };
            }
            pstay
        };

        // you can roll
        // `get_ok_rolls` can be grouped by the output of `get_valid_holds`. e.g. 14 and 16 both have the same valid holds of [1]
        let (ok_rolls, rem_prob) = self.precomputed.get_ok_rolls(dice_left);
        let mut prob_roll = get_val(0);
        if prob_win_stay < get_val(1) {
            *rotated_scores.last_mut().unwrap() += ScoreType::max(held_score, 50); // note: approx
            prob_roll = rem_prob * (get_val(1) - self.decide_action(cache_decide_action, 0, NUM_DICE, &rotated_scores).0);
            for (roll, prob) in ok_rolls {
                let decision = self.decide_held_dice(cache_decide_action, held_score, *roll, scores);
                prob_roll += prob * decision.0;
            }
        }

        let res = if prob_win_stay >= prob_roll { (prob_win_stay, Action::Stay) } else { (prob_roll, Action::Roll) };
        
        if DEBUG { println!("decide_action(held_score: {held_score}, dice_left: {dice_left}, scores: {scores:?}): prob_win_stay: {prob_win_stay}, prob_roll: {prob_roll}"); }
        cache_decide_action.insert(cache_key, res);
        res
    }

    pub fn decide_held_dice(&self, cache_decide_action: &mut MutableCache, held_score: ScoreType, roll: dice_set::DiceSet, scores: &Vec<ScoreType>) -> (ProbType, dice_set::DiceSet) {
        //if DEBUG { println!("decide_held_dice({held_score}, {}, {scores:?})", to_sorted_string(roll)); }
        //return (get_val(1), *self.precomputed.get_valid_holds(roll).iter().nth(0).unwrap_or(&0));
        let (mut max_prob, mut max_comb) = (get_val(-1), dice_set::empty());
        for hold in self.precomputed.get_valid_holds(roll) {
            let new_held_score = held_score + self.precomputed.calc_score(roll);
            let new_dice_left = self.precomputed.get_num_dice(roll) - self.precomputed.get_num_dice(*hold);
            let (new_prob, _) = self.decide_action(cache_decide_action, new_held_score, new_dice_left, scores);
            if new_prob > max_prob {
                (max_prob, max_comb) = (new_prob, hold.to_owned());
            }
        }
        
        (max_prob, max_comb)
    }

    fn get_cache_key(held_score: ScoreType, dice_left: usize, scores: &Vec<ScoreType>) -> u64 {
        debug_assert!(scores.len() < 7);
        let mut key = 0u64;
        key |= Self::score_to_byte(held_score) as u64;
        key |= (dice_left as u64) << 8;
        for (i, score) in scores.iter().enumerate() {
            key |= (*score as u64) << (16 + 8*i);
        }
        key
    }

    fn score_to_byte(score: ScoreType) -> u8 { (score / 200) as u8 }
}