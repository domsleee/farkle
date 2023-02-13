#![allow(dead_code, unused_variables)]

mod defs;
mod precompute;
mod dice_set;
mod farkle_solver;

use std::time::Instant;

use clap::{Parser, command, arg};
use defs::ScoreType;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, num_args = 2..)]
   scores: Vec<ScoreType>,

   #[arg(short = 'H', long, default_value_t = 0)]
   held_score: ScoreType,

   #[arg(short = 'd', long, default_value_t = defs::NUM_DICE)]
   dice_left: usize,
}

pub fn main() {
    bench_precompute();
    let args = Args::parse();
    let scores = if args.scores.len() == 0 { vec![0, 0] } else { args.scores };
    let mut solver = farkle_solver::FarkleSolver::default();
    dbg!(solver.decide_action_ext2(args.held_score, args.dice_left, scores));
    println!("cache size: {}", solver.get_cache_info());
}

fn bench_precompute() {
    let sw = Instant::now();
    let precomputed = precompute::Precomputed::default();
    let elapsed = sw.elapsed();
    println!("precomputed took {elapsed:?}");

    for dice_left in 0..=6 {
        let ok_rolls_len = precomputed.get_ok_rolls(dice_left).0.len();
        let ok_rolls_merged_len = precomputed.get_ok_rolls_merged(dice_left).0.len();

        println!("dice_left: {dice_left}: ok_rolls_len: {ok_rolls_len}, ok_rolls_merged_len: {ok_rolls_merged_len}");
    }

}
