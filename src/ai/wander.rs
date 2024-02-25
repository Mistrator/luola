use crate::ai::AI;
use crate::creature::action::{Action, MoveAction};
use crate::grid::gridalgos;
use crate::world::Layer;
use rand::prelude::*;

pub fn act(actor: &AI, layer: &Layer) -> Action {
    let mut rng = rand::thread_rng();

    let actor_creature = layer
        .creatures
        .get(&actor.owner_id)
        .expect("AI should have a valid owner");

    let cur_position = actor_creature.get_position();
    let movement_speed = actor_creature
        .stats
        .movement_speed
        .get_value(actor_creature.stats.level);

    #[rustfmt::skip]
    let all_paths = gridalgos::find_all_shortest_paths(
        &vec![cur_position],
        movement_speed,
        &layer.grid
    );

    let reachable_squares = gridalgos::get_reachable_squares(&all_paths);
    let destination = reachable_squares.choose(&mut rng);

    match destination {
        Some(d) => {
            println!("move from {} to {}", cur_position, *d);
            Action::Move(MoveAction { destination: *d })
        }
        None => {
            println!("no reachable destination squares, idle");
            Action::Idle
        }
    }
}
