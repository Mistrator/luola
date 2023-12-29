use crate::player::Player;
use luola::constants;
use luola::messages::*;
use luola::world::{Layer, World};
use std::net::{TcpListener, TcpStream};

mod player;
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

            let mut player = Player::new(socket, join_msg.character_name);

            let response = Message::JoinOk;
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

fn wait_for_join(n_players: usize) -> Vec<Player> {
    let addr = "127.0.0.1:26988";

    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => panic!("failed to bind to address {}: {:?}", addr, e),
    };

    let mut players: Vec<Player> = Vec::new();

    while players.len() < n_players {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new connection from {}", addr);
                match handle_join(socket) {
                    Some(player) => {
                        println!("{} joined the game", player.character.name);
                        players.push(player);
                    }
                    None => println!("{} failed to join the game", addr),
                }
            }
            Err(e) => println!("failed to accept connection: {:?}", e),
        }
    }

    players
}

fn send_game_state(layer: Layer, players: &mut Vec<Player>) {
    let message = GameStateMsg { layer };
    let message = Message::GameState(message);
    for player in players {
        luola::net::send(&mut player.socket, &message);
    }
}

fn start_game(world: World, mut players: Vec<Player>) {
    let layer = world.layers[0].clone();

    send_game_state(layer, &mut players);
}

fn main() {
    let n_players: usize = 2;
    let worldgen_seed: u64 = 1;

    println!("generating world with seed {}", worldgen_seed);
    let world: World = worldgen::generate_world(worldgen_seed);
    println!("world generated with {} layers", world.layers.len());

    let players: Vec<Player> = wait_for_join(n_players);
    println!("{} players connected, ready to start", players.len());

    start_game(world, players);
}
