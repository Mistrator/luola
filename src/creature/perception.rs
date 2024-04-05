use crate::ai::AI;
use crate::creature::Creature;
use crate::grid::{gridalgos, Grid, GridSquare};
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Awareness {
    Wander,
    Combat,
}

#[allow(dead_code)]
pub struct Observation {
    creature_id: u128,
    position: GridSquare,
    round: i64,
    direct: bool,
}

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

    pub fn seek(
        &self,
        _grid: &Grid,
        creatures: &HashMap<u128, Creature>,
        cur_round: i64,
    ) -> Vec<Observation> {
        let mut observations: Vec<Observation> = Vec::new();
        let owner_pos = creatures.get(&self.owner_id).unwrap().get_position();

        for (id, creature) in creatures {
            let pos: GridSquare = creature.get_position();

            // todo: get sense properties from creature stats
            // todo: make obstacles block senses
            let sensing_distance = 5;
            if gridalgos::distance(owner_pos, pos) <= sensing_distance {
                println!(
                    "creature {} noticed creature {} while seeking",
                    self.owner_id, *id
                );
                let obs = Observation {
                    creature_id: *id,
                    position: pos,
                    round: cur_round,
                    direct: true,
                };
                observations.push(obs);
            }
        }

        observations
    }

    pub fn update_observations(
        &mut self,
        grid: &Grid,
        creatures: &HashMap<u128, Creature>,
        cur_round: i64,
    ) {
        let obs_max_lifetime_rounds = 3;
        self.observations
            .retain(|x| cur_round - x.round <= obs_max_lifetime_rounds);

        let mut new_obs = self.seek(grid, creatures, cur_round);
        self.observations.append(&mut new_obs);
        // todo: notify nearby creatures of the observation
    }

    pub fn update_all_observations(
        creature_ai: &mut HashMap<u128, AI>,
        grid: &Grid,
        creatures: &HashMap<u128, Creature>,
        cur_round: i64,
    ) {
        for (_, c_ai) in creature_ai {
            c_ai.perception
                .update_observations(grid, creatures, cur_round);
        }
    }

    pub fn get_friendly_observations(&self) -> Vec<Observation> {
        panic!("unimplemented");
    }

    pub fn get_hostile_observations(&self) -> Vec<Observation> {
        panic!("unimplemented");
    }

    pub fn get_awareness(&self) -> Awareness {
        let hostile = self.get_hostile_observations();

        if !hostile.is_empty() {
            return Awareness::Combat;
        }
        Awareness::Wander
    }
}
