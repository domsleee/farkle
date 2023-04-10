use clap::{command, Parser, Subcommand};
use farkle::defs::ScoreType;

#[derive(Parser, Debug)]
/// Solver CLI for farkle game
#[clap(author, version, subcommand_required = true)]
pub struct MyArgs {
    #[arg(short = 'H', long, default_value_t = 0)]
    pub held_score: ScoreType,

    #[arg(short = 'd', long, default_value_t = farkle::defs::NUM_DICE)]
    pub dice_left: usize,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run approximation (1 iteration), and output the result to a bincode file
    Approx {
        #[arg(long, default_value = "./pkg/approx.bincode")]
        approx_out: String,
    },
    /// Run one execution
    Exec {
        #[arg(short, long, num_args = 2..)]
        scores: Vec<ScoreType>,

        #[arg(long, default_value = "./pkg/exact.bincode")]
        cache_to_use: String,
    },
    Relax(RelaxationArgs),
    Simulate(SimulateArgs),
}

/// Run relaxation algorithm, and output the result to a bincode file
#[derive(Parser, Debug)]
pub struct RelaxationArgs {
    #[arg(short, long, num_args = 2..)]
    pub scores: Vec<ScoreType>,

    /// The exact out (JSON file)
    #[arg(long, default_value = "./pkg/exact.bincode")]
    pub exact_out: String,
}

/// Run simulation to compare relaxation + approximation. Must run `approx` and `relax` first
#[derive(Parser, Debug)]
pub struct SimulateArgs {
    #[arg(short, long, num_args = 2..)]
    pub scores: Option<Vec<ScoreType>>,

    #[arg(long, default_value = "500")]
    pub games: usize,
}
