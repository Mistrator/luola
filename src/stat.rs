use crate::constants;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
pub enum LevelScaling {
    NoScaling,
    Linear(i32),
    Exponential(f64),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum Proficiency {
    Extreme,
    High,
    Moderate,
    Low,
    Terrible,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Stat {
    raw_value: i32,
    level_scaling: LevelScaling,

    additive_modifiers: HashMap<u128, i32>,
    multiplicative_modifiers: HashMap<u128, f64>,
}

impl Stat {
    pub fn new(raw_value: i32, scaling: LevelScaling) -> Self {
        Self {
            raw_value: raw_value,
            additive_modifiers: HashMap::new(),
            multiplicative_modifiers: HashMap::new(),
            level_scaling: scaling,
        }
    }

    pub fn get_value(&self, level: i32) -> i32 {
        let mut val: f64 = self.raw_value as f64;

        val = self.scale_by_level(val, level);

        val += self.get_total_additive_modifier() as f64;
        val *= self.get_total_multiplicative_modifier();

        val as i32
    }

    pub fn get_raw_value(&self) -> i32 {
        self.raw_value
    }

    pub fn get_value_without_modifiers(&self, level: i32) -> i32 {
        let val: f64 = self.raw_value as f64;

        self.scale_by_level(val, level) as i32
    }

    fn scale_by_level(&self, val: f64, level: i32) -> f64 {
        let mut scaled_val = val;

        match self.level_scaling {
            LevelScaling::Linear(increment) => {
                scaled_val += (level * increment) as f64;
            }
            LevelScaling::Exponential(base) => {
                // Our level scale starts from constants::MIN_LEVEL instead of 1,
                // so shift levels so that MIN_LEVEL is mapped to level 1.
                let shift: i32 = 1 - constants::MIN_LEVEL;
                let shifted_level = level + shift;

                scaled_val *= base.powi(shifted_level);
            }
            LevelScaling::NoScaling => (),
        }

        scaled_val
    }

    pub fn get_total_additive_modifier(&self) -> i32 {
        let mut additive_mod: i32 = 0;

        for (_, m) in &self.additive_modifiers {
            additive_mod += m;
        }

        additive_mod
    }

    pub fn get_total_multiplicative_modifier(&self) -> f64 {
        let mut multiplicative_mod: f64 = 1.0;

        for (_, m) in &self.multiplicative_modifiers {
            multiplicative_mod *= m;
        }

        multiplicative_mod
    }

    pub fn is_modified(&self) -> bool {
        self.get_total_additive_modifier() != 0 || self.get_total_multiplicative_modifier() != 1.0
    }

    pub fn apply_additive_modifier(&mut self, modifier_id: u128, modifier: i32) {
        self.additive_modifiers.insert(modifier_id, modifier);
    }

    pub fn apply_multiplicative_modifier(&mut self, modifier_id: u128, modifier: f64) {
        self.multiplicative_modifiers.insert(modifier_id, modifier);
    }

    pub fn remove_modifier(&mut self, modifier_id: u128) {
        self.additive_modifiers.remove(&modifier_id);
        self.multiplicative_modifiers.remove(&modifier_id);
    }
}
