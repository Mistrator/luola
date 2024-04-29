use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Inventory {
    items: Vec<Option<u128>>,
}

impl Inventory {
    pub fn new(n_slots: usize) -> Self {
        Self {
            items: vec![None; n_slots],
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn valid_slot(&self, slot: usize) -> bool {
        slot < self.len()
    }

    pub fn get_item(&self, slot: usize) -> Option<u128> {
        if !self.valid_slot(slot) {
            return None;
        }

        self.items[slot]
    }

    pub fn replace_item(&mut self, slot: usize, new_item: u128) -> Option<u128> {
        let old_item = self.get_item(slot);
        self.items[slot] = Some(new_item);

        old_item
    }

    pub fn drop_item(&mut self, slot: usize) -> Option<u128> {
        let old_item = self.items[slot];
        self.items[slot] = None;

        old_item
    }
}
