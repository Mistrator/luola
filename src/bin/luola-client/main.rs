use crate::terminal::Terminal;
use crate::ui::UI;
use luola::constants;
use luola::world::Layer;
use std::{thread, time};

mod actions;
mod input;
mod network;
mod terminal;
mod ui;

pub struct GameState {
    layer: Layer,
    ui: UI,
    acting_creature: Option<u128>,
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
        acting_creature: None,
    };

    loop {
        input::handle_input(&input_rx, &outgoing_tx, &mut state);

        network::handle_messaging(&incoming_rx, &outgoing_tx, &mut state);

        let rendered_ui = state.ui.render(&state.layer);
        terminal.next_frame.paste(&rendered_ui, 0, 0);
        terminal.render_next();

        let delay = time::Duration::from_millis(16);
        thread::sleep(delay);
    }
}
