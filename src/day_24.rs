use std::{collections::HashSet, hash::Hash};

static INPUT: &str = include_str!("assets/day_24_input.txt");

#[derive(Debug, Hash, PartialEq, Eq)]
struct Tile {
    col: i16, // skewed
    row: i16,
}

impl Tile {
    fn new(col: i16, row: i16) -> Self {
        Self { col, row }
    }

    fn get_neighbors(&self) -> [Tile; 6] {
        [
            Self::new(self.col + 1, self.row),
            Self::new(self.col - 1, self.row),
            Self::new(self.col, self.row + 1),
            Self::new(self.col, self.row - 1),
            Self::new(self.col + 1, self.row - 1),
            Self::new(self.col - 1, self.row + 1),
        ]
    }

    fn get_adjacent_black(&self, black_tiles: &HashSet<Tile>) -> usize {
        self.get_neighbors()
            .iter()
            .filter(|neighbor| black_tiles.contains(neighbor))
            .count()
    }
}

fn find_tile(path: &str) -> Tile {
    // println!("Find tile at '{}'", path);
    let mut chars = path.chars();
    let mut col = 0;
    let mut row = 0;
    loop {
        let next = chars.next();
        if next.is_none() {
            let tile = Tile { col, row };
            // println!("Return tile {:?}", tile);
            return tile;
        }
        let next = next.unwrap();
        match next {
            'w' => col -= 1,
            'e' => col += 1,
            // treat northwest/southeast as the 'vertical' column
            'n' => {
                let subheading = chars.next().unwrap();
                match subheading {
                    'w' => row -= 1,
                    'e' => {
                        row -= 1;
                        col += 1;
                    }
                    _ => panic!("Bad input"),
                }
            }
            's' => {
                let subheading = chars.next().unwrap();
                match subheading {
                    'w' => {
                        row += 1;
                        col -= 1;
                    }
                    'e' => row += 1,
                    _ => panic!("Bad input"),
                }
            }
            _ => panic!("Bad input"),
        }
    }
}

fn flip_all_tiles(input: &str) -> HashSet<Tile> {
    let mut flipped = HashSet::new();
    for line in input.lines() {
        let tile = find_tile(line);
        if !flipped.remove(&tile) {
            flipped.insert(tile);
        }
    }

    flipped
}

fn perform_daily_flip(black_tiles: HashSet<Tile>) -> HashSet<Tile> {
    let tiles_to_check: HashSet<Tile> = black_tiles
        .iter()
        .map(|tile| Vec::from(tile.get_neighbors()))
        .flatten()
        .collect();
    let mut new_black_tiles = HashSet::new();
    for tile in tiles_to_check {
        let neighbor_count = tile.get_adjacent_black(&black_tiles);
        let is_black = black_tiles.contains(&tile);
        if is_black && (neighbor_count == 1 || neighbor_count == 2) {
            // This tile remains black
            new_black_tiles.insert(tile);
        } else if !is_black && neighbor_count == 2 {
            // White tile flipped to black
            new_black_tiles.insert(tile);
        }
    }

    new_black_tiles
}

fn daily_flips(black_tiles: HashSet<Tile>) -> HashSet<Tile> {
    let mut black_tiles = black_tiles;
    for _ in 0..100 {
        black_tiles = perform_daily_flip(black_tiles);
    }

    black_tiles
}

pub fn p1() -> usize {
    flip_all_tiles(&INPUT).len()
}

pub fn p2() -> usize {
    daily_flips(flip_all_tiles(&INPUT)).len()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"#;

    #[test]
    fn p1_example() {
        let flipped = flip_all_tiles(&EXAMPLE);
        assert_eq!(10, flipped.len());
    }

    #[test]
    fn p1_correct_answer() {
        let flipped = flip_all_tiles(&INPUT);
        assert_eq!(244, flipped.len());
    }

    #[test]
    fn p2_example() {
        let flipped = flip_all_tiles(&EXAMPLE);
        let hundred_days = daily_flips(flipped);
        assert_eq!(2208, hundred_days.len());
    }

    #[test]
    fn p2_correct_answer() {
        let flipped = flip_all_tiles(&INPUT);
        let hundred_days = daily_flips(flipped);
        assert_eq!(3665, hundred_days.len());
    }
}
