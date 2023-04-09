use clap::Parser;
use farkle::{
    farkle_serialiser,
    farkle_solver::{self},
    precompute,
};
use std::time::Instant;

use crate::binlib::args::{Commands, MyArgs};

use super::subcommands::{relaxation::run_relaxation, simulate::run_simulate};

pub fn run() -> Result<(), std::io::Error> {
    let args = MyArgs::parse();
    bench_precompute();

    match &args.command {
        Some(Commands::Relaxation(relaxation_args)) => run_relaxation(&args, relaxation_args),
        Some(Commands::Simulate(simulate_args)) => run_simulate(&args, simulate_args)?,
        None => {
            let scores = if args.scores.is_empty() {
                vec![0, 0]
            } else {
                args.scores
            };
            let mut solver = farkle_solver::FarkleSolver::default();
            solver.farkle_solver_internal.is_approx = true;
            solver.make_suggested_reserves();
            dbg!(solver.decide_action_ext(args.held_score, args.dice_left, [scores[0], scores[1]]));
            let num_nodes = solver.get_mutable_data().nodes;
            println!(
                "cache size: {}, num_nodes: {num_nodes}",
                solver.get_mutable_data().cache_decide_action.len()
            );
            println!("dice_left -> num_nodes");
            for k in 1..=6 {
                println!("{k}: {}", solver.get_nodes_dice_left()[k]);
            }

            if args.cache_out.is_some() {
                farkle_serialiser::write_solver(&solver, &args.cache_out.unwrap());
            }
        }
    }
    Ok(())
}

fn bench_precompute() {
    let sw = Instant::now();
    let precomputed = precompute::Precomputed::default();
    let elapsed = sw.elapsed();
    println!("precomputed took {elapsed:?}");

    for dice_left in 1..=6 {
        let ok_rolls_len = precomputed.get_ok_rolls(dice_left).0.len();
        let ok_rolls_merged_len = precomputed.get_ok_rolls_merged(dice_left).0.len();

        println!("dice_left: {dice_left}: ok_rolls_len: {ok_rolls_len}, ok_rolls_merged_len: {ok_rolls_merged_len}");
    }
}
