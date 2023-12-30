use rand::prelude::*;
use std::net::TcpStream;

pub struct Player {
    pub socket: TcpStream,
    id: u128,
}

impl Player {
    pub fn new(socket: TcpStream) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            socket: socket,
            id: rng.gen(),
        }
    }

    pub fn build_existing(socket: TcpStream, id: u128) -> Self {
        Self { socket, id }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }
}
