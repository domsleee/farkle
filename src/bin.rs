#![allow(dead_code, unused_variables)]

mod defs;
mod precompute;
mod dice_set;
mod farkle_solver;

use std::{time::Instant, io::BufWriter, fs::File};

use clap::{Parser, command, arg};
use defs::{ScoreType, CACHE_CUTOFF};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, num_args = 2..)]
   scores: Vec<ScoreType>,

   #[arg(short = 'H', long, default_value_t = 0)]
   held_score: ScoreType,

   #[arg(short = 'd', long, default_value_t = defs::NUM_DICE)]
   dice_left: usize,

   #[arg(short = 'c', long)]
   cache_out: Option<String>,
}

pub fn main() {
    bench_precompute();
    let args = Args::parse();
    let scores = if args.scores.is_empty() { vec![0, 0] } else { args.scores };
    let mut solver = farkle_solver::FarkleSolver::default();
    dbg!(solver.decide_action_ext2(args.held_score, args.dice_left, scores));
    println!("cache size: {}", solver.get_cache_info());

    if args.cache_out.is_some() {
        let path = args.cache_out.unwrap();
        println!("writing cache to file {path}");
        let mut f = BufWriter::new(File::create(path).unwrap());
        let mut cache = solver.get_cache_ref().clone();
        let keys = solver.get_cache_ref().keys().clone().collect_vec();
        for key in &keys {
            let (held_score, _, scores) = solver.unpack_cache_key(**key);
            if held_score != 0 {
                cache.remove(key);
            }
        }
        println!("writing {} keys, cache_cutoff {CACHE_CUTOFF} {}%...", cache.len(), 100.0 * (cache.len() as f64) / (solver.get_cache_ref().len() as f64));
        bincode::serialize_into(&mut f, &cache).unwrap();
        //serde_json::to_writer(&mut f, &cache).unwrap();
    }
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
