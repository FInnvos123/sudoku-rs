//! Sudoku grid

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use crate::parse_error::ParseError;

const SIZE: usize = 9;

/// Grid cell
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell value
    pub value: u32,
}

impl Cell {
    /// Create new cell
    fn new(value: u32) -> Cell {
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
        let mut grid = Grid { cells: Vec::with_capacity(SIZE * SIZE) };

        for (line_nr, line) in Iterator::zip(1..=SIZE, reader.lines().take(SIZE)) {
            let line = line.ok().unwrap_or("".to_string());
            let line = line.trim_end();

            if line.chars().count() != SIZE {
                return Err(Box::new(ParseError::InvalidLineLength(line_nr, SIZE, line.chars().count())));
            }

            for ch in line.chars() {
                match ch {
                    '1'...'9' => grid.cells.push(Cell::new(ch.to_digit(10).unwrap())),
                    '_' => grid.cells.push(Cell::new(0)),
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
    pub fn get_num(&self, coords: [usize; 2]) -> &str {
        match self.cells[coords[0] + coords[1] * SIZE].value {
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => "0",
        }
    }
}
