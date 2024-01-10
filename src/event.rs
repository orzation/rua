use std::io::{Stdin, Stdout, Write};
use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
use termion::cursor;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::RawTerminal;
use tokio::time::interval;

use crate::{config, map};
use crate::{
    config::{GameMode, GlobleConfig},
    draw::{self},
};

pub fn entry_event(
    init_pos: &draw::Pos,
    key_config: &GlobleConfig,
    stdin: &Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
) -> Option<config::GameConfig> {
    let mut selected_item_idx = 0;
    let difficultis = vec![GameMode::Simple, GameMode::Normal, GameMode::Hard];
    let difficultis_items: Vec<String> = difficultis.iter().map(|v| v.to_string()).collect();

    let pos = draw::ferris_says_difficulty(init_pos, selected_item_idx);
    let last_pos = draw::show_menu(&pos, &difficultis_items, selected_item_idx)
        .expect("cannot show start menu");

    stdout.flush().unwrap();
    for c in stdin.lock().events() {
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
                            draw::ferris_says_quit(&pos);
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
                draw::ferris_says_difficulty(init_pos, selected_item_idx);
                draw::show_menu(&pos, &difficultis_items, selected_item_idx)
                    .expect("cannot show start menu");
            }
            Event::Mouse(MouseEvent::Press(btn, x, y)) => {
                match btn {
                    MouseButton::Left => {
                        let x = (pos.0 + 1).max((last_pos.0 - 1).min(x));
                        let y = (pos.1 + 1).max((last_pos.1 - 1).min(y));
                        let selected_item_idx = (y - pos.1 - 1) as usize;

                        let pos = draw::ferris_says_difficulty(init_pos, selected_item_idx);
                        draw::show_menu(&pos, &difficultis_items, selected_item_idx)
                            .expect("cannot show start menu");
                        print!("{}", cursor::Goto(x, y));

                        let game_mode = config::GameMode::from_usize(selected_item_idx);
                        let game_config = config::GameConfig::from_game_mode(game_mode);
                        return Some(game_config);
                    }
                    MouseButton::WheelUp => selected_item_idx = selected_item_idx.wrapping_sub(1),
                    MouseButton::WheelDown => selected_item_idx = selected_item_idx.wrapping_add(1),
                    MouseButton::Right => {
                        draw::ferris_says_quit(init_pos);
                        return None;
                    }
                    _ => (),
                }
                selected_item_idx %= difficultis_items.len();
                draw::ferris_says_difficulty(init_pos, selected_item_idx);
                draw::show_menu(&pos, &difficultis_items, selected_item_idx)
                    .expect("cannot show start menu");
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
    None
}

#[tokio::main]
pub async fn game_event(
    key_conf: &config::GlobleConfig,
    game_conf: &config::GameConfig,
    stdin: &Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
    rng: &mut ThreadRng,
) -> draw::Pos {
    let mut init_mine = true;

    let mut flag_num = 0;
    let mut left_cover = game_conf.get_size();
    // init map
    let mut graph_map =
        vec![map::Cell::new(map::Content::Zero, map::Surface::Cover); game_conf.get_size()];
    let init_pos = draw::Pos(1, 1);
    let pos = draw::ferris_says_start(&init_pos);

    let interval_handle = tokio::spawn(time_record_event(pos.clone(), game_conf.clone()));
    let pos = draw::show_bomb_status(&pos, game_conf.bomb - flag_num);

    let last_pos = draw::show_map(&pos, game_conf, &graph_map, draw::ShowMode::Normal);
    let init_at = draw::Pos(pos.0 + 1, pos.1 + 1);
    let mut now_at = draw::Pos(pos.0 + 1, pos.1 + 1);
    print!("{}", cursor::Goto(now_at.0, now_at.1));

    stdout.lock().flush().unwrap();
    for c in stdin.lock().events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(key) => {
                draw::ferris_says_move(&init_pos, rng.gen_range(0..2));
                match key {
                    Key::Up => now_at.1 -= 1,
                    Key::Down => now_at.1 += 1,
                    Key::Left => now_at.0 -= 1,
                    Key::Right => now_at.0 += 1,
                    Key::Char(char) => {
                        if char == key_conf.up_key {
                            now_at.1 -= 1
                        } else if char == key_conf.down_key {
                            now_at.1 += 1
                        } else if char == key_conf.left_key {
                            now_at.0 -= 1
                        } else if char == key_conf.right_key {
                            now_at.0 += 1
                        } else if char == key_conf.mine_key {
                            match mine_event(
                                &init_pos,
                                &pos,
                                &init_at,
                                &now_at,
                                game_conf,
                                rng,
                                &mut init_mine,
                                &mut left_cover,
                                graph_map,
                            ) {
                                Some(res) => graph_map = res,
                                None => return last_pos,
                            }
                        } else if char == key_conf.flag_key {
                            graph_map = flag_event(
                                &init_pos,
                                &init_at,
                                &now_at,
                                game_conf,
                                init_mine,
                                graph_map,
                                &mut flag_num,
                            );
                        } else if char == key_conf.quit_key {
                            draw::ferris_says_quit(&init_pos);
                            return last_pos;
                        }
                    }
                    _ => (),
                }
                let x = (pos.0 + 1).max((last_pos.0 - 1).min(now_at.0));
                let y = (pos.1 + 1).max((last_pos.1 - 1).min(now_at.1));
                now_at = draw::Pos(x, y);
                print!("{}", cursor::Goto(x, y));
            }
            Event::Mouse(MouseEvent::Press(btn, x, y)) => {
                let x = (pos.0 + 1).max((last_pos.0 - 1).min(x));
                let y = (pos.1 + 1).max((last_pos.1 - 1).min(y));
                now_at = draw::Pos(x, y);

                match btn {
                    MouseButton::Left => match mine_event(
                        &init_pos,
                        &pos,
                        &init_at,
                        &now_at,
                        game_conf,
                        rng,
                        &mut init_mine,
                        &mut left_cover,
                        graph_map,
                    ) {
                        Some(res) => graph_map = res,
                        None => return last_pos,
                    },
                    MouseButton::Right => {
                        graph_map = flag_event(
                            &init_pos,
                            &init_at,
                            &now_at,
                            game_conf,
                            init_mine,
                            graph_map,
                            &mut flag_num,
                        );
                    }
                    _ => (),
                }
                print!("{}", cursor::Goto(now_at.0, now_at.1));
            }
            _ => (),
        }
        stdout.lock().flush().unwrap();
    }
    match tokio::join!(interval_handle) {
        _ => (),
    };
    last_pos
}

