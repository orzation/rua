use std::io::{stdin, stdout};

use rua::{config, draw, event};
use termion::{input::MouseTerminal, raw::IntoRawMode};

fn main() {
    let key_config = config::globle_config_from_env();
    let init_pos = draw::Pos(1, 1);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    draw::clean_output();

    // Choose a difficulty.
    let option = event::entry_event(init_pos, &key_config, &stdin, &mut stdout);
    if let None = option {
        // exit from press q.
        return;
    }
    draw::clean_output();

    // Game start.
    let game_config = option.unwrap();
    let mut rng = rand::thread_rng();
    event::game_event(&key_config, &game_config, &stdin, &mut stdout, &mut rng)
}
