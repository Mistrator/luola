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
    world.layers[0]
        .creature_ai
        .get_mut(&creature_id)
        .unwrap()
        .set_override_behavior(Behavior::PlayerControlled(player_id));

    // debug: add stat modifiers
    for (_, c) in &mut world.layers[0].creatures {
        c.stats.melee_attack.apply_additive_modifier(1111, 5);
        c.stats
            .magic_attack
            .apply_multiplicative_modifier(2222, 1.2);

        c.stats.reflex_dc.apply_additive_modifier(3333, -2);
        c.stats.reflex_dc.apply_multiplicative_modifier(4444, 1.4);

        c.stats.movement_speed.apply_additive_modifier(5555, 3);
        c.stats.initiative.apply_multiplicative_modifier(6666, 0.85);
    }

    /*
    let mut creature = world.layers[0].creatures.get_mut(&creature_id).unwrap();
    creature.stats.melee_attack.apply_additive_modifier(1111, 5);
    creature.stats.magic_attack.apply_multiplicative_modifier(2222, 1.2);

    creature.stats.reflex_dc.apply_additive_modifier(3333, 2);
    creature.stats.reflex_dc.apply_multiplicative_modifier(4444, 1.4);
    */

    gameplay::run_game(world, players);
}
