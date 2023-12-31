use crate::creature::{action::Action, perception::Awareness, Creature};
use crate::world::Layer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Behavior {
    PlayerControlled(u128),
    Inactive,
}

pub fn act(actor: &Creature, _layer: &Layer) -> Action {
    let actor_behavior = match actor.perception.get_awareness() {
        Awareness::Wander => actor.get_wander_behavior(),
        Awareness::Combat => actor.get_combat_behavior(),
    };

    match actor_behavior {
        Behavior::PlayerControlled(_) => {
            panic!("AI can't control player-controlled characters");
        }
        Behavior::Inactive => Action::Idle,
    }
}
