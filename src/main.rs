use std::io::{stdin, stdout};

use rua::{
    config, draw,
    event::{self},
};
use termion::{input::MouseTerminal, raw::IntoRawMode};

fn main() {
    let key_config = config::globle_config_from_env();
    let init_pos = draw::Pos(1, 1);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let mut rng = rand::thread_rng();

    // Splash screen with a typewriter-style greeting animation.
    draw::clean_output();
    draw::type_writer(&init_pos, "Welcome to Rua Minesweeper!");
    std::thread::sleep(std::time::Duration::from_millis(500));

    // All event start
    'start: loop {
        // Choose a difficulty.
        draw::clean_output();
        let option = event::entry_event(&init_pos, &key_config, &stdin, &mut stdout);
        if let None = option {
            // exit from press q.
            return;
        }

        let game_config = option.unwrap();

        'game: loop {
            // Game start.
            draw::clean_output();
            let pos = event::game_event(&key_config, &game_config, &stdin, &mut stdout, &mut rng);

            // End menu.
            match event::end_event(&pos, &key_config, &game_config, &stdin, &mut stdout) {
                0 => continue,
                1 => break 'game,
                _ => break 'start,
            }
        }
    }
    draw::put_cursor_bottom();
}
