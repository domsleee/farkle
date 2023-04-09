use farkle::{farkle_solver::{PrevDecideActionCache, FarkleSolverInternal, self}, defs::{NUM_DICE, ScoreType, get_val}, farkle_serialiser};

use crate::binlib::args::{MyArgs, RelaxationArgs};

pub fn run_relaxation(_args: &MyArgs, relaxation_args: &RelaxationArgs) {
    let mut cache_prev_run = PrevDecideActionCache::default();

    let dice_left = NUM_DICE;
    let scores = if relaxation_args.scores.is_empty() { vec![0, 0] } else { relaxation_args.scores.to_owned() };
    let mut all_keys: Vec<(ScoreType, usize, [ScoreType; 2])> = Vec::new();
    for held_score in (0..=10000).step_by(50) {
        for score1 in (scores[0]..=10000).step_by(50) {
            for score2 in (scores[1]..=10000).step_by(50) {
                all_keys.push((held_score, dice_left, [score1, score2]));
                let cache_key = FarkleSolverInternal::get_cache_key(held_score, dice_left, &[score1, score2]);
                cache_prev_run.insert(cache_key, get_val(1) / get_val(2));
            }
        }
    }
    
    let mut delta = 1.00;
    let mut last_delta = -1.00;

    let mut solver = farkle_solver::FarkleSolver::default();

    for it in 1..=i32::MAX {
        if delta == last_delta {
            break;
        }
        last_delta = delta;

        println!("it {it}, delta: {delta}");
        solver = farkle_solver::FarkleSolver::default();
        solver.farkle_solver_internal.cache_previous_run = cache_prev_run.clone();
        
        let prev_cache_prev_run = cache_prev_run.clone();
        for (held_score, dice_left, scores) in &all_keys {
            let cache_key = FarkleSolverInternal::get_cache_key(*held_score, *dice_left, scores);
            cache_prev_run.insert(cache_key, solver.decide_action_ext(*held_score, *dice_left, *scores).0);
        }
        delta = 0.00f64;
        for (held_score, dice_left, scores) in &all_keys {
            let cache_key = FarkleSolverInternal::get_cache_key(*held_score, *dice_left, scores);
            let this_delta = f64::abs(cache_prev_run.get(&cache_key).unwrap() - prev_cache_prev_run.get(&cache_key).unwrap());
            delta = f64::max(delta, this_delta);
        }
    }

    farkle_serialiser::write_solver(&solver, &relaxation_args.exact_out);

}
