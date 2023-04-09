use crate::defs::*;
use crate::dice_set;
use std::collections::HashMap;

use crate::precompute::Precomputed;

type CacheKeyType = u32; // for 2 players
pub type DecideActionCache = HashMap<CacheKeyType, (ProbType, Action)>;
type PrevDecideActionCache = HashMap<CacheKeyType, ProbType>;

const DEBUG: bool = false;

#[derive(Default)]
pub struct FarkleSolver<const PLAYERS: usize = 2> {
    pub farkle_solver_internal: FarkleSolverInternal<PLAYERS>,
    mutable_data: MutableData,
}

#[derive(Default)]
pub struct MutableData {
    pub cache_decide_action: DecideActionCache,
    pub nodes: usize,
    pub nodes_dice_left: [usize; 7],
}

pub struct FarkleSolverInternal<const PLAYERS: usize = 2> {
    pub precomputed: Precomputed,
    pub is_approx: bool,
    pub cache_previous_run: PrevDecideActionCache,
}

impl<const PLAYERS: usize> Default for FarkleSolverInternal<PLAYERS> {
    fn default() -> Self {
        FarkleSolverInternal {
            precomputed: Precomputed::default(),
            is_approx: false,
            cache_previous_run: PrevDecideActionCache::default(),
        }
    }
}

impl<const PLAYERS: usize> FarkleSolver<PLAYERS> {
    pub fn decide_action_ext(
        &mut self,
        held_score: ScoreType,
        dice_left: usize,
        scores: [ScoreType; PLAYERS],
    ) -> (ProbType, Action) {
        self.farkle_solver_internal.decide_action(
            &mut self.mutable_data,
            held_score,
            dice_left,
            &scores,
        )
    }

    pub fn decide_held_dice_ext(
        &mut self,
        held_score: ScoreType,
        roll: dice_set::DiceSet,
        scores: [ScoreType; PLAYERS],
    ) -> (ProbType, dice_set::DiceSet) {
        self.farkle_solver_internal.decide_held_dice(
            &mut self.mutable_data,
            held_score,
            roll,
            &scores,
        )
    }

    pub fn make_suggested_reserves(&mut self) {
        self.mutable_data.cache_decide_action.reserve(22901694);
    }

    pub fn get_mutable_data(&self) -> &MutableData {
        &self.mutable_data
    }

    pub fn set_cache(&mut self, cache: &DecideActionCache) {
        self.farkle_solver_internal.cache_previous_run.clear();
        for (k, v) in cache.iter() {
            self.farkle_solver_internal
                .cache_previous_run
                .insert(*k, v.0);
        }
        self.mutable_data.cache_decide_action = cache.clone();
    }

    pub fn unpack_cache_key(&self, cache_key: CacheKeyType) -> (ScoreType, usize, Vec<ScoreType>) {
        unpack_cache_key(cache_key)
    }

    pub fn get_nodes_dice_left(&self) -> [usize; 7] {
        self.mutable_data.nodes_dice_left
    }
}

