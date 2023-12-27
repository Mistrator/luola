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

    let sockets = wait_for_connections(player_count);
    println!("{}", sockets.len());
}
