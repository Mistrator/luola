use crate::terminal::canvas::Canvas;
use crate::terminal::color::Color;
use crate::terminal::styled_char::Style;
use crate::ui::color_scheme;
use luola::creature::Creature;
use luola::grid::{Grid, GridSquare, Tile};
use luola::world::Layer;
use std::collections::HashMap;

const TILE_WIDTH: usize = 2 * TILE_HEIGHT;
const TILE_HEIGHT: usize = 2;
const MIN_MARGIN: i32 = 1;

pub struct Viewport {
    top_left: GridSquare,
    width_squares: usize,
    height_squares: usize,

    selection: GridSquare,
}

impl Viewport {
    pub fn new(width: usize, height: usize) -> Self {
        if width % TILE_WIDTH != 0 {
            panic!("viewport width must be a multiple of tile width");
        }

        if height % TILE_HEIGHT != 0 {
            panic!("viewport height must be a multiple of tile height");
        }

        let width_squares = width / TILE_WIDTH;
        let height_squares = height / TILE_HEIGHT;

        Self {
            top_left: GridSquare { y: 0, x: 0 },
            width_squares,
            height_squares,
            selection: GridSquare {
                y: (height_squares / 2) as i32,
                x: (width_squares / 2) as i32,
            },
        }
    }

    pub fn render(&self, layer: &Layer) -> Canvas {
        let mut grid = self.render_grid(&layer.grid);

        let creatures = self.render_creatures(&layer.creatures);
        grid.paste(&creatures, 0, 0);

        let selection = self.render_selection();
        let canvas_row = (self.selection.y as usize) * TILE_HEIGHT;
        let canvas_column = (self.selection.x as usize) * TILE_WIDTH;
        grid.paste(&selection, canvas_row, canvas_column);

        grid
    }

    pub fn move_selection(&mut self, delta_squares: GridSquare) {
        let new_y = self.selection.y + delta_squares.y;
        let new_x = self.selection.x + delta_squares.x;

        if new_y - MIN_MARGIN < 0 {
            self.top_left.y += delta_squares.y;
        } else if new_y + MIN_MARGIN >= (self.height_squares as i32) {
            self.top_left.y += delta_squares.y;
        } else {
            self.selection.y = new_y;
        }

        if new_x - MIN_MARGIN < 0 {
            self.top_left.x += delta_squares.x;
        } else if new_x + MIN_MARGIN >= (self.width_squares as i32) {
            self.top_left.x += delta_squares.x;
        } else {
            self.selection.x = new_x;
        }
    }

    pub fn get_selected_world_square(&self) -> GridSquare {
        self.viewport_to_world(self.selection)
    }

    fn width_chars(&self) -> usize {
        TILE_WIDTH * self.width_squares
    }

    fn height_chars(&self) -> usize {
        TILE_HEIGHT * self.height_squares
    }

    fn world_to_viewport(&self, world_square: GridSquare) -> Option<GridSquare> {
        let x = world_square.x - self.top_left.x;
        let y = world_square.y - self.top_left.y;

        if x < 0 || x >= self.width_squares as i32 {
            return None;
        }

        if y < 0 || y >= self.height_squares as i32 {
            return None;
        }

        Some(GridSquare { x, y })
    }

    fn viewport_to_world(&self, viewport_square: GridSquare) -> GridSquare {
        let x = viewport_square.x + self.top_left.x;
        let y = viewport_square.y + self.top_left.y;

        GridSquare { x, y }
    }

    fn render_grid(&self, grid: &Grid) -> Canvas {
        let mut canvas = Canvas::new(self.width_chars(), self.height_chars());

        for vp_y in 0..self.height_squares {
            for vp_x in 0..self.width_squares {
                let viewport_square = GridSquare {
                    x: vp_x as i32,
                    y: vp_y as i32,
                };

                let world_square = self.viewport_to_world(viewport_square);
                let tile = grid.get_tile(world_square);

                let rendered_tile = self.render_tile(tile);
                canvas.paste(&rendered_tile, vp_y * TILE_HEIGHT, vp_x * TILE_WIDTH);
            }
        }

        canvas
    }

    fn render_tile(&self, tile: Option<Tile>) -> Canvas {
        let mut canvas = Canvas::new(TILE_WIDTH, TILE_HEIGHT);

        let style = Style {
            foreground_color: Color::White,
            background_color: Color::Black,
        };

        if tile.is_none() {
            return canvas;
        }

        match tile.unwrap() {
            Tile::Wall => {
                for _ in 0..TILE_WIDTH * TILE_HEIGHT {
                    canvas.write(String::from("\u{2588}"), style);
                }
            }
            _ => (),
        }

        canvas
    }

    fn render_creatures(&self, creatures: &HashMap<u128, Creature>) -> Canvas {
        let mut canvas = Canvas::new_transparent(self.width_chars(), self.height_chars());

        for (_, creature) in creatures {
            let world_square = creature.get_position();
            let viewport_square = self.world_to_viewport(world_square);

            if viewport_square.is_none() {
                continue;
            }

            let viewport_square = viewport_square.unwrap();
            let rendered_creature = self.render_creature(&creature);

            let square_y = viewport_square.y as usize;
            let square_x = viewport_square.x as usize;

            canvas.paste(
                &rendered_creature,
                square_y * TILE_HEIGHT,
                square_x * TILE_WIDTH,
            );
        }

        canvas
    }

    fn render_creature(&self, _creature: &Creature) -> Canvas {
        let mut canvas = Canvas::new_transparent(TILE_WIDTH, TILE_HEIGHT);

        let style = Style {
            foreground_color: Color::Red,
            background_color: Color::Transparent,
        };

        canvas.set_cursor_position(1, 1);

        // "black large circle"
        // If the space after is removed, the circle is drawn only partially
        canvas.write(String::from("\u{2b24} "), style);

        canvas
    }

    fn render_selection(&self) -> Canvas {
        let mut canvas = Canvas::new_transparent(TILE_WIDTH, TILE_HEIGHT);

        // rounded corners
        canvas.write(String::from("\u{256d}"), color_scheme::SELECTION_STYLE);

        canvas.set_cursor_position(0, TILE_WIDTH - 1);
        canvas.write(String::from("\u{256e}"), color_scheme::SELECTION_STYLE);

        canvas.set_cursor_position(TILE_HEIGHT - 1, 0);
        canvas.write(String::from("\u{2570}"), color_scheme::SELECTION_STYLE);

        canvas.set_cursor_position(TILE_HEIGHT - 1, TILE_WIDTH - 1);
        canvas.write(String::from("\u{256f}"), color_scheme::SELECTION_STYLE);

        canvas
    }
}
