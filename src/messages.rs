use crate::creature::action::Action;
use crate::creature::Creature;
use crate::grid::Grid;
use crate::item::Item;
use crate::world::Layer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct Header {
    pub payload_len: u64,
}

impl Header {
    pub fn new(payload_len: usize) -> Self {
        Self {
            payload_len: payload_len as u64,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum Message {
    Join(JoinMsg),
    JoinOk(JoinOkMsg),
    JoinError(ErrorMsg),
    GameState(GameStateMsg),
    Act(Action),
    ActionOk,
    ActionError(ErrorMsg),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Message::Join(_) => "Join",
            Message::JoinOk(_) => "JoinOk",
            Message::JoinError(_) => "JoinError",
            Message::GameState(_) => "GameState",
            Message::Act(_) => "Act",
            Message::ActionOk => "ActionOk",
            Message::ActionError(_) => "ActionError",
        };

        write!(f, "{}", variant)
    }
}

#[derive(Deserialize, Serialize)]
pub struct JoinMsg {
    pub version: String,
    pub character_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct JoinOkMsg {
    pub player_id: u128,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorMsg {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct GameStateMsg {
    pub creatures: HashMap<u128, Creature>,
    pub items: HashMap<u128, Item>,
    pub grid: Grid,
}

impl GameStateMsg {
    pub fn new(layer: &Layer) -> Self {
        Self {
            creatures: layer.creatures.clone(),
            items: layer.items.clone(),
            grid: layer.grid.clone(),
        }
    }
}
