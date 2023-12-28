use luola::constants;
use luola::messages::*;
use std::net::TcpStream;

fn join(stream: &mut TcpStream) {
    let join_msg = Message::Join(JoinMsg {
        version: constants::get_version(),
        character_name: String::from("testcharacter"),
    });

    luola::net::send(stream, join_msg);
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
}
