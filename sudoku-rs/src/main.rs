#![warn(missing_docs)]

//! Graphical sudoku solver using solver library

extern crate solver;
extern crate find_folder;
extern crate piston_window;
extern crate structopt;

pub use crate::grid_view::{GridView, GridViewSettings};
use crate::options::Opt;
use piston_window::*;
use solver::Grid;
use structopt::StructOpt;

mod grid_view;
mod options;

fn main() {
    let options = Opt::from_args();

    let mut window: PistonWindow = WindowSettings::new("sudoku-rs", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // initialize grid view
    let grid_view_settings = GridViewSettings::new();
    let grid_view = GridView::new(grid_view_settings);

    // load sudoku grid
    let grid = Grid::from_file(&options.file).unwrap_or_else(|err| {
        println!("{}", err);
        std::process::exit(1);
    });

    // load font
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
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