fn mine_event(
    init_pos: &draw::Pos,
    map_pos: &draw::Pos,
    init_at: &draw::Pos,
    now_at: &draw::Pos,
    conf: &config::GameConfig,
    rng: &mut ThreadRng,
    init_mine: &mut bool,
    left_cover: &mut usize,
    mut graph_map: Vec<map::Cell>,
) -> Option<Vec<map::Cell>> {
    draw::ferris_says_mine(init_pos, rng.gen_range(0..2));
    if *init_mine {
        *init_mine = false;
        graph_map = map::gen_map(init_at, now_at, conf, rng);
    }
    graph_map = map::mine_map(init_at, now_at, conf, graph_map, left_cover);
    let pos = (now_at.1 - init_at.1) as usize * conf.width + (now_at.0 - init_at.0) as usize;
    if let (map::Surface::Open, map::Content::Bomb) =
        (&graph_map[pos].surface, &graph_map[pos].content)
    {
        draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Lose);
        draw::ferris_says_lose(init_pos, rng.gen_range(0..2));
        return None;
    }
    if *left_cover == conf.bomb {
        let pos = draw::ferris_says_win(init_pos, rng.gen_range(0..2));
        draw::show_bomb_status(&pos, 0);
        draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Win);
        return None;
    }
    draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Normal);
    Some(graph_map)
}

