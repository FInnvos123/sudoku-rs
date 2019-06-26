use std::path::PathBuf;
use structopt::StructOpt;

/// Rust sudoku solver
#[derive(StructOpt, Debug)]
#[structopt(name="Sudoku-rs")]
pub struct Opt {
    /// Delay between steps when solving in milliseconds
    #[structopt(short = "d", long = "delay", default_value = "200.0")]
    pub delay: f64,

    /// File containing the sudoku
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}
