use luola::constants;
use luola::creature::action::Action;
use luola::messages::*;
use luola::player::Player;
use luola::world::Layer;
use std::net::TcpStream;

fn act(player: &mut Player) {
    let cur_action = Action::Idle;
    let msg = Message::Act(cur_action);

    println!("trying to act");
    luola::net::send(&mut player.socket, &msg);

    let response = luola::net::receive(&mut player.socket);
    match response {
        Message::ActionOk => println!("action ok"),
        Message::ActionError(err) => println!("action rejected: {}", err.message),
        other => println!("received unexpected message type: {}", other),
    }

    receive_game_state(&mut player.socket);
}

fn receive_game_state(stream: &mut TcpStream) {
    let msg = luola::net::receive(stream);

    match msg {
        Message::GameState(state) => {
            let layer: Layer = state.layer;
            println!("received game state");
            println!("{}", layer);
        }
        other => {
            println!("received unexpected message type: {}", other);
        }
    }
}

fn join(stream: &mut TcpStream) -> u128 {
    let join_msg = Message::Join(JoinMsg {
        version: constants::get_version(),
        character_name: String::from("testcharacter"),
    });

    luola::net::send(stream, &join_msg);

    let response = luola::net::receive(stream);
    match response {
        Message::JoinOk(ok_msg) => {
            println!(
                "successfully joined the game with player id {}",
                ok_msg.player_id
            );

            return ok_msg.player_id;
        }
        Message::JoinError(err) => {
            panic!("failed to join the game: {}", err.message);
        }
        other => {
            panic!("received unexpected message type: {}", other);
        }
    }
}

fn open_stream() -> TcpStream {
    let addr = "127.0.0.1:26988";
    let stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => panic!("failed to connect to {}: {:?}", addr, e),
    };

    stream
}

fn play(player: &mut Player) {
    loop {
        act(player);
    }
}

fn main() {
    let mut stream = open_stream();
    let player_id = join(&mut stream);

    let mut player = Player::build_existing(stream, player_id);
    receive_game_state(&mut player.socket);

    play(&mut player);
}
