use crate::input::Direction;
use crate::network;
use crate::ui::UI;
use crate::GameState;
use luola::creature::action::{Action, UseItemAction};
use luola::grid::GridSquare;
use luola::info_message::MessageType;
use luola::item::ItemKind;
use luola::messages::Message;
use std::sync::mpsc::Sender;

pub fn move_selection(direction: Direction, ui: &mut UI) {
    let delta = match direction {
        Direction::Up => GridSquare { y: -1, x: 0 },
        Direction::Down => GridSquare { y: 1, x: 0 },
        Direction::Left => GridSquare { y: 0, x: -1 },
        Direction::Right => GridSquare { y: 0, x: 1 },
    };

    ui.viewport.move_selection(delta);
}

pub fn select_inventory_slot(slot: usize, ui: &mut UI) {
    ui.inventory_info.select_slot(slot);
}

pub fn use_item(outgoing_tx: &Sender<Message>, state: &mut GameState) {
    let inventory_slot = state.ui.inventory_info.get_selected_slot();
    let acting_creature_id = state
        .acting_creature
        .expect("it should be the player's turn");
    let acting_creature = state.layer.creatures.get(&acting_creature_id).unwrap();

    let item_id = match inventory_slot {
        Some(slot) => acting_creature.inventory.get_item(slot),
        None => None,
    };

    if item_id.is_none() {
        let error_msg = MessageType::Error(String::from("Inventory slot is empty"));
        state.ui.message_log.add_message(error_msg);
        return;
    }

    let item_id = item_id.unwrap();
    let item = state.layer.items.get(&item_id).unwrap();

    let target = match item.kind {
        ItemKind::Active(_) => state.ui.viewport.get_selected_world_square(),
        ItemKind::Passive => {
            let error_msg = MessageType::Error(String::from("Can't activate passive items"));
            state.ui.message_log.add_message(error_msg);
            return;
        }
    };

    let action_details = UseItemAction {
        inventory_slot: inventory_slot.expect("slot should have been checked to exist"),
        target,
    };

    let cur_action = Action::UseItem(action_details);
    let msg = Message::Act(cur_action);

    network::send_message(outgoing_tx, msg);
}
