use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("assets/day_20_input.txt");

// We parse the sides clockwise
static TOP: usize = 0;
static RIGHT: usize = 1;
static BOT: usize = 2;
static LEFT: usize = 3;

// Status nibble:
// [0] [0] [0 0]
//  |   |    |--> 0-3 for rotation count
//  |   |--> flipped vertical
//  |--> flipped horizontal

#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    sides: [u16; 4],
    status: u8,
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

        Self {
            id,
            sides,
            status: 0,
        }
    }

    fn rotate_90(&mut self) {
        let mut new_sides = [0; 4];
        new_sides[0] = self.sides[3];
        new_sides[1] = self.sides[0];
        new_sides[2] = self.sides[1];
        new_sides[3] = self.sides[2];

        self.sides = new_sides;
        // clear and reassign rotation
        let rot_status = ((self.status & 0b11) + 1) % 4;
        self.status &= 0b1100;
        self.status |= rot_status;
    }

    fn flip(&mut self, vertical: bool, horizontal: bool) {
        let mut new_sides = self.sides.clone();
        if vertical {
            // swap top & bottom sides (flipping to preserve clockwise read direction)
            new_sides[TOP] = reverse_10_bits(self.sides[BOT]);
            new_sides[BOT] = reverse_10_bits(self.sides[TOP]);
            // reverse left & right sides
            new_sides[LEFT] = reverse_10_bits(self.sides[LEFT]);
            new_sides[RIGHT] = reverse_10_bits(self.sides[RIGHT]);
            self.status ^= 0b0100;
        }
        if horizontal {
            // swap left & right sides (flipping to preserve clockwise read direction)
            new_sides[LEFT] = reverse_10_bits(self.sides[RIGHT]);
            new_sides[RIGHT] = reverse_10_bits(self.sides[LEFT]);
            // reverse top & bottom sides
            new_sides[TOP] = reverse_10_bits(self.sides[TOP]);
            new_sides[BOT] = reverse_10_bits(self.sides[BOT]);
            self.status ^= 0b1000;
        }
        self.sides = new_sides;
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
            self.rotate_90();
        }
        self.flip(true, false);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate_90();
        }
        self.flip(false, true);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate_90();
        }
        self.flip(true, false);
        for _ in 0..4 {
            if self.check_fit(neighbors) {
                return true;
            }
            self.rotate_90();
        }
        // reset state
        self.flip(false, true);
        assert!(self.status == 0);

        false
    }
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_str| Tile::parse(tile_str))
        .collect()
}

type TileMap = HashMap<(i32, i32), Tile>;

fn multiply_corners(map: &TileMap) -> u64 {
    // Find the bounds of our map; since we start with an arbitrary tile
    // at (0, 0), the bounds aren't obvious
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

    // Multiply the four corners
    vec![
        map.get(&(min_x, min_y)).unwrap().id as u64,
        map.get(&(max_x, min_y)).unwrap().id as u64,
        map.get(&(min_x, max_y)).unwrap().id as u64,
        map.get(&(max_x, max_y)).unwrap().id as u64,
    ]
    .iter()
    .product()
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
    results
}

fn solve(tiles: &Vec<Tile>) -> TileMap {
    // build a map of id to tile for remaining tiles (eaiser to deal with as a map)
    let mut remaining: HashMap<u16, Tile> =
        tiles.iter().map(|tile| (tile.id, tile.clone())).collect();

    // Initialize our image and place our first tile at its "center"
    let mut image: TileMap = HashMap::new();
    let first_tile_key = remaining.keys().next().unwrap().clone();
    let first_tile = remaining.remove(&first_tile_key).unwrap();
    image.insert((0, 0), first_tile);

    loop {
        // For every open position next to an existing tile...
        for (next_x, next_y) in find_open_positions(&image) {
            // Find what values we need the tile to have to match the neighbors
            // of that particular location
            let neighbors: Vec<Option<u16>> = vec![
                image.get(&(next_x, next_y - 1)).map(Tile::bot_reversed),
                image.get(&(next_x + 1, next_y)).map(Tile::left_reversed),
                image.get(&(next_x, next_y + 1)).map(Tile::top_reversed),
                image.get(&(next_x - 1, next_y)).map(Tile::right_reversed),
            ];

            // Iterate through our remaining tiles and find the one that fits
            let mut found_id: Option<u16> = None;
            for (id, tile) in remaining.iter_mut() {
                if tile.find_fit(neighbors.as_slice()) {
                    found_id = Some(*id);
                    break;
                }
            }

            // If we found a tile that fits at this position, break so we can
            // recompute the unoccupied positions
            if let Some(id) = found_id {
                let tile = remaining.remove(&id);
                image.insert((next_x, next_y), tile.unwrap());
                break;
            }
        }

        if remaining.is_empty() {
            break;
        }
    }

    image
}

pub fn p1() -> u64 {
    let tiles = parse_tiles(INPUT);
    let image = solve(&tiles);
    multiply_corners(&image)
}

fn get_tile_data<'a>(input: &'a str) -> HashMap<u16, &'a str> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut split = tile.split(":\n");
            let id: u16 = split
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .parse()
                .unwrap();
            let data = split.next().unwrap();
            (id, data)
        })
        .collect()
}

type TileData = Vec<Vec<char>>;

fn tile_contents(tile_data: &str) -> TileData {
    tile_data
        .lines()
        .skip(1)
        .take(8)
        .map(|line| line.chars().skip(1).take(8).collect())
        .collect()
}

