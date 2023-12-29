use luola::constants;
use luola::messages::*;
use luola::world::Layer;
use std::net::TcpStream;

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

fn join(stream: &mut TcpStream) {
    let join_msg = Message::Join(JoinMsg {
        version: constants::get_version(),
        character_name: String::from("testcharacter"),
    });

    luola::net::send(stream, &join_msg);

    let response = luola::net::receive(stream);
    match response {
        Message::JoinOk => {
            println!("successfully joined the game");
        }
        Message::JoinError(err) => {
            println!("failed to join the game: {}", err.message);
        }
        other => {
            println!("received unexpected message type: {}", other);
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

fn main() {
    let mut stream = open_stream();
    join(&mut stream);
    receive_game_state(&mut stream);
}
