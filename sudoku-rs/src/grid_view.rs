use piston_window::types::Color;
use piston_window::*;
use solver::*;

/// Settings for grid view
pub struct GridViewSettings {
    /// Board size
    pub size: f64,

    /// Board background color
    pub bg_color: Color,

    /// Border color
    pub border_color: Color,

    /// Text color,
    pub text_color: Color,

    /// 3x3 block border radius
    pub block_border_radius: f64,

    /// Cell border radius
    pub cell_border_radius: f64,
}

impl GridViewSettings {
    /// Default grid view settings
    pub fn new() -> GridViewSettings {
        GridViewSettings {
            size: 512.0,
            bg_color: [0.1, 0.1, 0.1, 1.0],
            border_color: [6.0, 6.0, 6.0, 1.0],
            text_color: [1.0; 4],
            block_border_radius: 2.0,
            cell_border_radius: 1.0,
        }
    }
}

/// Grid drawing struct
pub struct GridView {
    settings: GridViewSettings,
}

impl GridView {
    /// Creates a new GridView with provided settings
    pub fn new(settings: GridViewSettings) -> GridView {
        GridView { settings }
    }

    /// Draw the grid
    pub fn draw(&self, grid: &Grid, glyphs: &mut Glyphs, c: &Context, g: &mut G2d) {
        let ref settings = self.settings;

        // board background
        let bg_rect = [0.0, 0.0, settings.size, settings.size];
        rectangle(settings.bg_color, bg_rect, c.transform, g);

        // cell borders
        for i in 0..9 {
            if (i % 3) == 0 {
                continue;
            }

            let x = i as f64 / 9.0 * settings.size;
            let y = i as f64 / 9.0 * settings.size;
            let x2 = settings.size;
            let y2 = settings.size;

            // vertical line
            line_from_to(
                settings.border_color,
                settings.cell_border_radius,
                [x, 0.0],
                [x, y2],
                c.transform,
                g,
            );

            // horizontal line
            line_from_to(
                settings.border_color,
                settings.cell_border_radius,
                [0.0, y],
                [x2, y],
                c.transform,
                g,
            );
        }

        // block borders
        // up to four for full board border
        for i in 0..4 {
            let x = i as f64 / 3.0 * settings.size;
            let y = i as f64 / 3.0 * settings.size;
            let x2 = settings.size;
            let y2 = settings.size;

            // vertical line
            line_from_to(
                settings.border_color,
                settings.block_border_radius,
                [x, 0.0],
                [x, y2],
                c.transform,
                g,
            );

            // horizontal line
            line_from_to(
                settings.border_color,
                settings.block_border_radius,
                [0.0, y],
                [x2, y],
                c.transform,
                g,
            );
        }

        // characters
        let text = Text::new_color(settings.text_color, 32);
        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                let ch = grid.get_value_str([i, j]);

                let pos = [
                    i as f64 * cell_size + 19.0,
                    j as f64 * cell_size + 39.0
                ];

                text.draw(
                    ch.as_str(),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans_pos(pos),
                    g,
                ).unwrap();
            }
        }
    }
}
