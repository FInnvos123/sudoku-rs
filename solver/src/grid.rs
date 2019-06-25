//! Sudoku grid

const SIZE: usize = 9;

/// Grid cell
#[derive(Copy, Clone)]
pub struct Cell {
    /// Cell value
    pub value: u32,
}

impl Cell {
    /// Create new empty cell
    fn new() -> Cell {
        Cell { value: 0 }
    }
}

/// Grid struct
pub struct Grid {
    /// 2D array containing all grid cells
    pub cells: [[Cell;SIZE];SIZE]
}

impl Grid {
    /// Create and initialize new grid
    pub fn new() -> Grid {
        Grid {
            cells: [[Cell::new();SIZE];SIZE]
        }
    }

    /// Get number at specified location
    pub fn get_num(&self, coords: [usize; 2]) -> &str {
        match self.cells[coords[1]][coords[0]].value {
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
