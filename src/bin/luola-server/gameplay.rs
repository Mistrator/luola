use crate::messaging;
use luola::ai;
use luola::creature::action::{self, Action};
use luola::creature::perception::{Awareness, Perception};
use luola::initiative::Initiative;
use luola::player::Player;
use luola::world::{Entity, Layer, World};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Serialize)]
enum GameplayMode {
    Start,
    Exploration,
    Combat,
}

// Returns true if a non-player character is alerted.
fn take_creature_turn(
    creature_id: u128,
    layer: &mut Layer,
    players: &mut HashMap<u128, Player>,
    gameplay_mode: GameplayMode,
    current_round: i64,
) -> bool {
    let mut prev_actions: Vec<Action> = Vec::new();

    // todo: take this from creature stats
    let creature_max_actions = 2;

    while prev_actions.len() < creature_max_actions {
        let creature = layer.creatures.get(&creature_id).unwrap();
        let c_ai = layer.creature_ai.get(&creature_id).unwrap();
        let cur_action: Action = match c_ai.get_controlling_player_id() {
            Some(player_id) => {
                let mut player = players
                    .get_mut(&player_id)
                    .expect("creature should be controlled by an existing player");
                messaging::get_player_action(&mut player, &prev_actions, creature, layer)
            }
            None => {
                let ai_action = ai::act(c_ai, layer);
                action::is_valid(&ai_action, &prev_actions, creature, layer)
                    .expect("AI should not take an invalid action");
                ai_action
            }
        };

        println!("creature {} acts", creature.get_id());

        action::execute(&cur_action, creature_id, layer);
        prev_actions.push(cur_action);

        Perception::update_all_observations(
            &mut layer.creature_ai,
            &layer.grid,
            &layer.creatures,
            current_round,
        );

        messaging::send_game_state(layer, players);

        // Re-borrow as immutable to satisfy the borrow checker
        let c_ai = layer.creature_ai.get(&creature_id).unwrap();

        // In exploration mode, if a player acts and any creature is alerted,
        // immediately end the turn. However, if a non-player creature
        // acts, only end the turn if the creature itself is alerted.
        if gameplay_mode == GameplayMode::Exploration {
            if c_ai.is_player_controlled() {
                for (id, _) in &layer.creatures {
                    let other_ai = layer.creature_ai.get(&id).unwrap();
                    if !other_ai.is_player_controlled()
                        && other_ai.perception.get_awareness() == Awareness::Combat
                    {
                        // A player alerted some non-player creature.
                        return true;
                    }
                }
            } else {
                if c_ai.perception.get_awareness() == Awareness::Combat {
                    // This non-player creature got alerted.
                    return true;
                }
            }
        }
    }

    for (id, _) in &layer.creatures {
        let other_ai = layer.creature_ai.get(&id).unwrap();
        if !other_ai.is_player_controlled()
            && other_ai.perception.get_awareness() == Awareness::Combat
        {
            // Some non-player creature is alerted at the moment.
            return true;
        }
    }

    // No non-player characters are alerted at the moment.
    return false;
}

fn run_exploration_round(
    layer: &mut Layer,
    players: &mut HashMap<u128, Player>,
    init: &Initiative,
    current_round: i64,
) -> GameplayMode {
    let aware = init.get_aware(&layer.creature_ai);
    let wandering = init.get_wandering(&layer.creature_ai);

    for (_, creature_id) in aware {
        assert!(
            layer
                .creature_ai
                .get(&creature_id)
                .unwrap()
                .is_player_controlled(),
            "only player-controlled characters should be aware in exploration mode"
        );

        // If a creature is alerted because of a player action,
        // immediately transition to combat. Otherwise players would always get the
        // first attack by rushing in and attacking.
        let someone_alerted = take_creature_turn(
            creature_id,
            layer,
            players,
            GameplayMode::Exploration,
            current_round,
        );

        if someone_alerted {
            return GameplayMode::Combat;
        }
    }

    // If a creature is alerted while it wanders, do not switch to combat immediately.
    // We let other wandering creatures take their turn first. This makes it possible
    // for a group of creatures to wander into players instead of only one.
    let mut wandering_creature_alerted = false;

    // todo: fast-forward these somehow
    for (_, creature_id) in wandering {
        let someone_alerted = take_creature_turn(
            creature_id,
            layer,
            players,
            GameplayMode::Exploration,
            current_round,
        );

        if someone_alerted {
            wandering_creature_alerted = true;
        }
    }

    if wandering_creature_alerted {
        return GameplayMode::Combat;
    }

    return GameplayMode::Exploration;
}

fn run_combat_round(
    layer: &mut Layer,
    players: &mut HashMap<u128, Player>,
    init: &Initiative,
    current_round: i64,
) -> GameplayMode {
    let aware = init.get_aware(&layer.creature_ai);
    let wandering = init.get_wandering(&layer.creature_ai);

    for (_, creature_id) in aware {
        let someone_alerted = take_creature_turn(
            creature_id,
            layer,
            players,
            GameplayMode::Combat,
            current_round,
        );

        if !someone_alerted {
            return GameplayMode::Exploration;
        }
    }

    // todo: fast-forward these somehow
    for (_, creature_id) in wandering {
        let someone_alerted = take_creature_turn(
            creature_id,
            layer,
            players,
            GameplayMode::Combat,
            current_round,
        );
        if !someone_alerted {
            return GameplayMode::Exploration;
        }
    }

    return GameplayMode::Combat;
}

pub fn run_game(mut world: World, mut players: HashMap<u128, Player>) {
    let current_layer: usize = 0;
    let mut current_mode = GameplayMode::Start;
    let mut next_mode = GameplayMode::Exploration;
    let mut init = Initiative::roll_initiative(&world.layers[current_layer]);

    let mut current_round: i64 = 0;

    messaging::send_game_state(&world.layers[current_layer], &mut players);

    loop {
        if next_mode != current_mode {
            init = Initiative::roll_initiative(&world.layers[current_layer]);

            match next_mode {
                GameplayMode::Exploration => {
                    println!("switch to exploration mode");
                }
                GameplayMode::Combat => {
                    println!("switch to combat mode");
                }
                GameplayMode::Start => panic!("can't transition into start mode"),
            }

            current_mode = next_mode;
        }

        match current_mode {
            GameplayMode::Exploration => {
                println!("start exploration round");
                next_mode = run_exploration_round(
                    &mut world.layers[current_layer],
                    &mut players,
                    &init,
                    current_round,
                );
            }
            GameplayMode::Combat => {
                println!("start combat round");
                next_mode = run_combat_round(
                    &mut world.layers[current_layer],
                    &mut players,
                    &init,
                    current_round,
                );
            }
            GameplayMode::Start => panic!("must switch away from start mode"),
        }

        current_round += 1;
    }
}
