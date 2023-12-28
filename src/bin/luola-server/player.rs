use luola::character::Character;
use std::net::TcpStream;

pub struct Player {
    pub socket: TcpStream,
    pub character: Character,
}

impl Player {
    pub fn new(socket: TcpStream, name: String) -> Self {
        let character = Character::new(name);
        Self { socket, character }
    }
}
