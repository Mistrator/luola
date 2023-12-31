use crate::ai::Behavior;
use crate::creature::perception::Perception;
use crate::world::{Entity, GridSquare};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod creature_types;
pub mod perception;

#[derive(Clone, Deserialize, Serialize)]
pub struct Statistics;

#[derive(Clone, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    pub stats: Statistics,
    pub perception: Perception,

    id: u128,
    position: GridSquare,

    default_wander_behavior: Behavior,
    default_combat_behavior: Behavior,
    override_behavior: Option<Behavior>,
}

impl Creature {
    pub fn new(name: String) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        Self {
            id: id,
            name: name,
            perception: Perception::new(id),
            position: GridSquare { y: 0, x: 0 },
            stats: Statistics {},
            default_wander_behavior: Behavior::Inactive,
            default_combat_behavior: Behavior::Inactive,
            override_behavior: None,
        }
    }

    pub fn get_wander_behavior(&self) -> Behavior {
        self.override_behavior
            .unwrap_or(self.default_wander_behavior)
    }

    pub fn get_combat_behavior(&self) -> Behavior {
        self.override_behavior
            .unwrap_or(self.default_combat_behavior)
    }

    pub fn set_override_behavior(&mut self, behavior: Behavior) {
        self.override_behavior = Some(behavior)
    }

    pub fn restore_default_behavior(&mut self) {
        self.override_behavior = None
    }

    pub fn get_controlling_player_id(&self) -> Option<u128> {
        match self.get_combat_behavior() {
            Behavior::PlayerControlled(player_id) => Some(player_id),
            _ => None,
        }
    }

    pub fn is_player_controlled(&self) -> bool {
        self.get_controlling_player_id().is_some()
    }

    pub fn is_alive(&self) -> bool {
        true
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
