use is_sorted::IsSorted;
use itertools::{repeat_n, Itertools};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use crate::{
    defs::{get_val, ProbType, ScoreType},
    dice_set::{self, DiceSet},
};
extern crate web_sys;

#[derive(Clone)]
pub struct Precomputed {
    cache_calc_score: Vec<ScoreType>,
    cache_get_valid_rolls: Vec<Option<Vec<DiceSet>>>,
    cache_get_num_dice: Vec<usize>,
    cache_get_rolls: Vec<Vec<DiceSet>>,
    cache_get_ok_rolls: Vec<(Vec<(DiceSet, ProbType)>, ProbType)>,
    cache_get_ok_rolls_grouped: Vec<(Vec<(DiceSet, ProbType)>, ProbType)>,
}

impl Default for Precomputed {
    fn default() -> Precomputed {
        let mut precomputed = Precomputed {
            cache_calc_score: (0..=dice_set::MAX_VAL).map(|_| ScoreType::MAX).collect(),
            cache_get_valid_rolls: (0..=dice_set::MAX_VAL).map(|_| Option::None).collect(),
            cache_get_num_dice: (0..=dice_set::MAX_VAL).map(|_| 0).collect(),
            cache_get_rolls: (0..=6).map(|_| Vec::new()).collect(),
            cache_get_ok_rolls: (0..=6).map(|_| (Vec::new(), get_val(0))).collect(),
            cache_get_ok_rolls_grouped: (0..=6).map(|_| (Vec::new(), get_val(0))).collect(),
        };

        let mut all_valid_dicesets = Vec::new();
        for k in 0..=6 {
            precomputed.cache_get_rolls[k] = precomputed.get_rolls_mut(k);
            for v in &precomputed.cache_get_rolls[k] {
                all_valid_dicesets.push(*v);
            }
        }

        for dice in &all_valid_dicesets {
            let score = precomputed.calc_score_mut(*dice);
            precomputed.cache_calc_score[*dice as usize] = score;
        }

        for dice in &all_valid_dicesets {
            let vec = precomputed.get_valid_holds_mut(*dice);
            precomputed.cache_get_valid_rolls[*dice as usize] = Some(vec);
            precomputed.cache_get_num_dice[*dice as usize] =
                dice_set::to_sorted_string(*dice).len();
        }

        for k in 0..=6 {
            precomputed.cache_get_ok_rolls[k] = precomputed.get_ok_rolls_mut(k);
            precomputed.cache_get_ok_rolls_grouped[k] = precomputed.get_ok_rolls_grouped_mut(k);
        }

        precomputed
    }
}

impl Precomputed {
    pub fn calc_score(&self, roll: dice_set::DiceSet) -> ScoreType {
        self.cache_calc_score[roll as usize]
    }

    pub fn get_valid_holds(&self, roll: dice_set::DiceSet) -> &Vec<DiceSet> {
        self.cache_get_valid_rolls[roll as usize].as_ref().unwrap()
    }

    pub fn get_num_dice(&self, dice: dice_set::DiceSet) -> usize {
        self.cache_get_num_dice[dice as usize]
    }

    pub fn get_rolls(&self, dice_left: usize) -> &Vec<DiceSet> {
        &self.cache_get_rolls[dice_left]
    }

    pub fn get_ok_rolls(&self, dice_left: usize) -> &(Vec<(DiceSet, ProbType)>, ProbType) {
        &self.cache_get_ok_rolls[dice_left]
    }

    pub fn get_ok_rolls_merged(&self, dice_left: usize) -> &(Vec<(DiceSet, ProbType)>, ProbType) {
        &self.cache_get_ok_rolls_grouped[dice_left]
    }

