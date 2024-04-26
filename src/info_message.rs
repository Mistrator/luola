use crate::check::Check;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum MessageType {
    Info(String),
    Error(String),
    Attack(AttackMessage),
}

#[derive(Deserialize, Serialize)]
pub struct AttackResult {
    pub target: u128,
    pub check: Check,
    pub damage: i32,
}

#[derive(Deserialize, Serialize)]
pub struct AttackMessage {
    pub attacker: u128,
    pub item: u128,
    pub results: Vec<AttackResult>,
}
