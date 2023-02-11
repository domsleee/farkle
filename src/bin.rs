#![allow(dead_code, unused_variables)]

mod defs;
mod precompute;
mod dice_set;
mod farkle_solver;

use clap::{Parser, command, arg};
use defs::ScoreType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value_t = 0)]
   score1: ScoreType,

   #[arg(short = 't', long, default_value_t = 0)]
   score2: ScoreType,
}

pub fn main() {
    let args = Args::parse();

    let mut solver = farkle_solver::FarkleSolver::default();
    dbg!(solver.decide_action_ext2(0, defs::NUM_DICE, vec![args.score1, args.score2]));
}