#![warn(missing_docs)]

//! Sudoku solver library
//!
//! # Example
//! ```
//! use solver::Grid;
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
//! let grid = Grid::from_str(sudoku).unwrap();
//! ```

pub use crate::grid::Grid;
pub use crate::parse_error::ParseError;

pub mod grid;
pub mod parse_error;
