use crate::world::Layer;
use serde::{Deserialize, Serialize};
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
    JoinOk,
    JoinError(ErrorMsg),
    GameState(GameStateMsg),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Message::Join(_) => "Join",
            Message::JoinOk => "JoinOk",
            Message::JoinError(_) => "JoinError",
            Message::GameState(_) => "GameState",
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
pub struct ErrorMsg {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
pub struct GameStateMsg {
    pub layer: Layer,
}
