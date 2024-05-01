use crate::terminal::canvas::Canvas;
use crate::ui::color_scheme;
use luola::creature::inventory::Inventory;
use luola::item::Item;
use luola::world::Layer;

pub struct InventoryInfo {
    width: usize,
    height: usize,

    selected_slot: Option<usize>,
}

impl InventoryInfo {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            selected_slot: None,
        }
    }

    pub fn select_slot(&mut self, slot: usize) {
        self.selected_slot = Some(slot);
    }

    pub fn get_selected_slot(&self) -> Option<usize> {
        self.selected_slot
    }

    pub fn render(&self, creature_id: Option<u128>, layer: &Layer) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);

        if creature_id.is_none() {
            return canvas;
        }

        let creature_id = creature_id.unwrap();
        let creature = layer.creatures.get(&creature_id).unwrap();
        let inventory = &creature.inventory;

        canvas.write(
            String::from("Inventory"),
            color_scheme::TEXT_HIGHLIGHT_STYLE,
        );
        canvas.write_newline();

        for slot in 0..inventory.len() {
            let item = get_item_from_slot(slot, inventory, layer);
            self.write_inventory_slot(&mut canvas, item, slot);
            canvas.write_newline();
        }
        canvas.write_newline();

        if let Some(selected) = self.selected_slot {
            if let Some(item) = get_item_from_slot(selected, inventory, layer) {
                self.write_selected_item_details(&mut canvas, item);
            }
        }

        canvas
    }

    fn write_inventory_slot(&self, canvas: &mut Canvas, item: Option<&Item>, slot: usize) {
        let mut slot_style = color_scheme::TEXT_HIGHLIGHT_STYLE;

        if let Some(selected) = self.selected_slot {
            if slot == selected {
                slot_style = color_scheme::SELECTION_STYLE;
            }
        }

        // Inventory slots are internally 0-indexed but user-facing slots are 1-indexed
        canvas.write(format!("[{}]: ", slot + 1), slot_style);

        let slot_contents = match item {
            Some(x) => x.name.clone(),
            None => String::from("(empty)"),
        };

        canvas.write(slot_contents, color_scheme::TEXT_STYLE);
    }

    fn write_selected_item_details(&self, canvas: &mut Canvas, item: &Item) {
        canvas.write(item.name.clone(), color_scheme::TEXT_HIGHLIGHT_STYLE);
        canvas.write_newline();

        canvas.write(item.description.clone(), color_scheme::TEXT_STYLE);
    }
}

fn get_item_from_slot<'a>(
    slot: usize,
    inventory: &Inventory,
    layer: &'a Layer,
) -> Option<&'a Item> {
    let item_id = inventory.get_item(slot);
    match item_id {
        Some(id) => layer.items.get(&id),
        None => None,
    }
}
