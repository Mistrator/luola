use crate::creature::statistics::Statistics;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Outcome {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}

#[derive(Deserialize, Serialize)]
pub struct Check {
    pub roll: i32,
    pub modifier: i32,
    pub dc: i32,
    pub outcome: Outcome,
}

impl Check {
    pub fn new(roll: i32, modifier: i32, dc: i32, outcome: Outcome) -> Self {
        Self {
            roll,
            modifier,
            dc,
            outcome,
        }
    }
}

pub fn get_outcome(result: i32, dc: i32) -> Outcome {
    if result >= dc + 10 {
        return Outcome::CriticalSuccess;
    } else if result >= dc {
        return Outcome::Success;
    } else if result > dc - 10 {
        return Outcome::Failure;
    }
    return Outcome::CriticalFailure;
}

pub fn melee_attack_roll(att_stats: &Statistics, def_stats: &Statistics) -> Check {
    let roll = d20();

    let attack_bonus = att_stats.melee_attack.get_value(att_stats.level);
    let ac = def_stats.armor_class.get_value(def_stats.level);
    let outcome = get_outcome(roll + attack_bonus, ac);

    Check::new(roll, attack_bonus, ac, outcome)
}

pub fn d20() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(1..=20)
}
