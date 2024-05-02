use crate::creature::action::Action;
use crate::creature::Creature;
use crate::grid::Grid;
use crate::info_message::MessageType;
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
    Info(MessageType),
    Act(Action),
    ActionOk,
    ActionError,
    TurnStart(TurnStartMsg),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variant = match self {
            Message::Join(_) => "Join",
            Message::JoinOk(_) => "JoinOk",
            Message::JoinError(_) => "JoinError",
            Message::GameState(_) => "GameState",
            Message::Info(_) => "Info",
            Message::Act(_) => "Act",
            Message::ActionOk => "ActionOk",
            Message::ActionError => "ActionError",
            Message::TurnStart(_) => "TurnStart",
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
pub struct TurnStartMsg {
    pub acting_creature: u128,
}

#[derive(Deserialize, Serialize)]
pub enum CreatureOwner {
    Player(u128),
    AI,
}

#[derive(Deserialize, Serialize)]
pub struct GameStateMsg {
    pub creatures: HashMap<u128, Creature>,
    pub creature_owners: HashMap<u128, CreatureOwner>,
    pub items: HashMap<u128, Item>,
    pub grid: Grid,
}

impl GameStateMsg {
    pub fn new(layer: &Layer) -> Self {
        let mut creature_owners: HashMap<u128, CreatureOwner> = HashMap::new();

        for (c_id, _) in &layer.creatures {
            let ai = layer
                .creature_ai
                .get(c_id)
                .expect("every creature should have an ai component");

            let player_id = ai.get_controlling_player_id();

            let creature_owner = match player_id {
                Some(id) => CreatureOwner::Player(id),
                None => CreatureOwner::AI,
            };

            creature_owners.insert(*c_id, creature_owner);
        }

        Self {
            creatures: layer.creatures.clone(),
            creature_owners,
            items: layer.items.clone(),
            grid: layer.grid.clone(),
        }
    }
}
