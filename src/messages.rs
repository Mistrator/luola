use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Serialize)]
pub enum Message {
    Join(JoinMsg),
    JoinOk,
    JoinError(ErrorMsg),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinMsg {
    pub version: String,
    pub character_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorMsg {
    pub message: String,
}
