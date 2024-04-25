use crate::terminal::canvas::Canvas;
use crate::terminal::color::Color;
use crate::terminal::styled_char::Style;
use crate::terminal::Terminal;
use crate::ui::UI;
use luola::constants;
use luola::creature::action::{Action, UseItemAction};
use luola::item::targeting::Target;
use luola::messages::*;
use luola::player::Player;
use luola::world::Layer;
use std::net::TcpStream;
use std::{thread, time};

mod terminal;
mod ui;

fn act(player: &mut Player, enemy: u128) {
    let action_details = UseItemAction {
        inventory_slot: 0,
        target: Target::Creatures(vec![enemy]),
    };

    let cur_action = Action::UseItem(action_details);
    let msg = Message::Act(cur_action);

    // println!("trying to act");
    luola::net::send(&mut player.socket, &msg);

    let response = luola::net::receive(&mut player.socket);
    /*match response {
        Message::ActionOk => println!("action ok"),
        Message::ActionError(err) => println!("action rejected: {}", err.message),
        other => println!("received unexpected message type: {}", other),
    }*/

    receive_game_state(&mut player.socket);
}

fn receive_game_state(stream: &mut TcpStream) -> Option<Layer> {
    let msg = luola::net::receive(stream);

    match msg {
        Message::GameState(state) => {
            let layer: Layer = Layer::reconstruct(state.grid, state.creatures, state.items);
            // println!("received game state");
            // println!("{}", layer);

            return Some(layer);
        }
        other => {
            // println!("received unexpected message type: {}", other);
        }
    }

    None
}

fn join(stream: &mut TcpStream) -> u128 {
    let join_msg = Message::Join(JoinMsg {
        version: constants::get_version(),
        character_name: String::from("testcharacter"),
    });

    luola::net::send(stream, &join_msg);

    let response = luola::net::receive(stream);
    match response {
        Message::JoinOk(ok_msg) => {
            println!(
                "successfully joined the game with player id {}",
                ok_msg.player_id
            );

            return ok_msg.player_id;
        }
        Message::JoinError(err) => {
            panic!("failed to join the game: {}", err.message);
        }
        other => {
            panic!("received unexpected message type: {}", other);
        }
    }
}

fn open_stream() -> TcpStream {
    let addr = "127.0.0.1:26988";
    let stream = match TcpStream::connect(addr) {
        Ok(s) => s,
        Err(e) => panic!("failed to connect to {}: {:?}", addr, e),
    };

    stream
}

fn play(player: &mut Player, enemy: u128) {
    loop {
        let delay = time::Duration::from_millis(1000);

        thread::sleep(delay);
        act(player, enemy);

        thread::sleep(delay);
        act(player, enemy);

        for _ in 0..4 {
            receive_game_state(&mut player.socket);
            receive_game_state(&mut player.socket);
        }
    }
}

fn main() {
    let mut stream = open_stream();
    let player_id = join(&mut stream);

    let mut player = Player::build_existing(stream, player_id);
    let layer = receive_game_state(&mut player.socket).unwrap();

    let mut enemy_id: u128 = 0;
    for (id, _) in layer.creatures {
        if id != player_id {
            enemy_id = id;
        }
    }

    let width: usize = 160;
    let height: usize = 48;
    let mut terminal = Terminal::init(width, height);
    let mut ui = UI::new(width, height);
    ui.select_creature(enemy_id);

    // play(&mut player, enemy_id);

    loop {
        let delay = time::Duration::from_millis(1000);

        thread::sleep(delay);
        act(&mut player, enemy_id);

        thread::sleep(delay);
        act(&mut player, enemy_id);

        for _ in 0..4 {
            receive_game_state(&mut player.socket);
            let layer = receive_game_state(&mut player.socket).unwrap();

            let rendered_ui = ui.render(&layer);
            terminal.next_frame.paste(&rendered_ui, 0, 0);
            terminal.render_next();
        }
    }
}
