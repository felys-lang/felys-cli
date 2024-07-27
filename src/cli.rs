use std::path::PathBuf;
use clap::Parser;
use felys::Language;

#[derive(Parser)]
#[command(name = "felys")]
pub struct Cli {
    #[arg(
        short, long,
        help = "Target file that you want to run"
    )]
    pub file: Option<PathBuf>,

    #[arg(
        short, long,
        help = "Show the runtime and exit code"
    )]
    pub verbose: bool,

    #[arg(
        short, long,
        help = "Set an approximate timeout",
        default_value = "0"
    )]
    pub timeout: f64,

    #[arg(
        short, long,
        help = "Choose the program language",
        default_value = "en"
    )]
    pub lang: Language
}
