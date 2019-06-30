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

    /// Get cell value at specified location
    pub fn get_value(&self, coords: [usize; 2]) -> Option<u32> {
        self.cells[coords[0] + coords[1] * SIZE].value
    }

    /// Get cell value at specified location as str
    pub fn get_value_str(&self, coords: [usize; 2]) -> String {
        if let Some(n) = self.cells[coords[0] + coords[1] * SIZE].value {
            return n.to_string();
        }
        "".to_string()
    }

    /// Get first empty cell, starting from top left
    pub fn get_empty_cell(&self) -> Option<[usize; 2]> {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.cells[x + y * SIZE].value == None {
                    return Some([x, y]);
                }
            }
        }
        None
    }

    /// Check if number is valid for the provided cell
    pub fn check_num(&self, cell: [usize; 2], n: u32) -> bool {
        for c in self.get_col(cell[0]) {
            if c.value == Some(n) {
                return false;
            }
        }

        for r in self.get_row(cell[1]) {
            if r.value == Some(n) {
                return false;
            }
        }

        true
    }

    /// Get grid column
    pub fn get_col(&self, col: usize) -> Vec<Cell> {
        let mut c = Vec::with_capacity(SIZE);
        for y in 0..SIZE {
            c.push(self.cells[col + y * SIZE]);
        }
        c
    }

    /// Get grid row
    pub fn get_row(&self, row: usize) -> Vec<Cell> {
        let mut c = Vec::with_capacity(SIZE);
        for x in 0..SIZE {
            c.push(self.cells[x + row * SIZE]);
        }
        c
    }

    /// Set cell value
    pub fn set(&mut self, cell: [usize; 2], n: u32) {
        self.cells[cell[0] + cell[1] * SIZE].value = Some(n);
    }

    /// Unset cell value
    pub fn unset(&mut self, cell: [usize; 2]) {
        self.cells[cell[0] + cell[1] * SIZE].value = None;
    }
}
