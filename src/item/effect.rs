use crate::item::targeting::Target;
use crate::world::Layer;
use rand::prelude::*;

pub enum Duration {
    Instantaneous,
    Temporary(i32),
    Permanent,
}

pub struct OngoingEffect {
    pub owner: u128,
    pub target: Target,
    pub remaining_duration: Duration,

    id: u128,
}

impl OngoingEffect {
    pub fn new(owner: u128, target: Target, duration: Duration) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen()

        Self {
            id: id,
            owner: owner,
            target: target,
            remaining_duration: duration,
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}

pub struct Effect {
    pub duration: Duration,

    pub apply: fn(effect: &Self, target: &Target, layer: &mut Layer) -> OngoingEffect,
    pub remove: Option<fn(effect: &ActiveEffect, target: &Target, layer: &mut Layer)>,
}
