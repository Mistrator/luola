use crate::item::statistics::Statistics;
use crate::item::targeting::TargetKind;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub mod effect;
pub mod item_types;
pub mod statistics;
pub mod targeting;

#[derive(Clone, Deserialize, Serialize)]
pub enum ItemKind {
    Active(TargetKind),
    Passive,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub kind: ItemKind,
    pub stats: Statistics,

    id: u128,
}

impl Item {
    pub fn new(name: String, description: String, kind: ItemKind, stats: Statistics) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        Self {
            id,
            name,
            description,
            kind,
            stats,
        }
    }
    pub fn get_id(&self) -> u128 {
        self.id
    }
}
