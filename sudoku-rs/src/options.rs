use std::path::PathBuf;
use structopt::StructOpt;

/// Rust sudoku solver
#[derive(StructOpt, Debug)]
#[structopt(name="Sudoku-rs")]
pub struct Opt {
    /// Updates per second
    #[structopt(short = "u", long = "ups", default_value = "120")]
    pub ups: u64,

    /// File containing the sudoku
    #[structopt(name = "FILE", parse(from_os_str))]
    pub file: PathBuf,
}
