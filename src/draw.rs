#![allow(dead_code)]

use std::io::{self, Write};
use std::iter;
use std::thread::sleep;
use std::time::Duration;

use rand::rngs::ThreadRng;
use rand::Rng;
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
    map: &[map::Cell],
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

/// Play an expanding-ring explosion animation centered on the mine that was
/// triggered: bombs are revealed ring by ring (Chebyshev distance) with a
/// flickering red/yellow flash, before settling into the final "lose" board.
pub fn animate_explosion(
    pos: &Pos,
    conf: &config::GameConfig,
    map: &[map::Cell],
    hit_idx: usize,
    rng: &mut ThreadRng,
) {
    let Pos(x, y) = *pos;
    let (hy, hx) = (hit_idx / conf.width, hit_idx % conf.width);
    let max_ring = conf.height.max(conf.width);

    for ring in 0..=max_ring {
        let mut frame = String::new();
        for (idx, cell) in map.iter().enumerate() {
            if !matches!(cell.content, map::Content::Bomb) {
                continue;
            }
            let (cy, cx) = (idx / conf.width, idx % conf.width);
            let dist = (cy as isize - hy as isize)
                .unsigned_abs()
                .max((cx as isize - hx as isize).unsigned_abs());
            if dist != ring {
                continue;
            }
            let (dx, dy) = ((idx % conf.width) as u16, (idx / conf.width) as u16);
            let flash = if rng.gen_bool(0.5) {
                format!(
                    "{}{}*{}",
                    color::Fg(color::Yellow),
                    color::Bg(color::Red),
                    style::Reset
                )
            } else {
                format!(
                    "{}{}*{}",
                    color::Fg(color::White),
                    color::Bg(color::LightRed),
                    style::Reset
                )
            };
            frame.push_str(&format!(
                "{}{}",
                cursor::Goto(x + dx + 1, y + dy + 1),
                flash
            ));
        }
        if !frame.is_empty() {
            print!("{}", frame);
            io::stdout().flush().ok();
            sleep(Duration::from_millis(45));
        }
    }
    show_map(pos, conf, map, ShowMode::Lose);
    io::stdout().flush().ok();
}

/// Play a diagonal sparkle wave sweeping across the board on victory, before
/// settling into the final "win" board (flags shown on every bomb).
pub fn animate_victory(
    pos: &Pos,
    conf: &config::GameConfig,
    map: &[map::Cell],
    rng: &mut ThreadRng,
) {
    let Pos(x, y) = *pos;
    let max_diag = conf.height + conf.width;

    for diag in 0..max_diag {
        let mut frame = String::new();
        for idx in 0..map.len() {
            let (cy, cx) = (idx / conf.width, idx % conf.width);
            if cy + cx != diag {
                continue;
            }
            let (dx, dy) = (cx as u16, cy as u16);
            let symbol = match rng.gen_range(0..4) {
                0 => format!("{}✦{}", color::Fg(color::LightYellow), style::Reset),
                1 => format!("{}✧{}", color::Fg(color::LightCyan), style::Reset),
                2 => format!("{}*{}", color::Fg(color::LightGreen), style::Reset),
                _ => format!("{}+{}", color::Fg(color::LightMagenta), style::Reset),
            };
            frame.push_str(&format!(
                "{}{}",
                cursor::Goto(x + dx + 1, y + dy + 1),
                symbol
            ));
        }
        if !frame.is_empty() {
            print!("{}", frame);
            io::stdout().flush().ok();
            sleep(Duration::from_millis(18));
        }
    }
    show_map(pos, conf, map, ShowMode::Win);
    io::stdout().flush().ok();
}

/// Briefly flash the just-placed flag so placing/removing a flag gives some
/// tactile visual feedback instead of popping in instantly.
pub fn animate_flag_place(now_at: &Pos, map: &[map::Cell], pos: usize) {
    let Pos(x, y) = *now_at;
    for _ in 0..2 {
        print!(
            "{}{}{}P{}",
            cursor::Goto(x, y),
            style::Bold,
            color::Fg(color::Yellow),
            style::Reset
        );
        io::stdout().flush().ok();
        sleep(Duration::from_millis(55));
        print!("{}{}", cursor::Goto(x, y), map[pos]);
        io::stdout().flush().ok();
        sleep(Duration::from_millis(55));
    }
    print!(
        "{}{}{}P{}",
        cursor::Goto(x, y),
        style::Bold,
        color::Fg(color::Yellow),
        style::Reset
    );
    io::stdout().flush().ok();
}

/// Reveal `words` one character at a time for a lightweight typewriter
/// effect, used for the intro ferris greeting.
pub fn type_writer(pos: &Pos, words: &str) {
    let Pos(x, y) = *pos;
    print!("{}", cursor::Goto(x, y));
    for ch in words.chars() {
        print!("{}", ch);
        io::stdout().flush().ok();
        sleep(Duration::from_millis(28));
    }
}

// Ferris' lines, each paired with its own (mouth, left eye, right eye) so
// every quote gets a matching facial expression instead of one fixed face
// reused for every variant in a category.
type FerrisLine = (&'static str, &'static str, &'static str, &'static str);

