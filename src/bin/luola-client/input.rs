use crate::actions;
use crate::GameState;
use luola::messages::Message;
use std::io::{self, ErrorKind, Read};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum InputEvent {
    Move(Direction),
    UseItem,
    SelectInventorySlot(usize),
}

pub fn handle_input(
    input_rx: &Receiver<InputEvent>,
    outgoing_tx: &Sender<Message>,
    state: &mut GameState,
) {
    let mut input_events: Vec<InputEvent> = Vec::new();

    // We could just iterate over try_iter() but it does not allow us to distinguish
    // between running out of values and sender failure
    loop {
        match input_rx.try_recv() {
            Ok(event) => input_events.push(event),
            Err(e) => match e {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => {
                    panic!("failed to receive input event: sender has been disconnected")
                }
            },
        }
    }

    for event in input_events {
        match event {
            InputEvent::Move(direction) => actions::move_selection(direction, &mut state.ui),
            InputEvent::SelectInventorySlot(slot) => {
                actions::select_inventory_slot(slot, &mut state.ui)
            }
            InputEvent::UseItem => actions::use_item(outgoing_tx, state),
        }
    }
}

pub fn spawn_polling_thread() -> Receiver<InputEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        let input_events = poll_input();
        for event in input_events {
            tx.send(event).expect(
                "failed to send input event: receiver has disconnected, did the main thread panic?",
            );
        }
    });

    rx
}

fn poll_input() -> Vec<InputEvent> {
    let mut buf = vec![0u8; 256];

    if let Err(e) = io::stdin().read(&mut buf) {
        match e.kind() {
            ErrorKind::Interrupted => (),
            _ => panic!("failed to read input: {:?}", e),
        }
    }

    let buf = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(_) => panic!("input bytes were not valid UTF-8"),
    };

    let buf = buf.to_lowercase();

    let mut input_events: Vec<InputEvent> = Vec::new();

    for c in buf.chars() {
        let event = match c {
            'w' => Some(InputEvent::Move(Direction::Up)),
            's' => Some(InputEvent::Move(Direction::Down)),
            'a' => Some(InputEvent::Move(Direction::Left)),
            'd' => Some(InputEvent::Move(Direction::Right)),
            'q' => Some(InputEvent::UseItem),
            '1'..='9' => {
                let slot = ((c as u32) - ('0' as u32)) as usize;
                Some(InputEvent::SelectInventorySlot(slot))
            }
            _ => None,
        };

        if let Some(e) = event {
            input_events.push(e);
        }
    }

    // Remove all duplicate events
    input_events.sort();
    input_events.dedup();

    input_events
}
