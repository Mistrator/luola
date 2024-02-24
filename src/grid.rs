use serde::{Deserialize, Serialize};
use std::fmt;

pub mod gridalgos;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Serialize)]
pub struct GridSquare {
    pub y: i32,
    pub x: i32,
}

impl fmt::Display for GridSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.y, self.x)
    }
}

// Upper left corner of a grid square.
#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct GridIntersection {
    pub y: i32,
    pub x: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Grid {
    grid: Vec<Vec<Tile>>,
}

impl Grid {
    pub fn new(height: i32, width: i32) -> Self {
        if height <= 0 || width <= 0 {
            panic!("grid dimensions must be greater than zero");
        }

        Self {
            grid: vec![vec![Tile::Empty; width as usize]; height as usize],
        }
    }

    pub fn height(&self) -> i32 {
        self.grid.len() as i32
    }

    pub fn width(&self) -> i32 {
        self.grid[0].len() as i32
    }

    pub fn valid_square(&self, square: &GridSquare) -> bool {
        square.y >= 0 && square.y < self.height() && square.x >= 0 && square.x < self.width()
    }

    pub fn free_square(&self, square: &GridSquare) -> bool {
        if !self.valid_square(square) {
            return false;
        }

        if *self.get_tile(square).unwrap() != Tile::Empty {
            return false;
        }

        return true;
    }

    pub fn get_tile(&self, square: &GridSquare) -> Option<&Tile> {
        if !self.valid_square(&square) {
            return None;
        }
        Some(&self.grid[square.y as usize][square.x as usize])
    }

    pub fn set_tile(&mut self, square: GridSquare, tile: Tile) {
        if !self.valid_square(&square) {
            panic!("out of bounds write to grid square {}", square);
        }
        self.grid[square.y as usize][square.x as usize] = tile;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                let square = GridSquare { y: i, x: j };
                let tile = self.get_tile(&square).unwrap();

                match tile {
                    Tile::Empty => write!(f, "."),
                    _ => write!(f, "#"),
                }?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