const SAYS_DIFFICULTIES: [FerrisLine; 3] = [
    ("Ez pz, let's go~", "w", "-", "-"),  // chill smirk
    ("Alright, bring it!", "3", "o", "o"), // pumped up
    ("Whoa, bold much?", "o", "0", "0"),   // wide-eyed surprise
];
const SAYS_END: [FerrisLine; 3] = [
    ("One more time!", "v", "$", "$"), // excited
    ("Back to menu.", "o", "-", "-"),  // calm
    ("Called it quits?", "u", "o", "o"), // curious
];
const SAYS_QUIT: FerrisLine = ("Bye~", "u", "-", "-"); // waving goodbye, eyes closed
const SAYS_START: FerrisLine = ("Ready?", "v", "0", "0");
const SAYS_WIN: [FerrisLine; 3] = [
    ("Win!", "3", "^", "^"),          // big grin
    ("WOW, Genius!", "D", "*", "*"),  // starry eyes
    ("GG, easy game.", "w", "^", "^"), // smug smile
];
const SAYS_LOSE: [FerrisLine; 3] = [
    ("You Lose!", "x", "x", "x"),  // knocked out
    ("BOOM!", "o", "@", "@"),      // dizzy
    ("Ouch...", "n", "T", "T"),    // crying
];
const SAYS_MINE: [FerrisLine; 3] = [
    ("You're safe.", "o", "0", "0"),
    ("Rua!!!", "D", "^", "^"),  // delighted
    ("Phew~", "u", "-", "-"),   // relieved
];
const SAYS_FLAG: [FerrisLine; 3] = [
    ("You can't flag here.", "x", "o", "o"), // rejected
    ("Good Flag.", "u", "^", "^"),            // approving
    ("Nice catch!", "D", "*", "*"),           // impressed
];
const SAYS_MOVE: [FerrisLine; 3] = [
    ("Move Move Move!", "r", "6", "6"), // hurried, dizzy eyes
    ("Are you sure?", "o", "?", "?"),   // suspicious
    ("Take your time.", "u", "-", "-"), // patient
];

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
///
/// The whole frame is assembled into a single string and flushed with one
/// write, instead of one `print!` (and syscall) per line/segment. This
/// avoids the partial-frame flicker that repeated small writes can cause
/// when the crab is redrawn frequently (e.g. on every keypress).
fn draw_ferris_with(pos: &Pos, words: &str, mouth: &str, leye: &str, reye: &str) -> Pos {
    let len = words.len();
    let Pos(x, y) = *pos;
    let border: String = iter::repeat('_').take(len + 2).collect();
    let underline: String = iter::repeat('-').take(len + 2).collect();

    let mut frame = String::new();
    frame.push_str(&format!("{}{}", cursor::Goto(x, y + 8), clear::BeforeCursor));
    frame.push_str(&format!("{}{}", cursor::Goto(x, y), color::Fg(color::LightRed)));
    frame.push_str(&format!("{}{}", cursor::Goto(x + 1, y), border));
    frame.push_str(&format!("{}< {} >", cursor::Goto(x, y + 1), words));
    frame.push_str(&format!("{} {}", cursor::Goto(x, y + 2), underline));
    frame.push_str(&format!("{}    \\", cursor::Goto(x, y + 3)));
    frame.push_str(&format!("{}     \\", cursor::Goto(x, y + 4)));
    frame.push_str(&format!("{}    _~^~^~_", cursor::Goto(x, y + 5)));
    frame.push_str(&format!(
        "{}\\) /  {} {}  \\ (/",
        cursor::Goto(x, y + 6),
        leye,
        reye
    ));
    frame.push_str(&format!("{}  '_   {}   _'", cursor::Goto(x, y + 7), mouth));
    frame.push_str(&format!(
        "{}  / '-----' \\{}",
        cursor::Goto(x, y + 8),
        style::Reset
    ));

    print!("{}", frame);
    io::stdout().flush().ok();
    return Pos(x, y + 9);
}

/// Play a quick blink animation on the crab's eyes (open -> closed -> open)
/// on top of an already-drawn frame, giving the mascot a bit of life instead
/// of a fully static pose. Cheap: only the eye row is touched per frame.
pub fn animate_ferris_blink(pos: &Pos, words: &str, mouth: &str, leye: &str, reye: &str) -> Pos {
    let ret = draw_ferris_with(pos, words, mouth, leye, reye);
    let Pos(x, y) = *pos;
    sleep(Duration::from_millis(220));
    print!("{}\\) /  -   -  \\ (/", cursor::Goto(x, y + 6));
    io::stdout().flush().ok();
    sleep(Duration::from_millis(90));
    print!("{}\\) /  {} {}  \\ (/", cursor::Goto(x, y + 6), leye, reye);
    io::stdout().flush().ok();
    ret
}

pub fn ferris_says_start(pos: &Pos) -> Pos {
    let (words, mouth, leye, reye) = SAYS_START;
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_win(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_WIN[idx % SAYS_WIN.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_lose(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_LOSE[idx % SAYS_LOSE.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_mine(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_MINE[idx % SAYS_MINE.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_flag(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_FLAG[idx % SAYS_FLAG.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_move(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_MOVE[idx % SAYS_MOVE.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_quit(pos: &Pos) -> Pos {
    let (words, mouth, leye, reye) = SAYS_QUIT;
    draw_ferris_with(pos, words, mouth, leye, reye)
}

pub fn ferris_says_difficulty(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_DIFFICULTIES[idx % SAYS_DIFFICULTIES.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
}

/// Same as [`ferris_says_difficulty`] but plays a quick blink the first time
/// the crab greets the player on the difficulty screen.
pub fn ferris_says_difficulty_blink(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_DIFFICULTIES[idx % SAYS_DIFFICULTIES.len()];
    animate_ferris_blink(pos, words, mouth, leye, reye)
}

pub fn ferris_says_end(pos: &Pos, idx: usize) -> Pos {
    let (words, mouth, leye, reye) = SAYS_END[idx % SAYS_END.len()];
    draw_ferris_with(pos, words, mouth, leye, reye)
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
