//! Sudoku grid

use crate::parse_error::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const SIZE: usize = 9;

/// Grid cell
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell value
    pub value: Option<u32>,
}

impl Cell {
    /// Create new cell
    fn new(value: Option<u32>) -> Cell {
        Cell { value }
    }
}

/// Grid struct
pub struct Grid {
    /// Vector containing all grid cells
    pub cells: Vec<Cell>,
}

impl Grid {
    /// Create grid from file
    ///
    /// See crate docs for expected format
    pub fn from_file(path: &Path) -> Result<Grid, Box<dyn Error>> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        Grid::from_reader(buf_reader)
    }

    /// Load grid from str
    ///
    /// See crate docs for expected format
    pub fn from_str(s: &str) -> Result<Grid, Box<dyn Error>> {
        Grid::from_reader(s.as_bytes())
    }

    /// Load grid from reader
    ///
    /// See crate docs for expected format
    pub fn from_reader<T: BufRead>(reader: T) -> Result<Grid, Box<dyn Error>> {
        let mut grid = Grid {
            cells: Vec::with_capacity(SIZE * SIZE),
        };

        for (line_nr, line) in reader
            .lines()
            .flatten()
            .map(|s| s.trim_end().to_string())
            .take(SIZE)
            .enumerate()
        {
            let line_nr = line_nr + 1;

            if line.chars().count() != SIZE {
                return Err(Box::new(ParseError::InvalidLineLength(line_nr, SIZE, line.chars().count())));
            }

            for ch in line.chars() {
                match ch {
                    '1'...'9' => grid.cells.push(Cell::new(Some(ch.to_digit(10).unwrap()))),
                    '_' => grid.cells.push(Cell::new(None)),
                    _ => return Err(Box::new(ParseError::InvalidCharacter(line_nr, ch))),
                }
            }
        }

        if grid.cells.len() != SIZE * SIZE {
            return Err(Box::new(ParseError::InvalidRowCount(SIZE, grid.cells.len() / SIZE)));
        }

        Ok(grid)
    }

    /// Get number at specified location
    pub fn get_num(&self, coords: [usize; 2]) -> String {
        if let Some(n) = self.cells[coords[0] + coords[1] * SIZE].value {
            return n.to_string();
        }
        "".to_string()
    }
}
