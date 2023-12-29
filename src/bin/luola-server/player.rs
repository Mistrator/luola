use luola::creature::Creature;
use std::net::TcpStream;

pub struct Player {
    pub socket: TcpStream,
    pub character: Creature,
}

impl Player {
    pub fn new(socket: TcpStream, name: String) -> Self {
        let character = Creature::new(name);
        Self { socket, character }
    }
}
