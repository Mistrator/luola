use luola::ai::{self, Behavior};
use luola::constants;
use luola::creature::action::{self, Action};
use luola::creature::Creature;
use luola::messages::*;
use luola::player::Player;
use luola::world::{Entity, Layer, World};
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

mod worldgen;

fn handle_join(mut socket: TcpStream) -> Option<Player> {
    let msg = luola::net::receive(&mut socket);
    match msg {
        Message::Join(join_msg) => {
            let server_version = constants::get_version();
            let client_version = join_msg.version;

            if server_version != client_version {
                let response_text = format!(
                    "mismatching game versions: server version {}, client version {}",
                    server_version, client_version
                );
                println!("{}", response_text);

                let response = ErrorMsg {
                    message: String::from(response_text),
                };
                let response = Message::JoinError(response);

                luola::net::send(&mut socket, &response);
                return None;
            }

            let mut player = Player::new(socket);

            let response = JoinOkMsg {
                player_id: player.get_id(),
            };
            let response = Message::JoinOk(response);
            luola::net::send(&mut player.socket, &response);

            return Some(player);
        }
        other => {
            println!(
                "received unexpected message type: expected Join, got {}",
                other
            );

            let response = ErrorMsg {
                message: String::from(format!("unexpected message type: {}", other)),
            };
            let response = Message::JoinError(response);

            luola::net::send(&mut socket, &response);
            return None;
        }
    }
}

fn wait_for_join(n_players: usize) -> HashMap<u128, Player> {
    let addr = "127.0.0.1:26988";

    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => panic!("failed to bind to address {}: {:?}", addr, e),
    };

    let mut players: HashMap<u128, Player> = HashMap::new();

    while players.len() < n_players {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new connection from {}", addr);
                match handle_join(socket) {
                    Some(player) => {
                        println!("player id {} joined the game", player.get_id());
                        players.insert(player.get_id(), player);
                    }
                    None => println!("{} failed to join the game", addr),
                }
            }
            Err(e) => println!("failed to accept connection: {:?}", e),
        }
    }

    players
}

fn get_player_action(
    player: &mut Player,
    prev_actions: &Vec<Action>,
    creature: &Creature,
    layer: &Layer,
) -> Action {
    loop {
        let msg: Message = luola::net::receive(&mut player.socket);

        match msg {
            Message::Act(player_action) => {
                match action::is_valid(&player_action, prev_actions, creature, layer) {
                    Ok(()) => {
                        let response = Message::ActionOk;
                        luola::net::send(&mut player.socket, &response);

                        println!("received a valid action from player");
                        return player_action;
                    }
                    Err(msg) => {
                        println!(
                            "player {} tried to take an invalid action: {}",
                            player.get_id(),
                            msg
                        );
                        let response = ErrorMsg { message: msg };
                        let response = Message::ActionError(response);
                        luola::net::send(&mut player.socket, &response);
                    }
                }
            }
            other => {
                println!(
                    "received unexpected message type: expected Act, got {}",
                    other
                );

                let response = ErrorMsg {
                    message: String::from(format!("unexpected message type: {}", other)),
                };
                let response = Message::ActionError(response);

                luola::net::send(&mut player.socket, &response);
            }
        };
    }
}

fn take_creature_turn(creature_id: u128, layer: &mut Layer, players: &mut HashMap<u128, Player>) {
    let mut prev_actions: Vec<Action> = Vec::new();

    // todo: take this from creature stats
    let creature_max_actions = 2;

    while prev_actions.len() < creature_max_actions {
        let creature = layer.creatures.get(&creature_id).unwrap();
        let cur_action: Action = match creature.get_controlling_player_id() {
            Some(player_id) => {
                let mut player = players
                    .get_mut(&player_id)
                    .expect("creature should be controlled by an existing player");
                get_player_action(&mut player, &prev_actions, creature, layer)
            }
            None => {
                let ai_action = ai::act(creature, layer);
                action::is_valid(&ai_action, &prev_actions, creature, layer)
                    .expect("AI should not take an invalid action");
                ai_action
            }
        };

        println!("creature {} acts", creature.get_id());
        action::execute(&cur_action, creature_id, layer);
        prev_actions.push(cur_action);
    }
}

fn send_game_state(layer: Layer, players: &mut HashMap<u128, Player>) {
    let message = GameStateMsg { layer };
    let message = Message::GameState(message);
    for (_, player) in players {
        luola::net::send(&mut player.socket, &message);
    }
}

fn start_game(mut world: World, mut players: HashMap<u128, Player>) {
    let layer = world.layers[0].clone();

    send_game_state(layer, &mut players);

    let mut initiative: Vec<u128> = Vec::new();
    for (id, _) in &world.layers[0].creatures {
        initiative.push(*id);
    }

    for id in initiative {
        println!("turn of creature {}", id);
        take_creature_turn(id, &mut world.layers[0], &mut players);
    }
}

fn main() {
    let n_players: usize = 1;
    let worldgen_seed: u64 = 1;

    println!("generating world with seed {}", worldgen_seed);
    let mut world: World = worldgen::generate_world(worldgen_seed);
    println!("world generated with {} layers", world.layers.len());

    let players: HashMap<u128, Player> = wait_for_join(n_players);
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
        .creatures
        .get_mut(&creature_id)
        .unwrap()
        .set_override_behavior(Behavior::PlayerControlled(player_id));

    start_game(world, players);
}
