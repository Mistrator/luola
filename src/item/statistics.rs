use crate::stat::Stat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Statistics {
    pub rarity: Rarity,
    pub stats: HashMap<String, Stat>,

    level: i32,
}

impl Statistics {
    pub fn new(level: i32, rarity: Rarity) -> Self {
        Self {
            level: level,
            rarity: rarity,
            stats: HashMap::new(),
        }
    }

    pub fn get_raw_level(&self) -> i32 {
        self.level
    }

    pub fn get_level(&self) -> i32 {
        let modifier = match self.rarity {
            Rarity::Common => 0,
            Rarity::Uncommon => 1,
            Rarity::Rare => 2,
            Rarity::VeryRare => 3,
            Rarity::Legendary => 4,
        };

        self.level + modifier
    }
}