    fn get_valid_holds_mut(&self, roll: dice_set::DiceSet) -> Vec<DiceSet> {
        let mut res: Vec<DiceSet> = Vec::new();
        let chars = dice_set::to_human_readable(roll);
        let mut max_lt_k_score: ScoreType = 0;
        for k in 1..=chars.len() {
            let mut k_combinations: Vec<DiceSet> = Vec::new();
            for comb in chars.iter().combinations(k) {
                let comb_dice = dice_set::from_human_readable_str(&comb);
                k_combinations.push(comb_dice);
            }

            // d1 is "better" than d2 if
            // size(d1) <= size(d2), and
            // score(d1) >= score(d2)
            k_combinations.sort();
            k_combinations.sort_by_key(|b| std::cmp::Reverse(self.calc_score(*b)));

            if k_combinations.is_empty() {
                continue;
            }
            let dice = *k_combinations.first().unwrap();
            let max_k_score = self.calc_score(dice);
            if max_k_score == 0 {
                continue;
            }
            if max_k_score <= max_lt_k_score {
                continue;
            }
            max_lt_k_score = ScoreType::max(max_lt_k_score, max_k_score);
            if k == chars.len() {
                return vec![dice];
            }
            res.push(dice);
        }

        assert!(
            res.len() <= 5,
            "{} {}",
            res.len(),
            dice_set::to_sorted_string(roll)
        );
        res.sort();
        res
    }

    fn calc_score_mut(&mut self, dice: dice_set::DiceSet) -> ScoreType {
        if self.cache_calc_score[dice as usize] != ScoreType::MAX {
            return self.cache_calc_score[dice as usize];
        }

        let mut max_score = 0;
        let melds: HashMap<&str, ScoreType> = HashMap::from([
            ("1", 100),
            ("5", 50),
            ("111", 1000),
            ("222", 200),
            ("333", 300),
            ("444", 400),
            ("555", 500),
            ("666", 600),
            // four of kind: 1000
            // five of kind: 2000
            // six of kind: 3000
            // three pairs: 1500
            ("123456", 2500),
        ]);
        let sorted_str = dice_set::to_sorted_string(dice);

        for (meld, score) in melds {
            if sorted_str.contains(meld) {
                max_score = ScoreType::max(
                    max_score,
                    score + self.calc_score(dice_set::subtract_dice(dice, meld)),
                );
            }
        }

        let freqdist = dice_set::get_freqdist(dice);
        for c in dice_set::get_chars() {
            if freqdist[c] >= 4 {
                let four_dice = c.to_string().repeat(4).to_string();
                max_score = ScoreType::max(
                    max_score,
                    1000 + self.calc_score(dice_set::subtract_dice(dice, &four_dice)),
                );
            }
            if freqdist[c] >= 5 {
                let five_dice = c.to_string().repeat(5).to_string();
                max_score = ScoreType::max(
                    max_score,
                    2000 + self.calc_score(dice_set::subtract_dice(dice, &five_dice)),
                );
            }
            if freqdist[c] >= 6 {
                max_score = ScoreType::max(max_score, 3000);
            }
        }

        let vals: HashSet<usize> = HashSet::from_iter(freqdist.iter().map(|(_, b)| *b));
        if sorted_str.len() == 6 {
            let only_twos = HashSet::from_iter([2]);
            let four_and_two = HashSet::from_iter([2, 4]);
            if vals == only_twos || vals == four_and_two {
                max_score = ScoreType::max(max_score, 1500);
            }
        }

        let res = max_score;
        self.cache_calc_score[dice as usize] = res;
        res
    }

    fn get_rolls_mut(&mut self, dice_left: usize) -> Vec<DiceSet> {
        if dice_left == 0 {
            return vec![dice_set::empty()];
        }
        let mut res = Vec::new();
        for comb in dice_set::get_chars()
            .iter()
            .combinations_with_replacement(dice_left)
        {
            let act_comb = comb.iter().copied().join("");
            let new_dice_set = dice_set::from_string(&act_comb);
            res.push(new_dice_set);
        }
        res
    }

    fn get_ok_rolls_mut(&mut self, dice_left: usize) -> (Vec<(DiceSet, ProbType)>, ProbType) {
        let mut zero_tot = 0;
        let mut ok_rolls: Vec<(DiceSet, ProbType)> = Vec::new();
        let roll_freq = self.get_roll_distribution(dice_left);
        let total_ct: usize = roll_freq.values().sum();
        for (roll, roll_ct) in roll_freq {
            if self.calc_score(roll) == 0 {
                zero_tot += roll_ct;
                continue;
            }
            ok_rolls.push((roll, get_val(roll_ct as i64) / get_val(total_ct as i64)));
        }
        let rem_prob = get_val(zero_tot as i64) / get_val(total_ct as i64);
        (ok_rolls, rem_prob)
    }

