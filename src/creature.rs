use crate::ai::Behavior;
use crate::world::{Entity, GridSquare};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod creature_types;

#[derive(Clone, Deserialize, Serialize)]
pub struct Statistics;

#[derive(Clone, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    pub stats: Statistics,

    id: u128,
    position: GridSquare,

    default_behavior: Behavior,
    override_behavior: Option<Behavior>,
}

impl Creature {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            id: rng.gen(),
            name: name,
            position: GridSquare { y: 0, x: 0 },
            stats: Statistics {},
            default_behavior: Behavior::Inactive,
            override_behavior: None,
        }
    }

    pub fn get_behavior(&self) -> Behavior {
        self.override_behavior.unwrap_or(self.default_behavior)
    }

    pub fn set_override_behavior(&mut self, behavior: Behavior) {
        self.override_behavior = Some(behavior)
    }

    pub fn restore_default_behavior(&mut self) {
        self.override_behavior = None
    }

    pub fn get_controlling_player_id(&self) -> Option<u128> {
        match self.get_behavior() {
            Behavior::PlayerControlled(player_id) => Some(player_id),
            _ => None,
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
