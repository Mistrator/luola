use crate::world::{Entity, GridSquare};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod creature_types;

#[derive(Clone, Deserialize, Serialize)]
pub struct Statistics;

#[derive(Clone, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    pub stats: Statistics,

    id: u128,
    position: GridSquare,
}

impl Creature {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            id: rng.gen(),
            name: name,
            position: GridSquare { y: 0, x: 0 },
            stats: Statistics {},
        }
    }
}

impl Entity for Creature {
    fn get_id(&self) -> u128 {
        self.id
    }

    fn get_position(&self) -> &GridSquare {
        &self.position
    }

    fn set_position(&mut self, pos: &GridSquare) {
        self.position.y = pos.y;
        self.position.x = pos.x;
    }
}