    fn get_roll_distribution(&self, dice_left: usize) -> HashMap<DiceSet, usize> {
        let mut roll_freq: HashMap<DiceSet, usize> = HashMap::new();
        let iter = dice_set::get_chars().iter();
        for comb in repeat_n(iter, dice_left).multi_cartesian_product() {
            let act_comb = comb.iter().copied().join("");
            let new_dice_set = dice_set::from_string(&act_comb);
            roll_freq.entry(new_dice_set).or_insert(0);
            *roll_freq.get_mut(&new_dice_set).unwrap() += 1;
        }
        roll_freq
    }

    fn get_ok_rolls_grouped_mut(
        &mut self,
        dice_left: usize,
    ) -> (Vec<(DiceSet, ProbType)>, ProbType) {
        // idea: group ok_rolls by their valid_holds, and sum their probabilities
        //
        let (ok_rolls, rem_prob) = &self.get_ok_rolls(dice_left);

        let mut valid_holds_to_roll: HashMap<_, (DiceSet, ProbType)> = HashMap::new();
        for (roll, prob) in ok_rolls {
            let valid_holds = self.get_valid_holds(*roll).to_owned();
            assert!(IsSorted::is_sorted(&mut valid_holds.iter()));
            if !valid_holds_to_roll.contains_key(&valid_holds) {
                valid_holds_to_roll.insert(valid_holds, (*roll, *prob));
            } else {
                let (rep_roll, curr_prob) = valid_holds_to_roll.get(&valid_holds).unwrap();
                valid_holds_to_roll.insert(valid_holds, (*rep_roll, curr_prob + prob));
            }
        }

        (valid_holds_to_roll.values().copied().collect(), *rem_prob)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::dice_set::{self};

    use super::Precomputed;
    #[test]
    pub fn test_calc_score() {
        let precomputed = Precomputed::default();
        assert_eq!(100, precomputed.calc_score(dice_set::from_string("1")));
        assert_eq!(50, precomputed.calc_score(dice_set::from_string("5")));
        assert_eq!(1000, precomputed.calc_score(dice_set::from_string("111")));
        assert_eq!(200, precomputed.calc_score(dice_set::from_string("222")));
        assert_eq!(300, precomputed.calc_score(dice_set::from_string("333")));
        assert_eq!(400, precomputed.calc_score(dice_set::from_string("444")));
        assert_eq!(500, precomputed.calc_score(dice_set::from_string("555")));
        assert_eq!(600, precomputed.calc_score(dice_set::from_string("666")));
        assert_eq!(1000, precomputed.calc_score(dice_set::from_string("2222")));
        assert_eq!(2000, precomputed.calc_score(dice_set::from_string("22222")));
        assert_eq!(
            3000,
            precomputed.calc_score(dice_set::from_string("222222"))
        );
        assert_eq!(
            1500,
            precomputed.calc_score(dice_set::from_string("112233"))
        );
        assert_eq!(
            2500,
            precomputed.calc_score(dice_set::from_string("123456"))
        );
        assert_eq!(
            2500,
            precomputed.calc_score(dice_set::from_string("654321"))
        );
        assert_eq!(
            1500,
            precomputed.calc_score(dice_set::from_string("336666"))
        );
        assert_eq!(1100, precomputed.calc_score(dice_set::from_string("1111")));
    }

    #[test]
    pub fn test_get_valid_holds() {
        let precomputed = Precomputed::default();
        let holds = precomputed.get_valid_holds(dice_set::from_string("14"));
        assert!(holds.len() == 1);
    }

    #[test]
    pub fn test_ok_rolls() {
        let precomputed = Precomputed::default();
        let (ok_rolls, rem_prob) = precomputed.get_ok_rolls(2);
        let ok_rolls_human = ok_rolls
            .iter()
            .map(|x| (dice_set::to_sorted_string(x.0), x.1))
            .collect_vec();
        let sum_ok: f64 = ok_rolls.iter().map(|x| x.1).sum();
        println!("{:?} {sum_ok} {rem_prob}", ok_rolls_human);
        assert!(false);
    }
}
