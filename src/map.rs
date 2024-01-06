#![allow(dead_code)]

use termion::{color, style};

/// The content type of a cell.
enum Content {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Bomb,
}

/// The surface type of a cell.
enum Surface {
    Cover,
    Open,
    Flag,
}

/// Cell is the minimal member of map.
pub struct Cell {
    content: Content,
    surface: Surface,
}

impl ToString for Surface {
    fn to_string(&self) -> String {
        match self {
            Surface::Cover => format!("▓"),
            Surface::Flag => format!("{}P{}", color::Fg(color::Yellow), style::Reset),
            _ => format!(" "),
        }
    }
}

impl ToString for Content {
    fn to_string(&self) -> String {
        match self {
            Content::Zero => format!(" "),
            Content::One => format!("{}1{}", color::Fg(color::LightBlue), style::Reset),
            Content::Two => format!("{}2{}", color::Fg(color::LightGreen), style::Reset),
            Content::Three => format!("{}3{}", color::Fg(color::LightRed), style::Reset),
            Content::Four => format!("{}4{}", color::Fg(color::Magenta), style::Reset),
            Content::Five => format!("{}5{}", color::Fg(color::Red), style::Reset),
            Content::Six => format!("{}6{}", color::Fg(color::Green), style::Reset),
            Content::Seven => format!("{}7{}", color::Fg(color::Blue), style::Reset),
            Content::Eight => format!("{}8{}", color::Fg(color::Cyan), style::Reset),
            Content::Bomb => format!(
                "{}{}*{}",
                color::Fg(color::Black),
                color::Bg(color::Red),
                style::Reset
            ),
        }
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        match self.surface {
            Surface::Open => self.content.to_string(),
            _ => self.surface.to_string(),
        }
    }
}

impl Cell {
    pub fn get_content_symbol(&self) -> String {
        self.content.to_string()
    }
    pub fn get_surface_symbol(&self) -> String {
        self.surface.to_string()
    }
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1), (1, 0), (1, 1),
];
