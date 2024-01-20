use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Inventory {
    pub active_items: Vec<u128>,
    pub passive_items: Vec<u128>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            active_items: Vec::new(),
            passive_items: Vec::new(),
        }
    }
}
