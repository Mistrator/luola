use crate::terminal::Terminal;
use crate::ui::UI;
use luola::constants;
use luola::messages::CreatureOwner;
use luola::world::Layer;
use std::collections::HashMap;
use std::{thread, time};

mod actions;
mod input;
mod network;
mod terminal;
mod ui;

pub struct GameState {
    layer: Layer,
    ui: UI,
    creature_owners: HashMap<u128, CreatureOwner>,
    acting_creature: Option<u128>,
    player_id: u128,
}

impl GameState {
    pub fn this_player_controls(&self, creature_id: u128) -> bool {
        let owner = self.creature_owners.get(&creature_id).unwrap();
        match owner {
            CreatureOwner::Player(owner_id) => *owner_id == self.player_id,
            CreatureOwner::AI => false,
        }
    }

    pub fn some_player_controls(&self, creature_id: u128) -> bool {
        let owner = self.creature_owners.get(&creature_id).unwrap();
        match owner {
            CreatureOwner::Player(_) => true,
            CreatureOwner::AI => false,
        }
    }
}

fn main() {
    let server_address = String::from("127.0.0.1:26988");
    let (tx_stream, rx_stream) = network::open_stream(server_address);

    let incoming_rx = network::spawn_incoming_thread(rx_stream);
    let outgoing_tx = network::spawn_outgoing_thread(tx_stream);

    let player_id = network::join_game(&outgoing_tx, &incoming_rx);

    let width: usize = 162;
    let height: usize = 48;
    let mut terminal = Terminal::init(width, height);
    let ui = UI::new(width, height);
    let input_rx = input::spawn_polling_thread();

    // Use a dummy layer until we receive the actual one so that we
    // don't have to deal with Option<Layer> everywhere
    let layer = Layer::new(constants::WORLD_HEIGHT, constants::WORLD_WIDTH);

    let mut state = GameState {
        layer,
        ui,
        creature_owners: HashMap::new(),
        acting_creature: None,
        player_id,
    };

    loop {
        input::handle_input(&input_rx, &outgoing_tx, &mut state);

        network::handle_messaging(&incoming_rx, &outgoing_tx, &mut state);

        let rendered_ui = UI::render(&mut state);
        terminal.next_frame.paste(&rendered_ui, 0, 0);
        terminal.render_next();

        let delay = time::Duration::from_millis(16);
        thread::sleep(delay);
    }
}
