use crate::ai::AI;
use crate::creature::Creature;
use crate::grid::{Grid, GridSquare};
use crate::item::effect::{Effect, OngoingEffect};
use crate::item::Item;
use std::collections::HashMap;
use std::fmt;

pub struct World {
    pub layers: Vec<Layer>,
}

impl World {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }
}

pub struct Layer {
    pub creatures: HashMap<u128, Creature>,
    pub creature_ai: HashMap<u128, AI>,
    pub grid: Grid,
    pub items: HashMap<u128, Item>,
    pub effects: HashMap<u128, Effect>,
    pub ongoing_effects: HashMap<u128, OngoingEffect>,
}

impl Layer {
    pub fn new(height: i32, width: i32) -> Self {
        if height <= 0 || width <= 0 {
            panic!("layer dimensions must be greater than zero");
        }

        Self {
            grid: Grid::new(height, width),
            creatures: HashMap::new(),
            creature_ai: HashMap::new(),
            items: HashMap::new(),
            effects: HashMap::new(),
            ongoing_effects: HashMap::new(),
        }
    }

    pub fn reconstruct(
        grid: Grid,
        creatures: HashMap<u128, Creature>,
        items: HashMap<u128, Item>,
    ) -> Self {
        Self {
            grid: grid,
            creatures: creatures,
            items: items,
            creature_ai: HashMap::new(),
            effects: HashMap::new(),
            ongoing_effects: HashMap::new(),
        }
    }

    pub fn add_creature(&mut self, creature: Creature, c_ai: AI) {
        let id = creature.get_id();

        self.creatures.insert(id, creature);
        self.creature_ai.insert(id, c_ai);
    }

    pub fn add_item(&mut self, item: Item, effect: Effect) {
        let id = item.get_id();

        self.items.insert(id, item);
        self.effects.insert(id, effect);
    }

    pub fn get_creatures_at(&self, square: GridSquare) -> Vec<u128> {
        let mut creatures: Vec<u128> = Vec::new();

        for (c_id, creature) in &self.creatures {
            if creature.get_position() == square {
                creatures.push(*c_id);
            }
        }

        creatures
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)?;

        for (_, creature) in &self.creatures {
            let pos = creature.get_position();
            write!(f, "{} ({}, {})\n", creature.name, pos.y, pos.x)?;
            write!(f, "{}", creature.stats)?;
        }

        for (_, item) in &self.items {
            write!(f, "{}\n", item.name)?;
            write!(f, "{}\n", item.description)?;
        }

        Ok(())
    }
}
