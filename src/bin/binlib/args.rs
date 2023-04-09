use clap::{command, Parser, Subcommand};
use farkle::defs::ScoreType;

#[derive(Parser, Debug)]
/// Solver CLI for farkle game
#[command(author, version)]
pub struct MyArgs {
   #[arg(short, long, num_args = 2..)]
   pub scores: Vec<ScoreType>,

   #[arg(short = 'H', long, default_value_t = 0)]
   pub held_score: ScoreType,

   #[arg(short = 'd', long, default_value_t = farkle::defs::NUM_DICE)]
   pub dice_left: usize,

   #[arg(short = 'c', long)]
   pub cache_out: Option<String>,

   #[command(subcommand)]
   pub command: Option<Commands>
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Relaxation(RelaxationArgs),
    Simulate(SimulateArgs)
}

/// Run relaxation algorithm
#[derive(Parser, Debug)]
pub struct RelaxationArgs {
    #[arg(short, long, num_args = 2..)]
    pub scores: Vec<ScoreType>,

    /// The exact out (JSON file)
    #[arg(short, long)]
    pub exact_out: String
}

/// Run simulation
#[derive(Parser, Debug)]
pub struct SimulateArgs {
    #[arg(short, long, num_args = 2..)]
    pub scores: Option<Vec<ScoreType>>,
}
