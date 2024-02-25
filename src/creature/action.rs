use crate::creature::Creature;
use crate::grid::gridalgos;
use crate::grid::GridSquare;
use crate::item::targeting::Target;
use crate::world::Layer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Action {
    Idle,
    Move(MoveAction),
    UseItem(UseItemAction),
}

#[derive(Deserialize, Serialize)]
pub struct MoveAction {
    pub destination: GridSquare,
}

#[derive(Deserialize, Serialize)]
pub struct UseItemAction {
    pub inventory_slot: usize,
    pub target: Target,
}

pub fn is_valid(
    action: &Action,
    _prev_actions: &Vec<Action>,
    actor: &Creature,
    layer: &Layer,
) -> Result<(), String> {
    match action {
        Action::Idle => Ok(()),
        Action::Move(m) => {
            let source = actor.get_position();
            let movement_speed = actor.stats.movement_speed.get_value(actor.stats.level);

            if !layer.grid.valid_square(m.destination) {
                return Err(String::from("move destination square is outside the grid"));
            }

            if !layer.grid.free_square(m.destination) {
                return Err(String::from("move destination square is not empty"));
            }

            #[rustfmt::skip]
            let all_paths = gridalgos::find_all_shortest_paths(
                &vec![source],
                movement_speed,
                &layer.grid,
            );

            let shortest_path = gridalgos::get_shortest_path(&all_paths, m.destination);

            if shortest_path.is_none() {
                return Err(String::from(
                    "move destination square not reachable or too far away",
                ));
            }

            Ok(())
        }
        Action::UseItem(u) => {
            // todo: check that the targets exist
            // todo: check that the targets are of the right type for the item
            // todo: check that the targets fulfill the targeting constraints of the item
            let inv = &actor.inventory;
            if !inv.valid_active_slot(u.inventory_slot) {
                return Err(String::from(format!(
                    "active item slot {} does not exist",
                    u.inventory_slot
                )));
            }

            let item_id = inv.get_active(u.inventory_slot);
            if item_id.is_none() {
                return Err(String::from(format!(
                    "active item slot {} is empty",
                    u.inventory_slot
                )));
            }

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
        Action::UseItem(u) => {
            let inv = &actor.inventory;
            let item_id = inv
                .get_active(u.inventory_slot)
                .expect("the slot should exist and contain an item");

            let effect = layer
                .effects
                .get(&item_id)
                .expect("an item should have an effect");

            let ongoing_effect = (effect.apply)(item_id, actor_id, u.target.clone(), layer);
            if let Some(e) = ongoing_effect {
                layer.ongoing_effects.insert(e.get_id(), e);
            }
        }
    }
}
