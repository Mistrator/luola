use crate::actions;
use crate::GameState;
use luola::info_message::MessageType;
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
    MoveSelection(Direction),
    MoveCreature,
    UseItem,
    SelectInventorySlot(usize),
}

fn allowed_to_act(state: &mut GameState) -> bool {
    let acting_creature = match state.acting_creature {
        Some(c) => c,
        None => return false,
    };

    if !state.this_player_controls(acting_creature) {
        let error_msg = MessageType::Error(String::from("Can't act: it's not your turn"));
        state.ui.message_log.add_message(error_msg);
        return false;
    }

    true
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
            InputEvent::MoveSelection(direction) => actions::move_selection(direction, state),
            InputEvent::SelectInventorySlot(slot) => {
                actions::select_inventory_slot(slot, &mut state.ui)
            }
            InputEvent::MoveCreature => {
                if allowed_to_act(state) {
                    actions::move_creature(outgoing_tx, state);
                }
            }
            InputEvent::UseItem => {
                if allowed_to_act(state) {
                    actions::use_item(outgoing_tx, state);
                }
            }
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
            'w' => Some(InputEvent::MoveSelection(Direction::Up)),
            's' => Some(InputEvent::MoveSelection(Direction::Down)),
            'a' => Some(InputEvent::MoveSelection(Direction::Left)),
            'd' => Some(InputEvent::MoveSelection(Direction::Right)),
            'q' => Some(InputEvent::UseItem),
            ' ' => Some(InputEvent::MoveCreature),
            '1'..='9' => {
                let slot = ((c as u32) - ('1' as u32)) as usize;
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