fn flag_event(
    init_pos: &draw::Pos,
    init_at: &draw::Pos,
    now_at: &draw::Pos,
    conf: &config::GameConfig,
    init_mine: bool,
    graph_map: Vec<map::Cell>,
    flag_num: &mut usize,
) -> Vec<map::Cell> {
    if init_mine {
        draw::ferris_says_flag(init_pos, 0);
        return graph_map;
    }
    let (graph_map, ok) = map::flag_map(init_at, now_at, conf, graph_map, flag_num);
    let pos = draw::ferris_says_flag(init_pos, ok as usize);
    let pos = draw::show_bomb_status(&pos, conf.bomb - *flag_num);
    draw::show_map(&pos, conf, &graph_map, draw::ShowMode::Normal);

    graph_map
}

/// End menu event, return usize meanings:
/// 0: retry
/// 1: back
pub fn end_event(
    pos: &draw::Pos,
    key_conf: &config::GlobleConfig,
    game_config: &config::GameConfig,
    stdin: &Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
) -> usize {
    let init_pos = draw::Pos(1, 1);
    let pos = draw::Pos(pos.0 + 1, pos.1 - game_config.height as u16 - 1);
    let opts = vec!["Retry".to_string(), "Go Back".to_string()];
    let mut selected_item_idx = 0;
    let last_pos = draw::show_menu(&pos, &opts, selected_item_idx).expect("cannot show end menu");

    stdout.flush().unwrap();
    for c in stdin.lock().events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(key) => {
                match key {
                    Key::Up => selected_item_idx = selected_item_idx.wrapping_sub(1),
                    Key::Down => selected_item_idx = selected_item_idx.wrapping_add(1),
                    Key::Char(char) => {
                        if char == key_conf.up_key {
                            selected_item_idx = selected_item_idx.wrapping_sub(1)
                        } else if char == key_conf.down_key {
                            selected_item_idx = selected_item_idx.wrapping_add(1)
                        } else if char == key_conf.quit_key {
                            draw::ferris_says_quit(&init_pos);
                            return 255;
                        } else if char == key_conf.mine_key {
                            return selected_item_idx;
                        }
                    }
                    _ => (),
                }
                selected_item_idx %= opts.len();
                draw::ferris_says_end(&init_pos, selected_item_idx);
                draw::show_menu(&pos, &opts, selected_item_idx).expect("cannot show end menu");
            }
            Event::Mouse(MouseEvent::Press(btn, x, y)) => {
                match btn {
                    MouseButton::Left => {
                        let x = (pos.0 + 1).max((last_pos.0 - 1).min(x));
                        let y = (pos.1 + 1).max((last_pos.1 - 1).min(y));
                        let selected_item_idx = (y - pos.1 - 1) as usize;

                        draw::ferris_says_end(&init_pos, selected_item_idx);
                        draw::show_menu(&pos, &opts, selected_item_idx)
                            .expect("cannot show end menu");
                        print!("{}", cursor::Goto(x, y));

                        return selected_item_idx;
                    }
                    MouseButton::WheelUp => selected_item_idx = selected_item_idx.wrapping_sub(1),
                    MouseButton::WheelDown => selected_item_idx = selected_item_idx.wrapping_add(1),
                    MouseButton::Right => {
                        draw::ferris_says_quit(&init_pos);
                        return 255;
                    }
                    _ => (),
                }
                selected_item_idx %= opts.len();
                draw::ferris_says_end(&init_pos, selected_item_idx);
                draw::show_menu(&pos, &opts, selected_item_idx).expect("cannot show end menu");
            }
            _ => (),
        }
        stdout.flush().unwrap();
    }
    255
}

pub async fn time_record_event(pos: draw::Pos, conf: config::GameConfig) {
    let mut interval = interval(Duration::from_secs(1));
    let mut time = 0;
    loop {
        interval.tick().await;
        time += 1;
        draw::show_time_status(&pos, &conf, time);
    }
}
