use luola::messages::*;
use std::net::{TcpListener, TcpStream};

fn wait_for_connections(expected_players: usize) -> Vec<TcpStream> {
    let addr = "127.0.0.1:26988";

    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => panic!("failed to bind to address {}: {:?}", addr, e),
    };

    let mut sockets: Vec<TcpStream> = Vec::new();

    while sockets.len() < expected_players {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new connection from {}", addr);
                sockets.push(socket);
            }
            Err(e) => println!("failed to accept connection: {:?}", e),
        }
    }

    sockets
}

fn main() {
    let player_count: usize = 1;

    let mut sockets = wait_for_connections(player_count);

    let msg: Message = luola::net::receive(&mut sockets[0]);

    match msg {
        Message::Join(join_msg) => {
            println!(
                "received join msg, character name: {}, version: {}",
                join_msg.character_name, join_msg.version
            );
        }
    };
}
