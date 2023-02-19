use std::collections::HashMap;
use crate::dice_set;
use crate::defs::*;

use crate::precompute::Precomputed;

pub type MutableCache = HashMap<u64, (ProbType, Action)>;

const DEBUG: bool = false;

#[derive(Default)]
pub struct FarkleSolver<const PLAYERS: usize = 2> {
    farkle_solver_internal: FarkleSolverInternal<PLAYERS>,
    mutable_data: MutableData
}

#[derive(Default)]
pub struct MutableData {
    pub cache_decide_action: MutableCache,
    pub nodes: usize,
    pub nodes_dice_left: [usize; 7]
}


#[derive(Default)]
struct FarkleSolverInternal<const PLAYERS: usize = 2> {
    precomputed: Precomputed
}

impl <const PLAYERS: usize> FarkleSolver<PLAYERS> {
    pub fn decide_action_ext(&mut self, held_score: ScoreType, dice_left: usize, scores: [ScoreType; PLAYERS]) -> (ProbType, Action) {
        self.farkle_solver_internal.decide_action(&mut self.mutable_data, held_score, dice_left, &scores)
    }

    pub fn decide_held_dice_ext(&mut self, held_score: ScoreType, roll: dice_set::DiceSet, scores: [ScoreType; PLAYERS]) -> (ProbType, dice_set::DiceSet) {
        self.farkle_solver_internal.decide_held_dice(&mut self.mutable_data, held_score, roll, &scores)
    }

    pub fn get_mutable_data(&self) -> &MutableData {
        &self.mutable_data
    }

    pub fn set_cache(&mut self, cache: &MutableCache) {
        self.mutable_data.cache_decide_action = cache.clone();
    }

    pub fn unpack_cache_key(&self, cache_key: u64) -> (ScoreType, usize, Vec<ScoreType>) {
        FarkleSolverInternal::<2>::unpack_cache_key(cache_key)
    }

    pub fn get_nodes_dice_left(&self) -> [usize; 7] {
        self.mutable_data.nodes_dice_left
    }
}

impl <const PLAYERS: usize> FarkleSolverInternal<PLAYERS> {
    // 22901694 in 4m32 
    // 200 * 6 * 200^2 = 48e6
    fn decide_action(&self, mutable_data: &mut MutableData, held_score: ScoreType, dice_left: usize, scores: &[ScoreType; PLAYERS]) -> (ProbType, Action) {
        mutable_data.nodes += 1;
        if held_score + scores[0] >= SCORE_WIN {
            return (get_val(1), Action::Stay);
        }
        let max_val = *scores.iter().max().unwrap();
        if max_val >= SCORE_WIN {
            return (get_val(0), Action::Stay);
        }

        debug_assert!(dice_left != 0);
        
        let cache_key = Self::get_cache_key(held_score, dice_left, scores);
        if let Some(res) = mutable_data.cache_decide_action.get(&cache_key) { return *res }
        if DEBUG { println!("decide_actionS({held_score}, {dice_left}, {scores:?})"); }

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
                    get_val(1) - self.decide_action(mutable_data, 0, NUM_DICE, &new_scores).0
                };
            }
            pstay
        };

        // you can roll
        let mut prob_roll = get_val(0);
        if prob_win_stay < get_val(1) {
            // improvement to reduce #calls to the cache of `decide_held_dice`
            // instead of 
            // * (Vec<(DiceSet, ProbType)>, ProbType), use
            // * (all_holds: Vec<DiceSet>, ok_rolls: Vec<(Vec<usize>, ProbType)>, rem_prob: ProbType)
            let (ok_rolls, rem_prob) = self.precomputed.get_ok_rolls_merged(dice_left);
            debug_assert!(rem_prob > &0f64);
            *rotated_scores.last_mut().unwrap() += 50; // note: approx
            prob_roll = rem_prob * (get_val(1) - self.decide_action(mutable_data, 0, NUM_DICE, &rotated_scores).0);
            for (roll, prob) in ok_rolls {
                let decision = self.decide_held_dice(mutable_data, held_score, *roll, scores);
                prob_roll += prob * decision.0;
            }
        }

        let res = if prob_win_stay >= prob_roll { (prob_win_stay, Action::Stay) } else { (prob_roll, Action::Roll) };
        
        if DEBUG { println!("decide_actionE(held_score: {held_score}, dice_left: {dice_left}, scores: {scores:?}): prob_win_stay: {prob_win_stay}, prob_roll: {prob_roll}"); }
        mutable_data.cache_decide_action.insert(cache_key, res);
        res
    }

    pub fn decide_held_dice(&self, mutable_data: &mut MutableData, held_score: ScoreType, roll: dice_set::DiceSet, scores: &[ScoreType; PLAYERS]) -> (ProbType, dice_set::DiceSet) {
        //if DEBUG { println!("decide_held_dice({held_score}, {}, {scores:?})", to_sorted_string(roll)); }
        let (mut max_prob, mut max_comb) = (get_val(-1), dice_set::empty());
        let old_dice_left = self.precomputed.get_num_dice(roll);
        for hold in self.precomputed.get_valid_holds(roll) {
            let new_held_score = held_score + self.precomputed.calc_score(*hold);
            let mut new_dice_left = old_dice_left - self.precomputed.get_num_dice(*hold);
            if new_dice_left == 0 {
                new_dice_left = 6;
            }
            mutable_data.nodes_dice_left[old_dice_left] += 1;
            let (new_prob, _) = self.decide_action(mutable_data, new_held_score, new_dice_left, scores);
            if new_prob > max_prob {
                (max_prob, max_comb) = (new_prob, hold.to_owned());
            }
        }
        
        (max_prob, max_comb)
    }

    fn get_cache_key(held_score: ScoreType, dice_left: usize, scores: &[ScoreType; PLAYERS]) -> u64 {
        debug_assert!(scores.len() < 7);
        let mut key = 0u64;
        key |= Self::score_to_byte(held_score) as u64;
        key |= (dice_left as u64) << 8;
        for (i, score) in scores.iter().enumerate() {
            key |= (Self::score_to_byte(*score) as u64) << (16 + 8*i);
        }
        key
    }

    fn score_to_byte(score: ScoreType) -> u8 { (score / 50) as u8 }
    fn byte_to_score(byte: u8) -> ScoreType { (byte as ScoreType) * 50 }

    pub fn unpack_cache_key(cache_key: u64) -> (ScoreType, usize, Vec<ScoreType>) {
        let held_score = Self::byte_to_score((cache_key & 0xFF) as u8);
        let dice_left = 1;
        let scores: Vec<ScoreType> = vec![Self::byte_to_score(((cache_key >> 16) & 0xFF) as u8), 0];
        return (held_score, dice_left, scores);
    }
}