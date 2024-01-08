use std::io::{Stdin, Stdout, Write};

use rand::rngs::ThreadRng;
use rand::Rng;
use termion::cursor;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::RawTerminal;

use crate::draw::ferris_says_quit;
use crate::{config, map};
use crate::{
    config::{GameMode, GlobleConfig},
    draw::{self},
};

pub fn entry_event(
    init_pos: draw::Pos,
    key_config: &GlobleConfig,
    stdin: &Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
) -> Option<config::GameConfig> {
    let mut selected_item_idx = 0;
    let difficultis = vec![GameMode::Simple, GameMode::Normal, GameMode::Hard];
    let difficultis_items: Vec<String> = difficultis.iter().map(|v| v.to_string()).collect();

    let pos = draw::ferris_says_difficulty(&init_pos, selected_item_idx);
    let last_pos =
        draw::show_menu(&pos, &difficultis_items, selected_item_idx).expect("cannot show menu");

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
                            ferris_says_quit(&pos);
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
                let pos = draw::ferris_says_difficulty(&init_pos, selected_item_idx);
                draw::show_menu(&pos, &difficultis_items, selected_item_idx)
                    .expect("cannot show menu");
            }
            Event::Mouse(MouseEvent::Press(btn, x, y)) => match btn {
                MouseButton::Left => {
                    let x = (pos.0 + 1).max((last_pos.0 - 1).min(x));
                    let y = (pos.1 + 1).max((last_pos.1 - 1).min(y));
                    let selected_item_idx = y - pos.1 - 1;

                    let pos = draw::ferris_says_difficulty(&init_pos, selected_item_idx as usize);
                    draw::show_menu(&pos, &difficultis_items, selected_item_idx as usize)
                        .expect("cannot show menu");
                    print!("{}", cursor::Goto(x, y));

                    let game_mode = config::GameMode::from_usize(selected_item_idx as usize);
                    let game_config = config::GameConfig::from_game_mode(game_mode);
                    return Some(game_config);
                }
                _ => {
                    ferris_says_quit(&pos);
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
    key_conf: &config::GlobleConfig,
    game_conf: &config::GameConfig,
    stdin: &Stdin,
    stdout: &mut MouseTerminal<RawTerminal<Stdout>>,
    rng: &mut ThreadRng,
) {
    let mut init_mine = true;

    // init map
    let mut graph_map =
        vec![map::Cell::new(map::Content::Zero, map::Surface::Cover); game_conf.get_size()];
    let init_pos = draw::Pos(1, 1);
    let pos = draw::ferris_says_start(&init_pos);

    // game status.
    // todo
    let pos = draw::Pos(pos.0, pos.1 + 1);

    let last_pos = draw::show_map(&pos, game_conf, &graph_map, draw::ShowMode::Normal);
    let init_at = draw::Pos(pos.0 + 1, pos.1 + 1);
    let mut now_at = draw::Pos(pos.0 + 1, pos.1 + 1);
    print!("{}", cursor::Goto(now_at.0, now_at.1));
    let mut left_cover = game_conf.get_size();
    let mut flag_num = 0;

    stdout.flush().unwrap();
    for c in stdin.lock().events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(key) => {
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
                                None => return,
                            }
                        } else if char == key_conf.flag_key {
                            graph_map = flag_event(
                                &init_pos,
                                &pos,
                                &init_at,
                                &now_at,
                                game_conf,
                                init_mine,
                                graph_map,
                                &mut flag_num,
                            );
                        } else if char == key_conf.quit_key {
                            draw::ferris_says_quit(&init_pos);
                            return;
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
                        None => return,
                    },
                    MouseButton::Right => {
                        graph_map = flag_event(
                            &init_pos,
                            &pos,
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
        stdout.flush().unwrap();
    }
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
        draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Win);
        draw::ferris_says_win(init_pos, rng.gen_range(0..2));
        return None;
    }
    draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Normal);
    Some(graph_map)
}

fn flag_event(
    init_pos: &draw::Pos,
    map_pos: &draw::Pos,
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
    draw::ferris_says_flag(init_pos, ok as usize);
    draw::show_map(map_pos, conf, &graph_map, draw::ShowMode::Normal);
    graph_map
}
