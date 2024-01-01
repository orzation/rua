#![allow(dead_code)]

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
struct Cell {
    content: Content,
    surface: Surface,
}

impl ToString for Cell {
    /// Returns the string of a cell.
    fn to_string(&self) -> String {
        match self.surface {
            Surface::Cover => "▓",
            Surface::Flag => "P",
            Surface::Open => match self.content {
                Content::Zero => "0",
                Content::One => "1",
                Content::Two => "2",
                Content::Three => "3",
                Content::Four => "4",
                Content::Five => "5",
                Content::Six => "6",
                Content::Seven => "7",
                Content::Eight => "8",
                Content::Bomb => "*",
            },
        }
        .to_string()
    }
}

#[rustfmt::skip]
const DIRS: [(isize, isize); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1), (1, 0), (1, 1),
];
