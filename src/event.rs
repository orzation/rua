use std::io::{Stdin, Stdout, Write};

use termion::cursor;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::RawTerminal;

use crate::config;
use crate::draw::ferris_say_quit;
use crate::{
    config::{GameMode, GlobleConfig},
    draw::{self, Pos},
};

pub fn entry_event(
    init_pos: Pos,
    key_config: GlobleConfig,
    stdin: Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
) -> Option<config::GameConfig> {
    let mut selected_item_idx = 0;
    let difficultis = vec![GameMode::Simple, GameMode::Normal, GameMode::Hard];
    let difficultis_items: Vec<String> = difficultis.iter().map(|v| v.to_string()).collect();

    let pos = draw::ferris_say_difficulty(&init_pos, selected_item_idx);
    let last_pos =
        draw::show_menu(&pos, &difficultis_items, selected_item_idx).expect("cannot show menu");

    stdout.flush().unwrap();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Up => selected_item_idx = selected_item_idx.wrapping_sub(1),
                    Key::Down => selected_item_idx = selected_item_idx.wrapping_add(1),
                    Key::Char(char) => {
                        if char == key_config.up_key {
                            selected_item_idx = selected_item_idx.wrapping_sub(1)
                        } else if char == key_config.down_key {
                            selected_item_idx = selected_item_idx.wrapping_add(1)
                        } else if char == key_config.quit_key {
                            ferris_say_quit(&pos);
                            return None;
                        } else if char == key_config.mine_key {
                            let game_mode = config::GameMode::from_usize(selected_item_idx);
                            let game_config = config::GameConfig::from_game_mode(game_mode);
                            return Some(game_config);
                        }
                    }
                    _ => (),
                }
                selected_item_idx %= difficultis_items.len();
                let pos = draw::ferris_say_difficulty(&init_pos, selected_item_idx);
                draw::show_menu(&pos, &difficultis_items, selected_item_idx)
                    .expect("cannot show menu");
            }
            Event::Mouse(MouseEvent::Press(btn, x, y)) => match btn {
                MouseButton::Left => {
                    let x = (pos.0 + 1).max((last_pos.0 - 1).min(x));
                    let y = (pos.1 + 1).max((last_pos.1 - 1).min(y));
                    let selected_item_idx = y - pos.1 - 1;

                    let pos = draw::ferris_say_difficulty(&init_pos, selected_item_idx as usize);
                    draw::show_menu(&pos, &difficultis_items, selected_item_idx as usize)
                        .expect("cannot show menu");
                    print!("{}", cursor::Goto(x, y));

                    let game_mode = config::GameMode::from_usize(selected_item_idx as usize);
                    let game_config = config::GameConfig::from_game_mode(game_mode);
                    return Some(game_config);
                }
                _ => {
                    ferris_say_quit(&pos);
                    return None;
                }
            },
            _ => (),
        }
        stdout.flush().unwrap();
    }
    None
}

pub fn game_event(
    conf: config::GameConfig,
    stdin: Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
) {
    // init map
    stdout.flush().unwrap();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key() => {}
            Event::Mouse() => {}
            _ => (),
        }
    }
    stdout.flush().unwrap();
}
