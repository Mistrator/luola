use crate::creature::{action::Action, Creature};
use crate::world::Layer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Behavior {
    PlayerControlled(u128),
    Inactive,
}

pub fn act(actor: &Creature, _layer: &Layer) -> Action {
    match actor.get_behavior() {
        Behavior::PlayerControlled(_) => {
            panic!("AI can't control player-controlled characters");
        }
        Behavior::Inactive => Action::Idle,
    }
}
