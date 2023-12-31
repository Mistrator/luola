use crate::world::{GridSquare, Layer};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub enum Awareness {
    Wander,
    Combat,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Observation {
    creature_id: u128,
    position: GridSquare,
    round: i64,
    direct: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Perception {
    observations: Vec<Observation>,
    owner_id: u128,
}

impl Perception {
    pub fn new(owner_id: u128) -> Self {
        Self {
            observations: Vec::new(),
            owner_id: owner_id,
        }
    }

    pub fn update_observations(&mut self, _cur_round: i64) {
        // todo: drop observations that are too old
        //       - this may cause a creature in combat to become wandering again
        // todo: seek environment and make new observations
        // todo: inform nearby creatures as well
    }

    pub fn update_observations_globally(layer: &mut Layer, cur_round: i64) {
        for (_, creature) in &mut layer.creatures {
            creature.perception.update_observations(cur_round);
        }
    }

    pub fn seek(_layer: &Layer) -> Vec<Observation> {
        let observations: Vec<Observation> = Vec::new();
        observations
    }

    pub fn get_friendly_observations(&self) -> Vec<Observation> {
        Vec::new()
    }

    pub fn get_hostile_observations(&self) -> Vec<Observation> {
        Vec::new()
    }

    pub fn get_awareness(&self) -> Awareness {
        let hostile = self.get_hostile_observations();

        if !hostile.is_empty() {
            return Awareness::Combat;
        }
        Awareness::Wander
    }
}
