use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

static INPUT: &str = include_str!("assets/day_20_input.txt");

// We parse the sides clockwise
static TOP: usize = 0;
static RIGHT: usize = 1;
static BOT: usize = 2;
static LEFT: usize = 3;

#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    sides: [u16; 4],
}

impl Tile {
    fn top_reversed(&self) -> u16 {
        reverse_10_bits(self.sides[TOP])
    }

    fn right_reversed(&self) -> u16 {
        reverse_10_bits(self.sides[RIGHT])
    }

    fn bot_reversed(&self) -> u16 {
        reverse_10_bits(self.sides[BOT])
    }

    fn left_reversed(&self) -> u16 {
        reverse_10_bits(self.sides[LEFT])
    }
}

fn reverse_10_bits(value: u16) -> u16 {
    let mut new_value = 0;
    for i in 0..10 {
        if value & (1 << i) != 0 {
            new_value |= 1 << (9 - i);
        }
    }
    new_value
}

// fn precalculate_reverse_lookup() -> HashMap<u16, u16> {
//     let mut map = HashMap::new();
//     for i in 0..0b1111111111 {
//         map.insert(i, reverse_10_bits(i));
//     }
//     map
// }

impl Tile {
    fn parse(input: &str) -> Self {
        fn fold_bits(mut acc: u16, value: char) -> u16 {
            if value == '#' {
                acc += 1;
            }
            acc << 1
        };

        let mut lines = input.lines();
        let id: u16 = lines
            .next()
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();
        let mut sides = [0; 4];

        let tile_lines: Vec<&str> = lines.collect();
        // trace sides clockwise
        sides[0] = tile_lines.first().unwrap().chars().fold(0, fold_bits) >> 1;
        sides[1] = tile_lines
            .iter()
            .map(|line| line.chars().last().unwrap())
            .fold(0, fold_bits)
            >> 1;
        sides[2] = tile_lines.last().unwrap().chars().rev().fold(0, fold_bits) >> 1;
        sides[3] = tile_lines
            .iter()
            .rev()
            .map(|line| line.chars().next().unwrap())
            .fold(0, fold_bits)
            >> 1;

        Self { id, sides }
    }

    // rotate by 90 degrees, 'count' times
    fn rotate(&mut self, count: usize) {
        let mut new_sides = [0; 4];
        new_sides[0] = self.sides[(0 + count) % 4];
        new_sides[1] = self.sides[(1 + count) % 4];
        new_sides[2] = self.sides[(2 + count) % 4];
        new_sides[3] = self.sides[(3 + count) % 4];

        self.sides = new_sides;
    }

    fn flip(&mut self, vertical: bool, horizontal: bool) {
        let mut new_sides = self.sides.clone();
        if vertical {
            // swap top & bottom sides
            new_sides[TOP] = self.sides[BOT];
            new_sides[BOT] = self.sides[TOP];
            // reverse left & right sides
            new_sides[LEFT] = reverse_10_bits(self.sides[LEFT]);
            new_sides[RIGHT] = reverse_10_bits(self.sides[RIGHT]);
        }
        if horizontal {
            // swap left & right sides
            new_sides[LEFT] = self.sides[RIGHT];
            new_sides[RIGHT] = self.sides[LEFT];
            // reverse top & bottom sides
            new_sides[TOP] = reverse_10_bits(self.sides[TOP]);
            new_sides[BOT] = reverse_10_bits(self.sides[BOT]);
        }
    }

    fn check_fit(&self, neighbors: &[Option<u16>]) -> bool {
        (0..4).all(|i| match neighbors[i] {
            Some(needed) => self.sides[i] == needed,
            None => true,
        })
    }

    fn find_fit(&mut self, neighbors: &[Option<u16>]) -> bool {
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate(1);
        }
        self.flip(true, false);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate(1);
        }
        self.flip(false, true);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate(1);
        }
        self.flip(true, false);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate(1);
        }

        false
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_str| Tile::parse(tile_str))
        .collect()
}

struct TileNode<'a> {
    tile: &'a Tile,
    up: Option<&'a Tile>,
    down: Option<&'a Tile>,
    left: Option<&'a Tile>,
    right: Option<&'a Tile>,
}

