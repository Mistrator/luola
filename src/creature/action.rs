use crate::creature::Creature;
use crate::world::{GridSquare, Layer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Action {
    Idle,
    Move(MoveAction),
}

#[derive(Deserialize, Serialize)]
pub struct MoveAction {
    pub destination: GridSquare,
}

pub fn is_valid(
    _action: &Action,
    _prev_actions: &Vec<Action>,
    _actor: &Creature,
    _layer: &Layer,
) -> Result<(), String> {
    Ok(())
}

pub fn execute(_action: &Action, _actor_id: u128, _layer: &mut Layer) {}
