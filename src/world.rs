use crate::ai::AI;
use crate::creature::Creature;
use crate::grid::Grid;
use crate::item::effect::Effect;
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
    pub item_effect: HashMap<u128, Effect>,
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
            item_effect: HashMap::new(),
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
            item_effect: HashMap::new(),
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
        self.item_effect.insert(id, effect);
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
            write!(f, "{} (Level {})\n", item.name, item.stats.get_level())?;
            write!(f, "{}\n", item.description)?;
        }

        Ok(())
    }
}
