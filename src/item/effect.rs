use crate::item::statistics::Statistics;
use crate::item::targeting::Target;
use crate::world::Layer;
use rand::prelude::*;

pub enum Duration {
    Instantaneous,
    Temporary(i32),
    Permanent,
}

pub struct OngoingEffect {
    pub effect: u128,
    pub owner: u128,
    pub target: Target,
    pub remaining_duration: Duration,

    id: u128,
}

impl OngoingEffect {
    pub fn new(effect: u128, owner: u128, target: Target, duration: Duration) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        Self {
            effect,
            id,
            owner,
            target,
            remaining_duration: duration,
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}

pub struct Effect {
    pub duration: Duration,
    pub stats: Statistics,

    #[rustfmt::skip]
    pub apply: fn(effect: u128, owner: u128, target: Target, layer: &mut Layer) -> Option<OngoingEffect>,
    pub tick: Option<fn(effect: u128, owner: u128, target: &Target, layer: &mut Layer)>,
    pub remove: Option<fn(effect: u128, owner: u128, target: &Target, layer: &mut Layer)>,

    id: u128,
}

impl Effect {
    #[rustfmt::skip]
    pub fn new(
        duration: Duration,
        stats: Statistics,

        apply: fn(effect: u128, owner: u128, target: Target, layer: &mut Layer) -> Option<OngoingEffect>,
        tick: Option<fn(effect: u128, owner: u128, target: &Target, layer: &mut Layer)>,
        remove: Option<fn(effect: u128, owner: u128, target: &Target, layer: &mut Layer)>,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let id = rng.gen();

        Self {
            duration,
            stats,
            apply,
            tick,
            remove,
            id,
        }
    }

    pub fn get_stat_value(&self, value: &str) -> i32 {
        let level = self.stats.get_level();
        let stat_value = self
            .stats
            .values
            .get(value)
            .expect("effect functions should only access existing stats")
            .get_value(level);

        stat_value
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}
