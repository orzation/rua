#![allow(dead_code)]

use std::iter;

use termion::{clear, color, cursor, style};

use crate::{config, map};

/// The position of 2d canvas (x, y).
#[derive(Clone)]
pub struct Pos(pub u16, pub u16);

/// Clean all outputs on the screen.
pub fn clean_output() {
    print!("{}", clear::All)
}

/// Fix cursor flash every where.
pub fn put_cursor_bottom() {
    let (height, _) = termion::terminal_size().unwrap();
    print!("{}", cursor::Goto(1, height));
}

/// Draw a box border with positon and size, just like this:
/// ```text
/// ┌──────────┐
/// │          │
/// │          │
/// │          │
/// └──────────┘
/// ```
fn draw_border(pos: &Pos, height: usize, width: usize) -> Pos {
    let Pos(x, y) = *pos;
    let (h, w) = (height as u16 + 1, width as u16 + 1);
    for i in 0..w {
        print!("{}─", cursor::Goto(x + i, y));
        print!("{}─", cursor::Goto(x + i, y + h));
    }
    for i in 0..h {
        print!("{}│", cursor::Goto(x, y + i));
        print!("{}│", cursor::Goto(x + w, y + i));
    }
    print!("{}┌", cursor::Goto(x, y));
    print!("{}┐", cursor::Goto(x + w, y));
    print!("{}└", cursor::Goto(x, y + h));
    print!("{}┘", cursor::Goto(x + w, y + h));
    Pos(x + w, y + h)
}

/// Draw a menu with given items(lenght limit: 26), just like this:
/// ```text
/// ┌─────────┐
/// │ a.one   │
/// │ b.two   │
/// │ c.three │
/// │ d.four  │
/// └─────────┘
/// ```
pub fn show_menu(pos: &Pos, opts: &Vec<String>, focus_idx: usize) -> Result<Pos, &'static str> {
    if opts.len() > 26 {
        return Err("The length of options should be between 1 and 26.");
    }
    if focus_idx >= opts.len() {
        return Err("The focus idx should not be bigger then length of options");
    }
    let Pos(x, y) = *pos;
    let max_height = opts.len();
    let max_word_len = opts
        .iter()
        .fold("", |acc, e| if e.len() > acc.len() { e } else { acc })
        .len();
    let max_width = max_word_len + 4;
    draw_border(pos, max_height, max_width);
    opts.iter().enumerate().for_each(|(idx, v)| {
        let enum_char = (('a' as u8) + idx as u8) as char;
        let space: String = iter::repeat(" ").take(max_word_len - v.len()).collect();
        if focus_idx == idx {
            print!(
                "{}{}{} {}.{} {}{}",
                color::Fg(color::Black),
                color::Bg(color::White),
                cursor::Goto(x + 1, y + 1 + idx as u16),
                enum_char,
                v,
                space,
                style::Reset
            );
        } else {
            print!(
                "{} {}.{} {}",
                cursor::Goto(x + 1, y + 1 + idx as u16),
                enum_char,
                v,
                space
            );
        }
    });
    put_cursor_bottom();
    Ok(Pos(x + 1 + max_width as u16, y + 1 + max_height as u16))
}

pub enum ShowMode {
    Normal,
    All,
    Win,
    Lose,
}

/// Show map, show_mode:
/// - normal show
/// - show all
/// - show win
/// - show lose
pub fn show_map(
    pos: &Pos,
    game_conf: &config::GameConfig,
    map: &Vec<map::Cell>,
    show_mode: ShowMode,
) -> Pos {
    let Pos(x, y) = *pos;
    let ret_pos = draw_border(&pos, game_conf.height, game_conf.width);
    map.iter().enumerate().for_each(|(idx, cell)| {
        let (dx, dy) = (
            (idx % game_conf.width) as u16,
            (idx / game_conf.width) as u16,
        );
        let symbol = match show_mode {
            ShowMode::Normal => cell.to_string(),
            ShowMode::All => cell.get_content_symbol(),
            ShowMode::Win => match cell.content {
                map::Content::Bomb => map::Surface::Flag.to_string(),
                _ => cell.get_content_symbol(),
            },
            ShowMode::Lose => match cell.content {
                map::Content::Bomb => cell.get_content_symbol(),
                _ => cell.to_string(),
            },
        };
        print!("{}{}", cursor::Goto(x + dx + 1, y + dy + 1), symbol);
    });
    ret_pos
}

