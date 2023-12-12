#![allow(dead_code)]
use queues::{queue, IsQueue, Queue};
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
enum Cell {
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
impl Cell {
    fn to_str(&self) -> &'static str {
        match self {
            Cell::Zero => "[0]",
            Cell::One => "[1]",
            Cell::Two => "[2]",
            Cell::Three => "[3]",
            Cell::Four => "[4]",
            Cell::Five => "[5]",
            Cell::Six => "[6]",
            Cell::Seven => "[7]",
            Cell::Eight => "[8]",
            Cell::Bomb => "[*]",
        }
    }
    fn from_i8(num: i8) -> Cell {
        match num {
            0 => Cell::Zero,
            1 => Cell::One,
            2 => Cell::Two,
            3 => Cell::Three,
            4 => Cell::Four,
            5 => Cell::Five,
            6 => Cell::Six,
            7 => Cell::Seven,
            8 => Cell::Eight,
            _ => Cell::Bomb,
        }
    }
}

#[derive(Debug, Clone)]
enum Surface {
    Cover(Cell),
    Open(Cell),
    Flag(Cell),
}
impl Surface {
    fn to_str(&self) -> &'static str {
        match self {
            Surface::Cover(_) => "[ ]",
            Surface::Open(cell) => cell.to_str(),
            Surface::Flag(_) => "[p]",
        }
    }
}

#[derive(Debug)]
enum GameState {
    Win,
    Lose,
}

const WIN: &str = "Win!";
const LOSE: &str = "Loser!";
const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

// check surround cells of pos, and execute fn(pos) for them
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

fn gen_map(rng: &mut ThreadRng, h: usize, w: usize, bomb: usize, init_pos: usize) -> Vec<Surface> {
    let size = h * w;
    if size < bomb {
        panic!("map size cannot be less then bombs")
    }
    let mut map = vec![0i8; size];
    binary_random(rng, bomb, &mut map);
    if map[init_pos] == Cell::Bomb as i8 {
        let idx = map.iter().position(|&x| x != Cell::Bomb as i8).unwrap();
        map.swap(init_pos, idx);
    }

    let mut map_clone = vec![0i8; map.len()];
    map.iter()
        .enumerate()
        .filter(|(_, &val)| val == Cell::Bomb as i8)
        .for_each(|(idx, &val)| {
            map_clone[idx] = val;
            check_around_fn(h, w, idx, |pos| {
                if map[pos] != Cell::Bomb as i8 {
                    map_clone[pos] += 1;
                }
            });
        });
    map_clone
        .into_iter()
        .map(|v| Surface::Cover(Cell::from_i8(v)))
        .collect()
}

fn binary_random(rng: &mut ThreadRng, bomb: usize, map: &mut [i8]) {
    let len = map.len();
    if bomb <= 1 {
        let rand_pos = rng.gen_range(0..len);
        map[rand_pos] = Cell::Bomb as i8;
        return;
    }
    let mid = len / 2;
    binary_random(rng, bomb / 2, &mut map[..mid]);
    binary_random(rng, (bomb + 1) / 2, &mut map[mid..]);
}

fn mine_map(
    h: usize,
    w: usize,
    pos: usize,
    mut map: Vec<Surface>,
) -> Result<Vec<Surface>, GameState> {
    let mut queue: Queue<usize> = queue![];
    queue.add(pos).unwrap();

    while queue.size() > 0 {
        let one = queue.remove().unwrap();
        map[one] = match &map[one] {
            Surface::Cover(cell) => Surface::Open({
                match cell {
                    Cell::Zero => check_around_fn(h, w, one, |pos| {
                        if let Surface::Cover(_) = &map[pos] {
                            queue.add(pos).unwrap();
                        }
                    }),
                    Cell::Bomb => return Err(GameState::Lose),
                    _ => (),
                };
                cell.clone()
            }),
            Surface::Open(cell) => Surface::Open(cell.clone()),
            Surface::Flag(cell) => Surface::Open(cell.clone()),
        };
    }
    Ok(map)
}

fn debug_print_out(map: &Vec<Surface>, all_open: bool) {
    for (i, v) in map.iter().enumerate() {
        print!(
            "{}",
            if !all_open {
                v.to_str()
            } else {
                match v {
                    Surface::Open(cell) => cell.to_str(),
                    Surface::Flag(cell) => cell.to_str(),
                    Surface::Cover(cell) => cell.to_str(),
                }
            }
        );
        if (i + 1) % 9 == 0 {
            println!()
        }
    }
    println!();
}

fn main() {
    let (height, width, bomb) = (9, 9, 10);
    let mut rng = rand::thread_rng();
    let init_pos = rng.gen_range(0..height * width);

    let graph_map = gen_map(&mut rng, height, width, bomb, init_pos);

    debug_print_out(&graph_map, true);

    let graph_map = mine_map(height, width, init_pos, graph_map).unwrap();
    debug_print_out(&graph_map, false);
}
