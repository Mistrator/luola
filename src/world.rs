use crate::creature::Creature;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Wall,
}

#[derive(Debug)]
pub struct GridSquare {
    pub y: i32,
    pub x: i32,
}

// Upper left corner of a grid square.
pub struct GridIntersection {
    pub y: i32,
    pub x: i32,
}

pub trait Entity {
    fn get_id(&self) -> u128;

    fn get_position(&self) -> &GridSquare;
    fn set_position(&mut self, pos: &GridSquare);
}

pub struct World {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    grid: Vec<Vec<Tile>>,
    creatures: HashMap<u128, Creature>,
}

impl Layer {
    pub fn new(height: i32, width: i32) -> Self {
        if height <= 0 || width <= 0 {
            panic!("layer dimensions must be greater than zero");
        }

        Self {
            grid: vec![vec![Tile::Empty; width as usize]; height as usize],
            creatures: HashMap::new(),
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

    pub fn get_tile(&self, square: GridSquare) -> Option<&Tile> {
        if !self.valid_square(&square) {
            return None;
        }
        Some(&self.grid[square.y as usize][square.x as usize])
    }

    pub fn set_tile(&mut self, square: GridSquare, tile: Tile) {
        if !self.valid_square(&square) {
            panic!("out of bounds write to grid square {:?}", square);
        }
        self.grid[square.y as usize][square.x as usize] = tile;
    }

    pub fn add_creature(&mut self, creature: Creature) {
        self.creatures.insert(creature.get_id(), creature);
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                let square = GridSquare { y: i, x: j };
                let tile = self.get_tile(square).unwrap();

                match tile {
                    Tile::Empty => write!(f, "."),
                    _ => write!(f, "#"),
                }?;
            }
            write!(f, "\n")?;
        }

        for (_, creature) in &self.creatures {
            let pos = creature.get_position();
            write!(f, "{} ({}, {})\n", creature.name, pos.y, pos.x)?;
        }

        Ok(())
    }
}