impl<const PLAYERS: usize> FarkleSolverInternal<PLAYERS> {
    // 22901694 in 4m32
    // 200 * 6 * 200^2 = 48e6
    fn decide_action(
        &self,
        mutable_data: &mut MutableData,
        held_score: ScoreType,
        dice_left: usize,
        scores: &[ScoreType; PLAYERS],
    ) -> (ProbType, Action) {
        mutable_data.nodes += 1;
        if held_score + scores[0] >= SCORE_WIN {
            return (PROB_CERTAIN, Action::Stay);
        }
        let max_val = *scores.iter().max().unwrap();
        if max_val >= SCORE_WIN {
            return (PROB_IMPOSSIBLE, Action::Stay);
        }

        debug_assert!(dice_left != 0);

        let cache_key = Self::get_cache_key(held_score, dice_left, scores);
        if let Some(res) = mutable_data.cache_decide_action.get(&cache_key) {
            return *res;
        }
        if DEBUG {
            println!("decide_actionS({held_score}, {dice_left}, {scores:?})");
        }

        let mut rotated_scores = {
            let mut new_scores = *scores;
            new_scores.rotate_left(1);
            new_scores
        };

        // you can stay
        let prob_win_stay = {
            let mut pstay = PROB_IMPOSSIBLE;
            if held_score > 0 {
                pstay = {
                    let mut new_scores = rotated_scores;
                    *new_scores.last_mut().unwrap() += held_score;
                    PROB_CERTAIN - self.decide_action(mutable_data, 0, NUM_DICE, &new_scores).0
                };
            }
            pstay
        };

        // you can roll
        let mut prob_roll = PROB_IMPOSSIBLE;
        if prob_win_stay < PROB_CERTAIN {
            // improvement to reduce #calls to the cache of `decide_held_dice`
            // instead of
            // * (Vec<(DiceSet, ProbType)>d, ProbType), use
            // * (all_holds: Vec<DiceSet>, ok_rolls: Vec<(Vec<usize>, ProbType)>, rem_prob: ProbType)
            let (ok_rolls, rem_prob) = self.precomputed.get_ok_rolls_merged(dice_left);
            debug_assert!(rem_prob > &0f64);
            if self.is_approx {
                *rotated_scores.last_mut().unwrap() += 50; // note: approx
                prob_roll = rem_prob
                    * (PROB_CERTAIN
                        - self
                            .decide_action(mutable_data, 0, NUM_DICE, &rotated_scores)
                            .0);
            } else {
                let nx_cache_key = Self::get_cache_key(0, NUM_DICE, &rotated_scores);
                prob_roll = rem_prob
                    * (PROB_CERTAIN
                        - self
                            .cache_previous_run
                            .get(&nx_cache_key)
                            .unwrap_or_else(|| {
                                panic!("{nx_cache_key} {:?}", unpack_cache_key(nx_cache_key))
                            }));
            }

            for (roll, prob) in ok_rolls {
                let decision = self.decide_held_dice(mutable_data, held_score, *roll, scores);
                prob_roll += prob * decision.0;
            }
        }

        let res = if prob_win_stay >= prob_roll {
            (prob_win_stay, Action::Stay)
        } else {
            (prob_roll, Action::Roll)
        };

        if DEBUG {
            println!("decide_actionE(held_score: {held_score}, dice_left: {dice_left}, scores: {scores:?}): prob_win_stay: {prob_win_stay}, prob_roll: {prob_roll}");
        }
        mutable_data.cache_decide_action.insert(cache_key, res);
        res
    }

    pub fn decide_held_dice(
        &self,
        mutable_data: &mut MutableData,
        held_score: ScoreType,
        roll: dice_set::DiceSet,
        scores: &[ScoreType; PLAYERS],
    ) -> (ProbType, dice_set::DiceSet) {
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
            let (new_prob, _) =
                self.decide_action(mutable_data, new_held_score, new_dice_left, scores);
            if new_prob > max_prob {
                (max_prob, max_comb) = (new_prob, hold.to_owned());
                if max_prob == PROB_CERTAIN {
                    break;
                }
            }
        }

        (max_prob, max_comb)
    }

    pub fn get_cache_key(
        held_score: ScoreType,
        dice_left: usize,
        scores: &[ScoreType; PLAYERS],
    ) -> CacheKeyType {
        debug_assert!(scores.len() < 7);
        let mut key: CacheKeyType = 0;
        key |= score_to_byte(held_score) as CacheKeyType;
        key |= (dice_left as CacheKeyType) << 8;
        for (i, score) in scores.iter().enumerate() {
            key |= (score_to_byte(*score) as CacheKeyType) << (16 + 8 * i);
        }
        key
    }
}

pub fn unpack_cache_key(cache_key: CacheKeyType) -> (ScoreType, usize, Vec<ScoreType>) {
    let held_score = byte_to_score((cache_key & 0xFF) as u8);
    let dice_left = ((cache_key >> 8) & 0xFF) as usize;
    let scores: Vec<ScoreType> = vec![
        byte_to_score(((cache_key >> 16) & 0xFF) as u8),
        byte_to_score(((cache_key >> 24) & 0xFF) as u8),
    ];
    (held_score, dice_left, scores)
}

#[inline]
fn score_to_byte(score: ScoreType) -> u8 {
    (score / 50) as u8
}

#[inline]
fn byte_to_score(byte: u8) -> ScoreType {
    (byte as ScoreType) * 50
}
