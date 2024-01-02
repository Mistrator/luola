use crate::creature::statistics::Statistics;
use crate::grid::GridSquare;
use crate::world::Entity;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod creature_types;
pub mod perception;
pub mod statistics;

#[derive(Clone, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    pub stats: Statistics,

    id: u128,
    position: GridSquare,
}

impl Creature {
    pub fn new(name: String, position: GridSquare, stats: Statistics) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        Self {
            id: id,
            name: name,
            position: position,
            stats: stats,
        }
    }

    pub fn is_alive(&self) -> bool {
        true
    }
}

impl Entity for Creature {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_position(&self) -> GridSquare {
        self.position
    }

    fn set_position(&mut self, pos: &GridSquare) {
        self.position.y = pos.y;
        self.position.x = pos.x;
    }
}