fn transpose(data: &mut TileData) {
    let len = data.len();
    for y in 0..len - 1 {
        for x in (y + 1)..len {
            let a = data[x][y];
            let b = data[y][x];
            data[x][y] = b;
            data[y][x] = a;
        }
    }
}

fn reverse_rows(data: &mut TileData) {
    data.iter_mut().for_each(|row| row.reverse());
}

fn reverse_cols(data: &mut TileData) {
    data.reverse();
}

fn align_tile(status: u8, data: &mut TileData) {
    // flip vertical
    if status & 0b0100 != 0 {
        reverse_cols(data);
    }
    // flip horizontal
    if status & 0b1000 != 0 {
        reverse_rows(data);
    }

    let rot = status & 0b11;

    match rot {
        1 => {
            transpose(data);
            reverse_rows(data);
        }
        2 => {
            reverse_rows(data);
            reverse_cols(data);
        }
        3 => {
            reverse_rows(data);
            transpose(data);
        }
        _ => {} // 0 or anything else, which won't happen
    }
}

fn assemble_image(tiles: TileMap, tile_strs: HashMap<u16, &str>) -> TileData {
    let mut image = Vec::new();
    let mut tile_row: TileData;
    // Find the bounds of our map; since we start with an arbitrary tile
    // at (0, 0), the bounds aren't obvious
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    tiles.keys().for_each(|&(x, y)| {
        min_x = std::cmp::min(x, min_x);
        max_x = std::cmp::max(x, max_x);
        min_y = std::cmp::min(y, min_y);
        max_y = std::cmp::max(y, max_y);
    });

    for y in min_y..max_y + 1 {
        tile_row = vec![Vec::new(); 8];
        for x in min_x..max_x + 1 {
            let tile = tiles.get(&(x, y)).unwrap();
            let mut data = tile_contents(tile_strs.get(&tile.id).unwrap());
            align_tile(tile.status, &mut data);
            for (i, mut row) in data.drain(..).enumerate() {
                tile_row[i].append(&mut row);
            }
        }
        image.append(&mut tile_row);
    }

    image
}

lazy_static! {
    static ref MONSTER_PATTERN: Vec<(usize, usize)> = vec![
        // tail
        (0, 1),
        (1, 2),
        // back hump
        (4, 2),
        (5, 1),
        (6, 1),
        (7, 2),
        // front hump
        (10, 2),
        (11, 1),
        (12, 1),
        (13, 2),
        // neck & head
        (16, 2),
        (17, 1),
        (18, 0),
        (18, 1),
        (19, 1),
    ];
}
static MONSTER_DIMS: (usize, usize) = (19, 2);

fn mark_monsters(map: &mut TileData) -> bool {
    let dim = map.len();
    let mut found_any = false;
    for y in 0..(dim - MONSTER_DIMS.1) {
        for x in 0..(dim - MONSTER_DIMS.0) {
            let found = MONSTER_PATTERN.iter().all(|offset| {
                let x_off = x + offset.0;
                let y_off = y + offset.1;
                map[y_off][x_off] == '#'
            });
            if found {
                // mark
                MONSTER_PATTERN.iter().for_each(|offset| {
                    let x_off = x + offset.0;
                    let y_off = y + offset.1;
                    map[y_off][x_off] = '0';
                });
                found_any = true;
            }
        }
    }
    found_any
}

fn check_water_roughness(input: &str) -> usize {
    let tiles = parse_tiles(input);
    let image = solve(&tiles);
    let map = assemble_image(image, get_tile_data(input));

    // Iterate over all possible orientations of the data
    for orientation in 0..=0b1111 {
        let mut modified_map = map.clone();
        align_tile(orientation, &mut modified_map);
        if mark_monsters(&mut modified_map) {
            return modified_map.iter().flatten().filter(|&c| *c == '#').count();
        }
    }
    panic!("No sea monsters found!");
}

pub fn p2() -> usize {
    check_water_roughness(&INPUT)
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
        let image = solve(&tiles);
        assert_eq!(20899048083289, multiply_corners(&image));
    }

    #[test]
    fn p1_correct_answer() {
        let tiles = parse_tiles(INPUT);
        let image = solve(&tiles);
        assert_eq!(174206308298779, multiply_corners(&image));
    }

    #[test]
    fn transpose_data() {
        let input = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let mut transposed = input.clone();
        transpose(&mut transposed);
        let expected = vec![
            vec!['1', '4', '7'],
            vec!['2', '5', '8'],
            vec!['3', '6', '9'],
        ];
        assert_eq!(expected, transposed);
        let input = vec![
            vec!['0', '1', '2', '3'],
            vec!['4', '5', '6', '7'],
            vec!['8', '9', 'A', 'B'],
            vec!['C', 'D', 'E', 'F'],
        ];
        let mut transposed = input.clone();
        transpose(&mut transposed);
        let expected = vec![
            vec!['0', '4', '8', 'C'],
            vec!['1', '5', '9', 'D'],
            vec!['2', '6', 'A', 'E'],
            vec!['3', '7', 'B', 'F'],
        ];
        assert_eq!(expected, transposed);
    }

    #[test]
    fn p2_example() {
        assert_eq!(273, check_water_roughness(&EXAMPLE));
    }

    #[test]
    fn p2_correct_answer() {
        assert_eq!(2409, check_water_roughness(&INPUT));
    }
}
