use crate::ui::UI;
use luola::grid::GridSquare;
use std::io::{self, ErrorKind, Read};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum InputEvent {
    Move(Direction),
    UseItem,
    SelectInventorySlot(usize),
}

pub fn handle_input(ui: &mut UI) {
    let input_events = poll_input();

    for event in input_events {
        match event {
            InputEvent::Move(direction) => {
                let delta = match direction {
                    Direction::Up => GridSquare { y: -1, x: 0 },
                    Direction::Down => GridSquare { y: 1, x: 0 },
                    Direction::Left => GridSquare { y: 0, x: -1 },
                    Direction::Right => GridSquare { y: 0, x: 1 },
                };

                ui.viewport.move_selection(delta);
            }
            InputEvent::UseItem => (),
            InputEvent::SelectInventorySlot(slot) => {
                ui.inventory_info.select_slot(slot);
            }
        }
    }
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
