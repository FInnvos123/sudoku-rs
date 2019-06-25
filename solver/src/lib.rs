#![warn(missing_docs)]

//! Sudoku solver library

pub use crate::grid::*;

pub mod grid;

/// Get sudoku grid
pub fn get_grid() -> Grid {
    Grid::new()
}
