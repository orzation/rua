use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone)]
enum Cell {
    Zero(bool),
    One(bool),
    Two(bool),
    Three(bool),
    Four(bool),
    Five(bool),
    Six(bool),
    Seven(bool),
    Eight(bool),
    Bomb(bool),
}

fn gen_map(rng: &mut ThreadRng, h: usize, w: usize, bomb: usize, init_pos: usize) -> Vec<Cell> {
    let size = h * w;
    if size < bomb {
        panic!("map size cannot be less then bombs")
    }
    let mut map = vec![0i8; size];
    binary_random(rng, bomb, &mut map);
    if map[init_pos] == Cell::Bomb as i8 {
        let idx = map.iter().position(|&x| x == Cell::Bomb as i8).unwrap();
        map.swap(init_pos, idx);
    }

    let (h, w) = (h as isize, w as isize);
    let dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];
    let mut map_clone = vec![0i8; map.len()];
    map.iter()
        .enumerate()
        .filter(|(_, &val)| val == Cell::Bomb as i8)
        .for_each(|(idx, &val)| {
            map_clone[idx] = val;
            let idx = idx as isize;
            let (x, y) = (idx / w, idx % w);
            dirs.iter().for_each(|(dx, dy)| {
                let (a, b) = (x + dx, y + dy);

                if a >= 0 && a < h && b >= 0 && b < w {
                    let pos = (a * h + b) as usize;
                    if map[pos] != Cell::Bomb as i8 {
                        map_clone[pos] += 1;
                    }
                }
            });
        });
    map_clone
        .into_iter()
        .map(|v| match v {
            0 => Cell::Zero(false),
            1 => Cell::One(false),
            2 => Cell::Two(false),
            3 => Cell::Three(false),
            4 => Cell::Four(false),
            5 => Cell::Five(false),
            6 => Cell::Six(false),
            7 => Cell::Seven(false),
            8 => Cell::Eight(false),
            _ => Cell::Bomb(false),
        })
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

fn main() {
    let (height, width, bomb) = (9, 9, 10);
    let map_size = height * width;

    let mut rng = rand::thread_rng();
    let init_pos = rng.gen_range(0..map_size);

    let graph_map = gen_map(&mut rng, height, width, bomb, init_pos);
    // debug print out
    for (i, v) in graph_map.iter().enumerate() {
        print!(
            "{} ",
            match v {
                Cell::Zero(false) => "0",
                Cell::One(false) => "1",
                Cell::Two(false) => "2",
                Cell::Three(false) => "3",
                Cell::Four(false) => "4",
                Cell::Five(false) => "5",
                Cell::Six(false) => "6",
                Cell::Seven(false) => "7",
                Cell::Eight(false) => "8",
                Cell::Bomb(false) => "*",
                _ => "!",
            }
        );
        if (i + 1) % 9 == 0 {
            println!()
        }
    }
}
