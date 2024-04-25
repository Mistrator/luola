use crate::stat::{LevelScaling, Proficiency, Stat};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Deserialize, Serialize)]
pub struct Statistics {
    pub level: i32,

    pub current_hp: i32,
    pub max_hp: Stat,

    pub armor_class: Stat,
    pub fortitude_dc: Stat,
    pub reflex_dc: Stat,
    pub will_dc: Stat,

    pub melee_attack: Stat,
    pub ranged_attack: Stat,
    pub magic_attack: Stat,

    pub movement_speed: Stat,
    pub initiative: Stat,

    pub n_actions: Stat,

    pub inventory_slots: Stat,
}

impl Statistics {
    pub fn get_default() -> Self {
        Self {
            level: 1,

            current_hp: 0,
            max_hp: new_hp(Proficiency::Moderate),

            armor_class: new_defense(Proficiency::Moderate),
            fortitude_dc: new_defense(Proficiency::Moderate),
            reflex_dc: new_defense(Proficiency::Moderate),
            will_dc: new_defense(Proficiency::Moderate),

            melee_attack: new_attack(Proficiency::Moderate),
            ranged_attack: new_attack(Proficiency::Moderate),
            magic_attack: new_attack(Proficiency::Moderate),

            movement_speed: new_speed(Proficiency::Moderate),
            initiative: new_initiative(Proficiency::Moderate),

            inventory_slots: new_inventory(Proficiency::Moderate),

            n_actions: new_actions(Proficiency::Moderate),
        }
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = self.level;

        write!(f, "Level: {}\n", level)?;

        write!(
            f,
            "[Defenses] HP: {}/{}, AC: {}, Fort: {}, Reflex: {}, Will: {}\n",
            self.current_hp,
            self.max_hp.get_value(level),
            self.armor_class.get_value(level),
            self.fortitude_dc.get_value(level),
            self.reflex_dc.get_value(level),
            self.will_dc.get_value(level)
        )?;

        write!(
            f,
            "[Attacks] Melee: {}, Ranged: {}, Magic: {}\n",
            self.melee_attack.get_value(level),
            self.ranged_attack.get_value(level),
            self.magic_attack.get_value(level)
        )?;

        write!(
            f,
            "[Other] Speed: {}, Initiative: {}, Actions: {}, Inventory slots: {}\n",
            self.movement_speed.get_value(level),
            self.initiative.get_value(level),
            self.n_actions.get_value(level),
            self.inventory_slots.get_value(level)
        )?;

        Ok(())
    }
}

pub fn new_attack(prof: Proficiency) -> Stat {
    // Attacker advantage over defender. If this is 0, moderate attacks have
    // a 55% chance to hit against a moderate defense of the same level. Increasing
    // this increases the hit probability for all attacks.
    let attacker_advantage = 0;

    let prof_mod: i32 = match prof {
        Proficiency::Extreme => 4,
        Proficiency::High => 2,
        Proficiency::Moderate => 0,
        Proficiency::Low => -2,
        Proficiency::Terrible => -4,
    };

    let raw_value = attacker_advantage + prof_mod;
    let increase_per_level = 1;

    Stat::new(raw_value, LevelScaling::Linear(increase_per_level))
}

pub fn new_defense(prof: Proficiency) -> Stat {
    let prof_mod: i32 = match prof {
        Proficiency::Extreme => 4,
        Proficiency::High => 2,
        Proficiency::Moderate => 0,
        Proficiency::Low => -2,
        Proficiency::Terrible => -4,
    };

    let raw_value = 10 + prof_mod;
    let increase_per_level = 1;

    Stat::new(raw_value, LevelScaling::Linear(increase_per_level))
}

pub fn new_hp(prof: Proficiency) -> Stat {
    let raw_hp: i32 = match prof {
        Proficiency::Extreme => 16,
        Proficiency::High => 12,
        Proficiency::Moderate => 8,
        Proficiency::Low => 6,
        Proficiency::Terrible => 4,
    };

    // After leveling up this many times, our HP has doubled.
    let double_after_levels = 2.0;
    let increase_per_level: f64 = f64::powf(2.0, 1.0 / double_after_levels);

    Stat::new(raw_hp, LevelScaling::Exponential(increase_per_level))
}

pub fn new_initiative(prof: Proficiency) -> Stat {
    let raw_init = match prof {
        Proficiency::Extreme => 4,
        Proficiency::High => 2,
        Proficiency::Moderate => 0,
        Proficiency::Low => -2,
        Proficiency::Terrible => -4,
    };

    let increase_per_level = 1;

    Stat::new(raw_init, LevelScaling::Linear(increase_per_level))
}

pub fn new_speed(prof: Proficiency) -> Stat {
    let raw_speed = match prof {
        Proficiency::Extreme => 10,
        Proficiency::High => 7,
        Proficiency::Moderate => 5,
        Proficiency::Low => 4,
        Proficiency::Terrible => 2,
    };

    Stat::new(raw_speed, LevelScaling::NoScaling)
}

pub fn new_inventory(prof: Proficiency) -> Stat {
    let raw_inventory_slots = match prof {
        Proficiency::Extreme => 10,
        Proficiency::High => 6,
        Proficiency::Moderate => 4,
        Proficiency::Low => 2,
        Proficiency::Terrible => 1,
    };

    Stat::new(raw_inventory_slots, LevelScaling::NoScaling)
}

pub fn new_actions(prof: Proficiency) -> Stat {
    let raw_actions = match prof {
        Proficiency::Extreme => 5,
        Proficiency::High => 4,
        Proficiency::Moderate => 3,
        Proficiency::Low => 2,
        Proficiency::Terrible => 1,
    };

    Stat::new(raw_actions, LevelScaling::NoScaling)
}
