#![allow(dead_code)]

use queues::{queue, IsQueue, Queue};
use rand::{rngs::ThreadRng, Rng};
use termion::{color, style};

use crate::{config, draw};

/// The content type of a cell.
#[derive(Clone)]
pub enum Content {
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
#[derive(Clone, PartialEq, Eq)]
pub enum Surface {
    Cover,
    Open,
    Flag,
}

/// Cell is the minimal member of map.
#[derive(Clone)]
pub struct Cell {
    pub content: Content,
    pub surface: Surface,
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

impl Content {
    pub fn from_i8(num: i8) -> Content {
        match num {
            0 => Content::Zero,
            1 => Content::One,
            2 => Content::Two,
            3 => Content::Three,
            4 => Content::Four,
            5 => Content::Five,
            6 => Content::Six,
            7 => Content::Seven,
            8 => Content::Eight,
            _ => Content::Bomb,
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
    pub fn new(content: Content, surface: Surface) -> Cell {
        Cell { content, surface }
    }
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

/// check surround cells of pos, and execute fn(pos) for them
fn check_around_fn(h: usize, w: usize, pos: usize, mut op: impl FnMut(usize)) {
    DIRS.iter().for_each(|(dx, dy)| {
        let pos = pos as isize;
        let (h, w) = (h as isize, w as isize);
        let (a, b) = (pos / w + dx, pos % w + dy);
        if a >= 0 && a < h && b >= 0 && b < w {
            op((a * w + b) as usize)
        }
    });
}

fn binary_random(rng: &mut ThreadRng, bomb: usize, map: &mut [i8]) {
    let len = map.len();
    if bomb <= 1 {
        let rand_pos = rng.gen_range(0..len);
        map[rand_pos] = Content::Bomb as i8;
        return;
    }
    let mid = len / 2;
    binary_random(rng, bomb / 2, &mut map[..mid]);
    binary_random(rng, (bomb + 1) / 2, &mut map[mid..]);
}

/// Generate a random map with a specific position and game configs.
pub fn gen_map(
    init_at: &draw::Pos,
    now_at: &draw::Pos,
    conf: &config::GameConfig,
    rng: &mut ThreadRng,
) -> Vec<Cell> {
    let init_pos = (now_at.1 - init_at.1) as usize * conf.width + (now_at.0 - init_at.0) as usize;
    let mut map = vec![0i8; conf.get_size()];
    binary_random(rng, conf.bomb, &mut map);

    if map[init_pos] == Content::Bomb as i8 {
        let idx = map.iter().position(|&x| x != Content::Bomb as i8).unwrap();
        map.swap(init_pos, idx);
    }

    let mut map_clone = vec![0i8; map.len()];
    map.iter()
        .enumerate()
        .filter(|(_, &val)| val == Content::Bomb as i8)
        .for_each(|(idx, &val)| {
            map_clone[idx] = val;
            check_around_fn(conf.height, conf.width, idx, |pos| {
                if map[pos] != Content::Bomb as i8 {
                    map_clone[pos] += 1;
                }
            });
        });
    map_clone
        .into_iter()
        .map(|v| Cell::new(Content::from_i8(v), Surface::Cover))
        .collect()
}

/// Open one cell in a specific position.
pub fn mine_map(
    init_at: &draw::Pos,
    now_at: &draw::Pos,
    conf: &config::GameConfig,
    mut map: Vec<Cell>,
    left_cover: &mut usize,
) -> Vec<Cell> {
    let pos = (now_at.1 - init_at.1) as usize * conf.width + (now_at.0 - init_at.0) as usize;
    let mut queue: Queue<usize> = queue![];
    queue.add(pos).unwrap();

    while queue.size() > 0 {
        let one = queue.remove().unwrap();

        map[one].surface = match map[one].surface {
            Surface::Cover => {
                match map[one].content {
                    Content::Zero => check_around_fn(conf.height, conf.width, one, |pos| {
                        if let Surface::Cover = &map[pos].surface {
                            queue.add(pos).unwrap();
                        }
                    }),
                    _ => (),
                };
                *left_cover -= 1;
                Surface::Open
            }
            Surface::Open => Surface::Open,
            Surface::Flag => Surface::Flag,
        };
    }
    map
}

/// Put a flag on specific position.
pub fn flag_map(
    init_at: &draw::Pos,
    now_at: &draw::Pos,
    conf: &config::GameConfig,
    mut map: Vec<Cell>,
    flag_num: &mut usize,
) -> (Vec<Cell>, bool) {
    let pos = (now_at.1 - init_at.1) as usize * conf.width + (now_at.0 - init_at.0) as usize;
    let pre_status = map[pos].surface.clone();

    map[pos].surface = match map[pos].surface {
        Surface::Open => Surface::Open,
        Surface::Cover => {
            if *flag_num == conf.bomb {
                return (map, false);
            }
            *flag_num += 1;
            Surface::Flag
        }
        Surface::Flag => {
            if *flag_num == 0 {
                return (map, false);
            }
            *flag_num -= 1;
            Surface::Cover
        }
    };
    let now_status = map[pos].surface.clone();
    (map, pre_status != now_status)
}
