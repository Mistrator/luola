use luola::ai::Behavior;
use luola::player::Player;
use luola::world::World;
use std::collections::HashMap;

mod gameplay;
mod messaging;
mod worldgen;

fn main() {
    let n_players: usize = 1;
    let worldgen_seed: u64 = 1;

    println!("generating world with seed {}", worldgen_seed);
    let mut world: World = worldgen::generate_world(worldgen_seed);
    println!("world generated with {} layers", world.layers.len());

    let players: HashMap<u128, Player> = messaging::wait_for_join(n_players);
    println!("{} players connected, ready to start", players.len());

    // debug: make one creature player-controlled
    let mut player_id: u128 = 0;
    let mut creature_id: u128 = 0;
    'outer: for (pid, _) in &players {
        for (cid, _) in &world.layers[0].creatures {
            player_id = *pid;
            creature_id = *cid;
            break 'outer;
        }
    }
    world
        .creature_ai
        .get_mut(&creature_id)
        .unwrap()
        .set_override_behavior(Behavior::PlayerControlled(player_id));

    gameplay::run_game(world, players);
}
