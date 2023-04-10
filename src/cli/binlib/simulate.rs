use std::{io, path::PathBuf};

use crate::binlib::args::{MyArgs, SimulateArgs};
use farkle::{
    defs::{Action, ScoreType, NUM_DICE, SCORE_WIN},
    dice_set,
    farkle_serialiser::populate_solver_from_file,
    farkle_solver::{self, FarkleSolver},
    precompute::Precomputed,
};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use stopwatch::Stopwatch;

pub fn run_simulate(_args: &MyArgs, simulate_args: &SimulateArgs) -> Result<(), io::Error> {
    let solver_approx = get_solver_from_file(true, &PathBuf::from("pkg/approx.bincode"))?;
    let solver_exact = get_solver_from_file(false, &PathBuf::from("pkg/exact.bincode"))?;

    let num_games = simulate_args.games;
    let mut solvers = vec![solver_approx, solver_exact];

    let default_scores = if simulate_args.scores.is_some() {
        [
            simulate_args.scores.clone().unwrap()[0],
            simulate_args.scores.clone().unwrap()[1],
        ]
    } else {
        [0, 0]
    };
    let mut stopwatches = (0..=5).map(|_| Stopwatch::default()).collect_vec();

    let mut num_wins = [0, 0];
    let mut total_rolls = 0;
    let precompute = Precomputed::default();
    for first_turn in 0..=1 {
        for game_id in 0..num_games {
            //println!("running game {game_id}");
            let offset = 5000 * (game_id as u64);
            let mut game_scores: [ScoreType; 2] = default_scores;

            let mut turn_number = 0;

            while *game_scores.iter().max().unwrap() <= SCORE_WIN {
                stopwatches[4].start();
                let mut rand = rand::rngs::StdRng::seed_from_u64(offset + turn_number);
                let mut dice_left = 6;
                let player_id: usize = ((first_turn + turn_number) % 2) as usize;
                let solver = &mut solvers[player_id];
                let rotated_game_scores = if player_id == 0 {
                    game_scores
                } else {
                    let mut new_game_scores = game_scores;
                    new_game_scores.rotate_left(1);
                    new_game_scores
                };
                let mut held_score = 0;
                stopwatches[4].stop();

                loop {
                    total_rolls += 1;
                    stopwatches[1].start();
                    //println!("total_rolls: {total_rolls}");
                    let mut roll = dice_set::empty();
                    for _ in 0..dice_left {
                        let dice = rand.gen_range(1..=6);
                        roll = dice_set::combine_diceset(roll, dice_set::from_ind(dice));
                    }
                    stopwatches[1].stop();

                    stopwatches[2].start();
                    if precompute.get_valid_holds(roll).is_empty() {
                        // println!("invalid roll (:");
                        break;
                    }
                    stopwatches[2].stop();

                    stopwatches[0].start();
                    let (held_dice, mut action) =
                        decide_held_and_action(solver, held_score, roll, rotated_game_scores);
                    stopwatches[0].stop();

                    stopwatches[3].start();
                    held_score += precompute.calc_score(held_dice);

                    if game_scores[player_id] + held_score >= SCORE_WIN {
                        action = Action::Stay;
                        num_wins[player_id] += 1;
                    }

                    match action {
                        Action::Roll => {
                            dice_left -= precompute.get_num_dice(held_dice);
                            if dice_left == 0 {
                                dice_left = NUM_DICE;
                            }
                        }
                        Action::Stay => {
                            game_scores[player_id] += held_score;
                            break;
                        }
                    }
                    stopwatches[3].stop();
                }

                turn_number += 1;
            }
        }
    }

    for i in 0..=4 {
        println!("stopwatch {i} {}ms", stopwatches[i].elapsed_ms());
    }

    let total = num_wins.iter().sum();
    let percs = [get_perc(num_wins[0], total), get_perc(num_wins[1], total)];
    println!(
        "{num_wins:?} ({}, {}) (total_rolls: {total_rolls})",
        percs[0], percs[1]
    );
    Ok(())
}

pub fn get_solver_from_file(
    is_approx: bool,
    file: &PathBuf,
) -> Result<FarkleSolver<2>, std::io::Error> {
    let mut solver = FarkleSolver::<2>::default();
    if !file.exists() {
        panic!("specified file {:?} does not exist!", file);
    }
    solver.farkle_solver_internal.is_approx = is_approx;
    populate_solver_from_file(&mut solver, file)?;
    Ok(solver)
}

fn get_perc(a: u64, b: u64) -> String {
    format!("{:.2}%", ((100 * a) as f64) / (b as f64))
}

fn decide_held_and_action(
    solver: &mut farkle_solver::FarkleSolver<2>,
    held_score: ScoreType,
    roll: dice_set::DiceSet,
    scores: [ScoreType; 2],
) -> (dice_set::DiceSet, Action) {
    let (_, held_dice) = solver.decide_held_dice_ext(held_score, roll, scores);
    let precomputed = &solver.farkle_solver_internal.precomputed;
    let new_held_score = held_score + precomputed.calc_score(held_dice);
    let mut dice_left = precomputed.get_num_dice(roll) - precomputed.get_num_dice(held_dice);
    if dice_left == 0 {
        dice_left = 6;
    }
    let (_, action) = solver.decide_action_ext(new_held_score, dice_left, scores);
    (held_dice, action)
}
