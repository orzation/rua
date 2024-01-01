#![allow(dead_code)]

use termion::{clear, color, cursor, style};

/// The position of 2d canvas (x, y).
pub struct Pos(u16, u16);

/// Draw a box border with positon and size, just like this:
/// ```text
/// ┌──────────┐
/// │          │
/// │          │
/// │          │
/// └──────────┘
/// ```
fn draw_border(pos: Pos, height: usize, width: usize) -> Pos {
    let Pos(x, y) = pos;
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
    print!("{}┘", cursor::Goto(x + h, y + w));
    print!("{}", cursor::Goto(x, y + w + 1));
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
pub fn show_menu(pos: Pos, opts: Vec<&'static str>, focus_idx: usize) -> Result<Pos, &str> {
    if opts.len() > 26 {
        return Err("The options should be between 1 and 26.");
    }
    let Pos(x, y) = pos;
    let max_height = opts.len() + 2;
    let max_word_len = opts
        .iter()
        .fold("", |acc, e| if e.len() > acc.len() { e } else { acc })
        .len();
    let max_width = max_word_len + 4;
    draw_border(pos, max_height, max_width);
    // todo!();
    Ok(Pos(x, y + max_height as u16))
}

// Some words that said by ferris.
const SAYS_START: &str = "Ready?";
const SAYS_WIN1: &str = "Win!";
const SAYS_WIN2: &str = "WOW, Genius!";
const SAYS_LOSE1: &str = "BOOM, You Lose!";
const SAYS_LOSE2: &str = "BOOM, Loser!";
const SAYS_MINE1: &str = "You're safe";
const SAYS_MINE2: &str = "Rua!!!";
const SAYS_FLAG1: &str = "Good Flag.";
const SAYS_FLAG2: &str = "You can't flag here.";
const SAYS_MOVE1: &str = "Move Move Move!";
const SAYS_MOVE2: &str = "Are you sure?";

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
fn draw_ferris_with(pos: Pos, words: &str, mouth: &str, leye: &str, reye: &str) -> Pos {
    let len = words.len();
    let Pos(x, y) = pos;
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

#[cfg(test)]
mod test {
    use crate::draw::draw_ferris_with;

    use super::draw_border;
    use super::Pos;

    #[test]
    fn print_border() {
        let Pos(x, y) = draw_border(Pos(1, 1), 9, 9);
        assert_eq!(1, x);
        assert_eq!(12, y);
    }

    #[test]
    fn print_ferris() {
        let Pos(x, y) = draw_ferris_with(Pos(1, 1), "just for test", "v", "O", "o");
        assert_eq!(1, x);
        assert_eq!(10, y);
    }
}
