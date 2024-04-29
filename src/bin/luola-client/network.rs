use crate::GameState;
use luola::constants;
use luola::messages::*;
use luola::world::Layer;
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

// Open two handles to the same underlying socket. This allows us to
// read and write simultaneously in separate threads.
pub fn open_stream(address: String) -> (TcpStream, TcpStream) {
    let tx_stream = match TcpStream::connect(address) {
        Ok(s) => s,
        Err(e) => panic!("failed to connect to server: {:?}", e),
    };

    let rx_stream = tx_stream.try_clone().expect("failed to clone TCP stream");

    (tx_stream, rx_stream)
}

pub fn spawn_incoming_thread(mut rx_stream: TcpStream) -> Receiver<Message> {
    // Messages client has received from server
    let (incoming_tx, incoming_rx) = mpsc::channel();

    thread::spawn(move || loop {
        let rx_message = luola::net::receive(&mut rx_stream);

        incoming_tx.send(rx_message).expect("failed to mpsc-send incoming message: mpsc receiver has disconnected, did the main thread panic?");
    });

    incoming_rx
}

pub fn spawn_outgoing_thread(mut tx_stream: TcpStream) -> Sender<Message> {
    // Messages client is about to send to server
    let (outgoing_tx, outgoing_rx) = mpsc::channel();

    thread::spawn(move || loop {
        let tx_message = outgoing_rx.recv().expect("failed to mpsc-receive outgoing message: mpsc sender has disconnected, did the main thread panic?");

        luola::net::send(&mut tx_stream, &tx_message);
    });

    outgoing_tx
}

pub fn send_message(outgoing_tx: &Sender<Message>, message: Message) {
    outgoing_tx
        .send(message)
        .expect("failed to mpsc-send outgoing message: mpsc receiver has disconnected");
}

fn receive_message(incoming_rx: &Receiver<Message>) -> Message {
    incoming_rx
        .recv()
        .expect("failed to mpsc-receive incoming message: mpsc sender has disconnected")
}

fn handle_rx_message(message: Message, outgoing_tx: &Sender<Message>, state: &mut GameState) {
    match message {
        Message::GameState(game_state) => {
            let layer = Layer::reconstruct(game_state.grid, game_state.creatures, game_state.items);
            state.layer = layer;
        }
        Message::Info(msg) => {
            state.ui.message_log.add_message(msg);
        }
        Message::ActionOk => (),
        Message::ActionError => (),
        _ => panic!("received unexpected message type: {}", message),
    }
}

pub fn handle_messaging(
    incoming_rx: &Receiver<Message>,
    outgoing_tx: &Sender<Message>,
    state: &mut GameState,
) {
    let mut received_messages: Vec<Message> = Vec::new();

    loop {
        match incoming_rx.try_recv() {
            Ok(msg) => received_messages.push(msg),
            Err(e) => match e {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => {
                    panic!("failed to mpsc-receive incoming message: mpsc sender has disconnected")
                }
            },
        }
    }

    for rx_message in received_messages {
        handle_rx_message(rx_message, outgoing_tx, state);
    }
}

pub fn join_game(outgoing_tx: &Sender<Message>, incoming_rx: &Receiver<Message>) -> u128 {
    let join_msg = Message::Join(JoinMsg {
        version: constants::get_version(),
        character_name: String::from("testcharacter"),
    });

    send_message(outgoing_tx, join_msg);

    let response = receive_message(incoming_rx);
    let player_id = match response {
        Message::JoinOk(msg) => msg.player_id,
        Message::JoinError(err) => panic!("failed to join game: {}", err.message),
        other => panic!("unexpected response type when joining game: {}", other),
    };

    player_id
}
