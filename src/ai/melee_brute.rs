use crate::ai::AI;
use crate::creature::action::{Action, MoveAction};
use crate::grid::gridalgos;
use crate::world::Layer;
use rand::prelude::*;

// Charge at the closest enemy and attack when in range.
// No fancy target selection.
// No retreat, fight until dead.
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

    let main_item_id = actor_creature.inventory.get_active(0).expect("a creature should have at least one item");
    let main_item = layer.items.get(&main_item_id).unwrap();
    let main_item_range = main_item.get_max_effective_range().expect("the item should be able to target other creatures");

    // Consider the observations of this creature
    // Get a line-of-sight emanation with a radius of the item's range
    // from each observation
    // Try each observation from most promising to the least promising
    //     Primarily sort by distance, then by recency
    //     Get a line-of-sight emanation with a radius of the item's range
    //     from the observation
    //     If we are already within that emanation
    //     and there actually is a creature, attack
    //     Else get the intersection of reachable squares and that emanation
    //     If non-empty, move there => we can try to attack next
    // If no reachable intersections, move to the reachable square closest to
    // the most promising observation

    #[rustfmt::skip]
    let all_paths = gridalgos::find_all_shortest_paths(
        &vec![cur_position],
        movement_speed,
        &layer.grid
    );

    let reachable_squares = gridalgos::get_reachable_squares(&all_paths);

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
