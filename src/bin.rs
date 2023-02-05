#![allow(dead_code, unused_variables)]

use std::collections::{BTreeMap};

use itertools::{Itertools};

use crate::precompute::Precomputed;

mod defs;
mod precompute;
mod dice_set;

pub fn main() {
    let precomputed = Precomputed::default();
    let mut map = BTreeMap::new();
    for k in 6..=6 {
        for comb in dice_set::get_chars().iter().combinations_with_replacement(k) {
            let act_comb = comb.iter().map(|x| *x).join("");
            let dice = dice_set::from_string(&act_comb);
            let sc = precomputed.calc_score(dice);

            let mut has_smaller_subset = false;
            for c in act_comb.chars() {
                let subt = dice_set::subtract_dice(dice, &c.to_string());
                if precomputed.calc_score(subt) == sc { has_smaller_subset = true; }
            }
            if has_smaller_subset { continue; }

            if !map.contains_key(&sc) {
                map.insert(sc, Vec::new());
                map.get_mut(&sc).unwrap().push(dice_set::to_sorted_string(dice));
            }
        }
    }
    println!("{map:#?}");
}