// Some words that said by ferris.
const SAYS_DIFFICULTIES: [&'static str; 3] = ["Eazy as fuck.", "It's OK.", "Really?"];
const SAYS_END: [&'static str; 2] = ["One more time!", "Back to menu."];
const SAYS_QUIT: &str = "Bye~";
const SAYS_START: &str = "Ready?";
const SAYS_WIN: [&'static str; 2] = ["Win!", "WOW, Genius!"];
const SAYS_LOSE: [&'static str; 2] = ["You Lose!", "BOOM!"];
const SAYS_MINE: [&'static str; 2] = ["You're safe.", "Rua!!!"];
const SAYS_FLAG: [&'static str; 2] = ["You can't flag here.", "Good Flag."];
const SAYS_MOVE: [&'static str; 2] = ["Move Move Move!", "Are you sure?"];

///  Draw a ferris with specific str, just like this:
///  ```text
///  __________________________
/// < Hello fellow Rustaceans! >
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// ```
fn draw_ferris_with(pos: &Pos, words: &str, mouth: &str, leye: &str, reye: &str) -> Pos {
    let len = words.len();
    let Pos(x, y) = *pos;
    print!("{}{}", cursor::Goto(x, y + 8), clear::BeforeCursor);
    print!("{}{}", cursor::Goto(x, y), color::Fg(color::LightRed));
    for i in 0..len + 2 {
        print!("{}{}", cursor::Goto(x + 1 + i as u16, y), '_');
    }
    print!("{}<", cursor::Goto(x, y + 1));
    print!(" {} >", words);
    print!("{} ", cursor::Goto(x, y + 2));
    for i in 0..len + 2 {
        print!("{}{}", cursor::Goto(x + 1 + i as u16, y + 2), '-');
    }
    print!("{}    \\", cursor::Goto(x, y + 3));
    print!("{}     \\", cursor::Goto(x, y + 4));
    print!("{}    _~^~^~_", cursor::Goto(x, y + 5));
    print!(
        "{}\\) /  {} {}  \\ (/",
        cursor::Goto(x, y + 6),
        leye,
        reye
    );
    print!("{}  '_   {}   _'", cursor::Goto(x, y + 7), mouth);
    print!(
        "{}  / '-----' \\{}",
        cursor::Goto(x, y + 8),
        style::Reset
    );
    return Pos(x, y + 9);
}

pub fn ferris_says_start(pos: &Pos) -> Pos {
    draw_ferris_with(pos, SAYS_START, "v", "0", "0")
}

pub fn ferris_says_win(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_WIN[idx % SAYS_WIN.len()], "3", "^", "^")
}

pub fn ferris_says_lose(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_LOSE[idx % SAYS_LOSE.len()], "x", "#", "#")
}

pub fn ferris_says_mine(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_MINE[idx % SAYS_MINE.len()], "o", "0", "0")
}

pub fn ferris_says_flag(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_FLAG[idx % SAYS_FLAG.len()], "u", "o", "o")
}

pub fn ferris_says_move(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_MOVE[idx % SAYS_MOVE.len()], "r", "6", "6")
}

pub fn ferris_says_quit(pos: &Pos) -> Pos {
    draw_ferris_with(pos, SAYS_QUIT, "o", "-", "-")
}

pub fn ferris_says_difficulty(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(
        pos,
        SAYS_DIFFICULTIES[idx % SAYS_DIFFICULTIES.len()],
        "w",
        "o",
        "o",
    )
}

pub fn ferris_says_end(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_END[idx % SAYS_END.len()], "v", "$", "$")
}

pub fn show_bomb_status(pos: &Pos, flag_num: usize) -> Pos {
    print!("{}{:02}", cursor::Goto(pos.0, pos.1), flag_num);
    Pos(pos.0, pos.1 + 1)
}

pub fn show_time_status(pos: &Pos, conf: &config::GameConfig, time: usize) -> Pos {
    print!(
        "{}{:03}",
        cursor::Goto(pos.0 + conf.width as u16 - 1, pos.1),
        time
    );
    Pos(pos.0, pos.1 + 1)
}

#[cfg(test)]
mod test {
    use super::draw_border;
    use super::draw_ferris_with;
    use super::show_menu;
    use super::Pos;

    #[test]
    fn print_border() {
        let Pos(x, y) = draw_border(&Pos(1, 1), 9, 9);
        assert_eq!(1, x);
        assert_eq!(12, y);
    }

    #[test]
    fn print_ferris() {
        let Pos(x, y) = draw_ferris_with(&Pos(1, 1), "just for test", "v", "O", "o");
        assert_eq!(1, x);
        assert_eq!(10, y);
    }

    #[test]
    fn print_menu() {
        let opts = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
        ];
        show_menu(&Pos(1, 1), &opts, 2).unwrap();
    }
}
