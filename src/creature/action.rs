use crate::creature::Creature;
use crate::grid::GridSquare;
use crate::world::{Entity, Layer};
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
    action: &Action,
    _prev_actions: &Vec<Action>,
    _actor: &Creature,
    layer: &Layer,
) -> Result<(), String> {
    match action {
        Action::Idle => Ok(()),
        Action::Move(m) => {
            if !layer.grid.valid_square(&m.destination) {
                return Err(String::from("move destination square is outside the grid"));
            }

            if !layer.grid.free_square(&m.destination) {
                return Err(String::from("move destination square is not empty"));
            }

            // todo: check that the destination is not too far away for the creature
            // todo: check that there is an unobstructed path to the destination square

            Ok(())
        }
    }
}

pub fn execute(action: &Action, actor_id: u128, layer: &mut Layer) {
    let actor = layer
        .creatures
        .get_mut(&actor_id)
        .expect("actor should be a valid creature");

    match action {
        Action::Idle => return,
        Action::Move(m) => {
            println!("position before move: {}", actor.get_position());
            actor.set_position(&m.destination);
            println!("position after move: {}", actor.get_position());
        }
    }
}
