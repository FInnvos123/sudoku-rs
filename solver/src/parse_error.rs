//! Sudoku parsing errors

use std::error::Error;
use std::fmt;

/// Error while parsing the sudoku
#[derive(Debug)]
pub enum ParseError {
    /// Line length didn't match the expected line length
    ///
    /// InvalidLineLength(line nr, expected, actual)
    InvalidLineLength(usize, usize, usize),

    /// Row count didn't match the expected amount of rows
    ///
    /// InvalidRowCount(expected, actual)
    InvalidRowCount(usize, usize),

    /// Invalid character encountered
    ///
    /// InvalidCharacter(line nr, character)
    InvalidCharacter(usize, char),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;

        match *self {
            InvalidLineLength(line_nr, expected, actual) => write!(
                f,
                "Line {} didn't match expected line length. Expected: {}, Actual: {}",
                line_nr, expected, actual
            ),
            InvalidRowCount(expected, actual) => write!(
                f,
                "Amount of rows didn't match expected amount. Expected: {}, Actual: {}",
                expected, actual
            ),
            InvalidCharacter(line_nr, ch) => {
                write!(f, "Line {} contains an invalid character '{}'", line_nr, ch)
            }
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
