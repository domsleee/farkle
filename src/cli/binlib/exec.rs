use farkle::defs::NUM_DICE;

use super::{path_util, simulate::get_solver_from_file};

pub fn run_exec(cache_file: &String) -> Result<(), std::io::Error> {
    let mut solver = get_solver_from_file(false, &path_util::to_abs_path(&cache_file))?;
    let scores = [0, 0];
    let res = solver.decide_action_ext(0, NUM_DICE, scores);
    println!("{res:?}");
    Ok(())
}
