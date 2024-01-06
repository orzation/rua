#![allow(dead_code)]

use termion::{clear, color, cursor, style};

use crate::{config, map};

/// The position of 2d canvas (x, y).
pub struct Pos(pub u16, pub u16);

/// Clean all outputs on the screen.
pub fn clean_output() {
    print!("{}", clear::All)
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
    Pos(x, y + w + 1)
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
        if focus_idx == idx {
            print!(
                "{}{}{} {}.{} {}",
                color::Fg(color::Black),
                color::Bg(color::White),
                cursor::Goto(x + 1, y + 1 + idx as u16),
                enum_char,
                v,
                style::Reset
            );
        } else {
            print!(
                "{} {}.{} ",
                cursor::Goto(x + 1, y + 1 + idx as u16),
                enum_char,
                v
            );
        }
    });
    Ok(Pos(x + 1 + max_width as u16, y + 1 + max_height as u16))
}

pub fn show_map(pos: &Pos, game_conf: config::GameConfig, map: &Vec<map::Cell>, show_all: bool) {
    let Pos(x, y) = *pos;
    draw_border(&pos, game_conf.height, game_conf.width);
    map.iter().enumerate().for_each(|(idx, cell)| {
        let (dx, dy) = (
            (idx % game_conf.width) as u16,
            (idx / game_conf.width) as u16,
        );
        let symbol = if show_all {
            cell.get_content_symbol()
        } else {
            cell.to_string()
        };
        print!("{}{}", cursor::Goto(x + dx, y + dy), symbol);
    })
}

// Some words that said by ferris.
const SAYS_DIFFICULTIES: [&'static str; 3] = ["Eazy as fuck.", "It's OK.", "Really?"];
const SAYS_QUIT: &str = "Bye";
const SAYS_START: &str = "Ready?";
const SAYS_WIN: [&'static str; 2] = ["Win!", "WOW, Genius!"];
const SAYS_LOSE: [&'static str; 2] = ["BOOM, You Lose!", "BOOM, Loser!"];
const SAYS_MINE: [&'static str; 2] = ["You're safe", "Rua!!!"];
const SAYS_FLAG: [&'static str; 2] = ["Good Flag.", "You can't flag here."];
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
    print!("{}        \\", cursor::Goto(x, y + 3));
    print!("{}         \\", cursor::Goto(x, y + 4));
    print!("{}            _~^~^~_", cursor::Goto(x, y + 5));
    print!(
        "{}        \\) /  {} {}  \\ (/",
        cursor::Goto(x, y + 6),
        leye,
        reye
    );
    print!("{}          '_   {}   _'", cursor::Goto(x, y + 7), mouth);
    print!(
        "{}          / '-----' \\{}",
        cursor::Goto(x, y + 8),
        style::Reset
    );
    return Pos(x, y + 9);
}

pub fn ferris_say_start(pos: &Pos) -> Pos {
    draw_ferris_with(pos, SAYS_START, "3", "0", "0")
}

pub fn ferris_say_win(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_WIN[idx], "3", "^", "^")
}

pub fn ferris_say_lose(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_LOSE[idx], "x", "#", "#")
}

pub fn ferris_say_mine(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_MINE[idx], "o", "0", "0")
}

pub fn ferris_say_flag(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_FLAG[idx], "u", "o", "o")
}

pub fn ferris_say_move(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_MOVE[idx], "r", "C", "C")
}

pub fn ferris_say_quit(pos: &Pos) -> Pos {
    draw_ferris_with(pos, SAYS_QUIT, "o", "-", "-")
}

pub fn ferris_say_difficulty(pos: &Pos, idx: usize) -> Pos {
    draw_ferris_with(pos, SAYS_DIFFICULTIES[idx], "w", "o", "o")
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
