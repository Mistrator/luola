use crate::ai::AI;
use crate::creature::Creature;
use crate::grid::{Grid, GridSquare};
use std::collections::HashMap;
use std::fmt;

pub trait Entity {
    fn get_id(&self) -> u128;

    fn get_position(&self) -> GridSquare;
    fn set_position(&mut self, pos: &GridSquare);
}

pub struct World {
    pub layers: Vec<Layer>,
}

impl World {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

pub struct Layer {
    pub creatures: HashMap<u128, Creature>,
    pub creature_ai: HashMap<u128, AI>,
    pub grid: Grid,
}

impl Layer {
    pub fn new(height: i32, width: i32) -> Self {
        if height <= 0 || width <= 0 {
            panic!("layer dimensions must be greater than zero");
        }

        Self {
            grid: Grid::new(height, width),
            creatures: HashMap::new(),
            creature_ai: HashMap::new(),
        }
    }

    pub fn reconstruct(grid: Grid, creatures: HashMap<u128, Creature>) -> Self {
        Self {
            grid: grid,
            creatures: creatures,
            creature_ai: HashMap::new(),
        }
    }

    pub fn add_creature(&mut self, creature: Creature, c_ai: AI) {
        let id = creature.get_id();

        self.creatures.insert(id, creature);
        self.creature_ai.insert(id, c_ai);
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)?;

        for (_, creature) in &self.creatures {
            let pos = creature.get_position();
            write!(f, "{} ({}, {})\n", creature.name, pos.y, pos.x)?;
            write!(f, "{}", creature.stats)?;
        }

        Ok(())
    }
}