impl<'a> TileNode<'a> {
    fn new(tile: &'a Tile) -> Self {
        Self {
            tile,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}

type TileMap = HashMap<(i32, i32), Tile>;

fn debug_print_map(map: &TileMap) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    map.keys().for_each(|&(x, y)| {
        min_x = std::cmp::min(x, min_x);
        max_x = std::cmp::max(x, max_x);
        min_y = std::cmp::min(y, min_y);
        max_y = std::cmp::max(y, max_y);
    });
    for x in min_x..max_x + 1 {
        println!();
        for y in min_y..max_y + 1 {
            match map.get(&(x, y)) {
                Some(tile) => print!("{} ", tile.id),
                None => print!("____ "),
            }
        }
    }
    println!();
}

fn find_open_positions(image: &TileMap) -> HashSet<(i32, i32)> {
    let mut results = HashSet::new();
    for ((x, y), _) in image.iter() {
        if image.get(&(x - 1, *y)).is_none() {
            results.insert((x - 1, *y));
        }
        if image.get(&(x + 1, *y)).is_none() {
            results.insert((x + 1, *y));
        }
        if image.get(&(*x, y - 1)).is_none() {
            results.insert((*x, y - 1));
        }
        if image.get(&(*x, y + 1)).is_none() {
            results.insert((*x, y + 1));
        }
    }
    println!("{} possible positions to check", results.len());
    results
}

fn solve(tiles: Vec<Tile>) -> TileMap {
    let mut remaining: HashMap<u16, Tile> =
        tiles.iter().map(|tile| (tile.id, tile.clone())).collect();
    // let dim = (tiles.len() as f64).sqrt() as usize;
    // let mut min_x = 0;
    // let mut min_y = 0;
    // let mut max_x = 0;
    // let mut max_y = 0;
    let mut image: TileMap = HashMap::new();
    let first_tile_key = remaining.keys().next().unwrap().clone();
    let first_tile = remaining.remove(&first_tile_key).unwrap();
    println!("Place {} at (0, 0)", first_tile_key);
    image.insert((0, 0), first_tile);

    let mut loop_count = 0;
    loop {
        println!("{} remaining", remaining.len());
        for (next_x, next_y) in find_open_positions(&image) {
            println!("Attempt to place tile at ({}, {})", next_x, next_y);
            let neighbors: Vec<Option<u16>> = vec![
                image.get(&(next_x, next_y - 1)).map(Tile::bot_reversed),
                image.get(&(next_x + 1, next_y)).map(Tile::left_reversed),
                image.get(&(next_x, next_y + 1)).map(Tile::top_reversed),
                image.get(&(next_x - 1, next_y)).map(Tile::right_reversed),
            ];
            println!("Neighbors to match: {:?}", neighbors);

            let mut found_id: Option<u16> = None;
            for (id, tile) in remaining.iter_mut() {
                println!("Check tile {}", tile.id);
                if tile.find_fit(neighbors.as_slice()) {
                    println!("\tTile {} fits!", id);
                    found_id = Some(*id);
                    break;
                }
            }

            if let Some(id) = found_id {
                let tile = remaining.remove(&id);
                image.insert((next_x, next_y), tile.unwrap());
                break;
            }
        }

        loop_count += 1;
        if loop_count >= 15 {
            println!();
            debug_print_map(&image);
            panic!("NO MORE LOOOPING");
        }
        // if remaining.is_empty() {
        //     break;
        // }
    }

    image
}

pub fn p1() -> u64 {
    let tiles = parse_tiles(INPUT);
    let _result = solve(tiles);
    0
}

pub fn p2() -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = include_str!("assets/day_20_example.txt");

    #[test]
    fn p1_reverse_bits() {
        assert_eq!(0, reverse_10_bits(0));
        assert_eq!(512, reverse_10_bits(1));
        assert_eq!(768, reverse_10_bits(3));
        assert_eq!(48, reverse_10_bits(48));
        assert_eq!(682, reverse_10_bits(341));
        assert_eq!(341, reverse_10_bits(682));
    }

    #[test]
    fn p1_example() {
        let tiles = parse_tiles(EXAMPLE);
        for tile in tiles.iter() {
            println!("{:?}", tile);
        }
        // panic!();
        let _result = solve(tiles);
        assert_eq!(0, 1);
    }

    // #[test]
    // fn p1_correct_answer() {
    // }

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
