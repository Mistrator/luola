use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Inventory {
    active_items: Vec<Option<u128>>,
    passive_items: Vec<Option<u128>>,
}

impl Inventory {
    pub fn new(n_active: usize, n_passive: usize) -> Self {
        Self {
            active_items: vec![None; n_active],
            passive_items: vec![None; n_passive],
        }
    }

    pub fn valid_active_slot(&self, slot: usize) -> bool {
        slot < self.active_items.len()
    }

    pub fn get_active(&self, slot: usize) -> Option<u128> {
        self.active_items[slot]
    }

    pub fn replace_active(&self, slot: usize, new_item: u128) -> Option<u128> {
        let old_item = self.active_items[slot];
        self.active_items[slot] = Some(new_item);

        old_item
    }

    pub fn drop_active(&self, slot: usize) -> Option<u128> {
        let old_item = self.active_items[slot];
        self.active_items[slot] = None;
        old_item
    }
}
