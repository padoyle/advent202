use std::collections::HashSet;

static INPUT: &str = r#".#.#.#..
..#....#
#####..#
#####..#
#####..#
###..#.#
#..##.##
#.#.####"#;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn add(&self, other: &Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

lazy_static! {
    static ref NEIGHBOR_COORDS: Vec<Point> = {
        let values = vec![-1, 0, 1];
        let mut result = Vec::new();
        for &x in values.iter() {
            for &y in values.iter() {
                for &z in values.iter() {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    result.push(Point::new(x, y, z));
                }
            }
        }
        result
    };
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut x;
    let mut y = 0;
    let mut result = HashSet::new();
    for line in input.lines() {
        x = 0;
        for next_char in line.chars() {
            match next_char {
                '#' => {
                    result.insert(Point::new(x, y, 0));
                }
                '.' | _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    result
}

fn run_cycle(active_set: &HashSet<Point>) -> HashSet<Point> {
    // Get all the neighbors of all the currently active ones
    let new_active_set: HashSet<Point> = active_set
        .iter()
        .map(|point| {
            NEIGHBOR_COORDS
                .iter()
                .map(move |neighbor| point.add(neighbor))
        })
        .flatten()
        .collect();

    new_active_set
        .into_iter()
        .filter(|point| {
            let is_active = active_set.contains(point);
            let active_neighbors = count_active_neighbors(active_set, point);

            (is_active && (active_neighbors == 2 || active_neighbors == 3)) // stays active
            || (!is_active && active_neighbors == 3) // becomes active
        })
        .collect()
}

fn count_active_neighbors(active_set: &HashSet<Point>, point: &Point) -> usize {
    NEIGHBOR_COORDS
        .iter()
        .map(|neighbor| point.add(neighbor))
        .filter(|neighbor_point| active_set.contains(neighbor_point))
        .count()
}

fn run_cycles(starting_set: HashSet<Point>, cycles: usize) -> HashSet<Point> {
    let mut set = starting_set;
    for _ in 0..cycles {
        set = run_cycle(&set);
    }

    set
}

pub fn p1() -> usize {
    run_cycles(parse_input(INPUT), 6).len()
}

pub fn p2() -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#".#.
..#
###"#;

    #[test]
    fn p1_example() {
        assert_eq!(112, run_cycles(parse_input(EXAMPLE), 6).len());
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(375, run_cycles(parse_input(INPUT), 6).len())
    }

    // #[test]
    // fn p2_example() {}

    // #[test]
    // fn p2_correct_answer() {}
}
