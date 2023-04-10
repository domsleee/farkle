use farkle::{farkle_serialiser, farkle_solver};

use crate::binlib::path_util;

use super::args::MyArgs;

pub fn run_approx(args: &MyArgs, approx_out: &String) -> Result<(), std::io::Error> {
    let scores = if args.scores.is_empty() {
        vec![0, 0]
    } else {
        args.scores.clone()
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

    farkle_serialiser::write_solver(&solver, &path_util::to_abs_path(&approx_out))?;
    Ok(())
}
