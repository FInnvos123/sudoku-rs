#![warn(missing_docs)]

//! Graphical sudoku solver using solver library

extern crate piston_window;
extern crate find_folder;
extern crate solver;

use piston_window::*;
use solver::Grid;
pub use crate::grid_view::*;

mod grid_view;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("sudoku-rs", [512; 2])
            .exit_on_esc(true)
            .build().unwrap();

    // initialize grid view
    let grid_view_settings = GridViewSettings::new();
    let grid_view = GridView::new(grid_view_settings);

    // load sudoku grid
    let grid_file = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("sudokus").unwrap().join("1");
    let grid = Grid::from_file(&grid_file).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    // load font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    // render loop
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, d| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            grid_view.draw(&grid, &mut glyphs, &c, g);

            glyphs.factory.encoder.flush(d);
        });
    }
}
