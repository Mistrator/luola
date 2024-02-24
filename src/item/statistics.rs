use crate::stat::{LevelScaling, Proficiency, Stat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Copy, Clone, Deserialize, Serialize)]
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
    pub values: HashMap<String, Stat>,

    level: i32,
}

impl Statistics {
    pub fn new(level: i32, rarity: Rarity) -> Self {
        Self {
            level: level,
            rarity: rarity,
            values: HashMap::new(),
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

pub fn new_single_target_damage(prof: Proficiency) -> Stat {
    let raw_damage: i32 = match prof {
        Proficiency::Extreme => 8,
        Proficiency::High => 6,
        Proficiency::Moderate => 4,
        Proficiency::Low => 3,
        Proficiency::Terrible => 2,
    };

    // After leveling up this many times, our damage has doubled.
    let double_after_levels = 2.0;
    let increase_per_level: f64 = f64::powf(2.0, 1.0 / double_after_levels);

    Stat::new(raw_damage, LevelScaling::Exponential(increase_per_level))
}

pub fn new_area_damage(prof: Proficiency) -> Stat {
    let raw_damage: i32 = match prof {
        Proficiency::Extreme => 6,
        Proficiency::High => 4,
        Proficiency::Moderate => 3,
        Proficiency::Low => 2,
        Proficiency::Terrible => 1,
    };

    // After leveling up this many times, our damage has doubled.
    let double_after_levels = 2.0;
    let increase_per_level: f64 = f64::powf(2.0, 1.0 / double_after_levels);

    Stat::new(raw_damage, LevelScaling::Exponential(increase_per_level))
}
