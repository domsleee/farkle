#![allow(dead_code, unused_variables)]

use std::collections::{BTreeMap};

use itertools::{Itertools};

use crate::precompute::Precomputed;

mod defs;
mod precompute;
mod dice_set;
mod farkle_solver;

pub fn main() {
    let precomputed = Precomputed::default();
    // println!("{:?}", precomputed.get_ok_rolls(6));

    let mut solver = farkle_solver::FarkleSolver::default();
    dbg!(solver.decide_action_ext2(0, defs::NUM_DICE, vec![7000, 9950]));
}