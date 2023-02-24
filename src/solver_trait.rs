// use crate::defs::ScoreType;


// trait Solver<PLAYERS> {
//     pub fn decide_action_ext(&mut self, held_score: ScoreType, dice_left: usize, scores: [ScoreType; PLAYERS]) -> (ProbType, Action) {
//     }

//     pub fn decide_held_dice_ext(&mut self, held_score: ScoreType, roll: dice_set::DiceSet, scores: [ScoreType; PLAYERS]) -> (ProbType, dice_set::DiceSet) {
//         self.farkle_solver_internal.decide_held_dice(&mut self.mutable_data, held_score, roll, &scores)
//     }
// }