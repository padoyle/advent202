static INPUT: &str = include_str!("assets/day_20_input.txt");

static TOP: usize = 0;
static BOT: usize = 0;
static LEFT: usize = 0;
static RIGHT: usize = 0;

#[derive(Debug)]
struct Tile {
    id: u16,
    sides: [u16; 4],
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
            .map(|line| line.chars().next().unwrap())
            .fold(0, fold_bits)
            >> 1;
        sides[2] = tile_lines.last().unwrap().chars().fold(0, fold_bits) >> 1;
        sides[3] = tile_lines
            .iter()
            .rev()
            .map(|line| line.chars().last().unwrap())
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
            new_sides[LEFT] = reverse_10_bits(self.sides[LEFT]);
            new_sides[RIGHT] = reverse_10_bits(self.sides[RIGHT]);
        }
        if horizontal {
            new_sides[TOP] = reverse_10_bits(self.sides[TOP]);
            new_sides[BOT] = reverse_10_bits(self.sides[BOT]);
        }
    }
}

fn fits_h(left: &Tile, right: &Tile) -> bool {
    left.sides[RIGHT] == reverse_10_bits(right.sides[LEFT])
}

fn fits_v(top: &Tile, bot: &Tile) -> bool {
    top.sides[BOT] == reverse_10_bits(bot.sides[TOP])
}

fn parse_tiles(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_str| Tile::parse(tile_str))
        .collect()
}

pub fn p1() -> u64 {
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
        for tile in tiles {
            println!("{:?}", tile);
        }
        // assert_eq!(1, 2);
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
