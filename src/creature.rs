use crate::creature::inventory::Inventory;
use crate::creature::statistics::Statistics;
use crate::grid::GridSquare;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod creature_types;
pub mod inventory;
pub mod perception;
pub mod statistics;

#[derive(Clone, Deserialize, Serialize)]
pub struct Creature {
    pub name: String,
    pub stats: Statistics,
    pub inventory: Inventory,

    id: u128,
    position: GridSquare,
}

impl Creature {
    pub fn new(name: String, position: GridSquare, mut stats: Statistics) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        let inventory_slots = stats.inventory_slots.get_value(stats.level) as usize;

        stats.current_hp = stats.max_hp.get_value(stats.level);

        Self {
            id: id,
            name: name,
            position: position,
            stats: stats,
            inventory: Inventory::new(inventory_slots),
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }

    pub fn get_position(&self) -> GridSquare {
        self.position
    }

    pub fn set_position(&mut self, pos: &GridSquare) {
        self.position.y = pos.y;
        self.position.x = pos.x;
    }

    pub fn change_hp(&mut self, amount: i32) {
        self.stats.current_hp += amount;

        let max_hp = self.stats.max_hp.get_value(self.stats.level);

        if self.stats.current_hp < 0 {
            self.stats.current_hp = 0;
        } else if self.stats.current_hp > max_hp {
            self.stats.current_hp = max_hp;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.stats.current_hp > 0
    }
}
