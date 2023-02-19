use farkle::{farkle_solver, precompute, farkle_serialiser, defs::ScoreType};
use std::{time::Instant};
use clap::{Parser, command, arg};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, num_args = 2..)]
   scores: Vec<ScoreType>,

   #[arg(short = 'H', long, default_value_t = 0)]
   held_score: ScoreType,

   #[arg(short = 'd', long, default_value_t = farkle::defs::NUM_DICE)]
   dice_left: usize,

   #[arg(short = 'c', long)]
   cache_out: Option<String>,
}

pub fn main() {
    bench_precompute();
    let args = Args::parse();
    let scores = if args.scores.is_empty() { vec![0, 0] } else { args.scores };
    let mut solver = farkle_solver::FarkleSolver::default();
    dbg!(solver.decide_action_ext(args.held_score, args.dice_left, [scores[0], scores[1]]));
    let num_nodes = solver.get_mutable_data().nodes;
    println!("cache size: {}, num_nodes: {num_nodes}", solver.get_mutable_data().cache_decide_action.len());
    println!("dice_left -> num_nodes");
    for k in 1..=6 {
        println!("{k}: {}", solver.get_nodes_dice_left()[k]);
    }

    if args.cache_out.is_some() {
        farkle_serialiser::write_solver(&solver, args.cache_out.unwrap());
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
