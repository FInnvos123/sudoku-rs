#![warn(missing_docs)]

//! Sudoku solver library
//!
//! # Example
//! ```
//! use solver::*;
//!
//! let sudoku = "\
//! ___26_7_1
//! 68__7__9_
//! 19___45__
//! 82_1___4_
//! __46_29__
//! _5___3_28
//! __93___74
//! _4__5__36
//! 7_3_18___";
//!
//! let mut grid = Grid::from_str(sudoku).unwrap();
//!
//! assert_eq!(solve(&mut grid), true);
//! ```

pub use crate::grid::Grid;
pub use crate::parse_error::ParseError;

pub mod grid;
pub mod parse_error;

/// Solves the provided sudoku grid.
///
/// Returns true when a solution was found,
/// false otherwise.
pub fn solve(grid: &mut Grid) -> bool {
    if let Some(cell) = grid.get_empty_cell() {
        for n in 1..=9 {
            if !grid.check_num(cell, n) {
                continue;
            }

            grid.set(cell, n);

            if solve(grid) {
                return true;
            }

            grid.unset(cell);
        }

        return false;
    }

    true
}

/// Struct to solve sudoku in steps
pub struct StepSolver {
    stack: Vec<[usize; 2]>,
}

impl StepSolver {
    /// Create new StepSolver
    pub fn new(grid: &Grid) -> StepSolver {
        let mut solver = StepSolver {
            stack: Vec::new(),
        };

        // initialize stack
        solver.stack.push(grid.get_empty_cell().unwrap());

        solver
    }

    /// Run one solving iteration on provided grid.
    ///
    /// Returns true when solution is found.
    pub fn solve_step(&mut self, grid: &mut Grid) -> bool {
        let cell_pos = *self.stack.last().unwrap();
        let mut n = 1;

        if let Some(val) = grid.get_value(cell_pos) {
            n = val + 1;
        }

        // TODO: allow arbitrary grid size
        for n in n..=9 {
            if !grid.check_num(cell_pos, n) {
                continue;
            }

            grid.set(cell_pos, n);

            if let Some(cell) = grid.get_empty_cell() {
                self.stack.push(cell);
                return false;
            }

            // no empty cell found, solution found
            return true;
        }

        // all values tried, empty cell and return to last
        grid.unset(cell_pos);
        self.stack.pop();
        false
    }
}
