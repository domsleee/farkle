use clap::Parser;
use farkle::precompute;
use std::time::Instant;

use crate::binlib::args::{Commands, MyArgs};

use super::{approx::run_approx, relax::run_relaxation, simulate::run_simulate};

pub fn run() -> Result<(), std::io::Error> {
    let args = MyArgs::parse();
    bench_precompute();

    match &args.command {
        Commands::Relax(relaxation_args) => run_relaxation(&args, relaxation_args)?,
        Commands::Simulate(simulate_args) => run_simulate(&args, simulate_args)?,
        Commands::Approx { approx_out } => run_approx(&args, approx_out)?,
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
