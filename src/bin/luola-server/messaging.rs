use luola::constants;
use luola::creature::action::{self, Action};
use luola::creature::Creature;
use luola::messages::*;
use luola::player::Player;
use luola::world::Layer;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};

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

pub fn wait_for_join(n_players: usize) -> HashMap<u128, Player> {
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

pub fn send_game_state(layer: Layer, players: &mut HashMap<u128, Player>) {
    let message = GameStateMsg { layer };
    let message = Message::GameState(message);
    for (_, player) in players {
        luola::net::send(&mut player.socket, &message);
    }
}

pub fn get_player_action(
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
