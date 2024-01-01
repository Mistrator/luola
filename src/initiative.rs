use crate::ai::AI;
use crate::creature::perception::Awareness;
use crate::dice;
use crate::world::Layer;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Serialize)]
pub struct Initiative {
    order: Vec<(i32, u128)>,
}

impl Initiative {
    fn roll_for_creature(creature_id: u128) -> (i32, u128) {
        // todo: get initiative bonus from creature stats
        let init_bonus: i32 = 0;
        let init_value = dice::d20() + init_bonus;

        (init_value, creature_id)
    }

    pub fn roll_initiative(layer: &Layer) -> Self {
        let mut init = Self { order: Vec::new() };

        for (id, _) in &layer.creatures {
            let creature_init = Self::roll_for_creature(*id);
            init.order.push(creature_init);
        }

        // Two creatures with equal initiative should both have an equal chance of
        // going first. Thus, we first shuffle and then stable sort.
        let mut rng = thread_rng();
        init.order.shuffle(&mut rng);
        init.order.sort_by(|a, b| a.0.cmp(&b.0));
        init.order.reverse();

        init
    }

    // Insert a new creature into initiative order. If there are ties, choose any
    // valid position with equal probability.
    #[allow(dead_code)]
    fn insert(&mut self, creature_init: (i32, u128)) {
        let mut first_i: i32 = 0;
        let mut last_i: i32 = 0;
        let tgt = creature_init.0;

        for i in 0..self.order.len() {
            if i > 0 {
                assert!(
                    self.order[i - 1].0 >= self.order[i].0,
                    "initiative order must be sorted in non-increasing order by initiative"
                );
            }
            if self.order[i].0 > tgt {
                first_i += 1;
            }
            if self.order[i].0 >= tgt {
                last_i += 1;
            }
        }

        let mut rng = thread_rng();
        let ind: usize = rng.gen_range((first_i as usize)..=(last_i as usize));
        self.order.insert(ind, creature_init);
    }

    pub fn remove(&mut self, creature_id: u128) {
        self.order.retain(|x| x.1 != creature_id);
    }

    pub fn get_aware(&self, creature_ai: &HashMap<u128, AI>) -> Vec<(i32, u128)> {
        let mut aware: Vec<(i32, u128)> = Vec::new();

        for (init, id) in &self.order {
            let c_ai = creature_ai
                .get(&id)
                .expect("initiative should not contain nonexistent creatures");

            if c_ai.is_player_controlled() || c_ai.perception.get_awareness() == Awareness::Combat {
                aware.push((*init, *id));
            }
        }

        aware
    }

    pub fn get_wandering(&self, creature_ai: &HashMap<u128, AI>) -> Vec<(i32, u128)> {
        let mut wander: Vec<(i32, u128)> = Vec::new();

        for (init, id) in &self.order {
            let c_ai = creature_ai
                .get(&id)
                .expect("initiative should not contain nonexistent creatures");

            if !c_ai.is_player_controlled() && c_ai.perception.get_awareness() == Awareness::Wander
            {
                wander.push((*init, *id));
            }
        }

        wander
    }
}
