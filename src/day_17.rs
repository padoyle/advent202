use std::{collections::HashSet, hash::Hash};

static INPUT: &str = r#".#.#.#..
..#....#
#####..#
#####..#
#####..#
###..#.#
#..##.##
#.#.####"#;

trait Point {
    fn new(x: i64, y: i64) -> Self;
    fn add(&self, other: &Self) -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Point for Point3 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y, z: 0 }
    }

    fn add(&self, other: &Point3) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point4 {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point for Point4 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y, z: 0, w: 0 }
    }

    fn add(&self, other: &Point4) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

lazy_static! {
    static ref NEIGHBOR_COORDS3: Vec<Point3> = {
        let values = vec![-1, 0, 1];
        let mut result = Vec::new();
        for &x in values.iter() {
            for &y in values.iter() {
                for &z in values.iter() {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    result.push(Point3 { x, y, z });
                }
            }
        }
        result
    };
    static ref NEIGHBOR_COORDS4: Vec<Point4> = {
        let values = vec![-1, 0, 1];
        let mut result = Vec::new();
        for &x in values.iter() {
            for &y in values.iter() {
                for &z in values.iter() {
                    for &w in values.iter() {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }
                        result.push(Point4 { x, y, z, w });
                    }
                }
            }
        }
        result
    };
}

fn parse_input<P: Point + Hash + Eq>(input: &str) -> HashSet<P> {
    let mut x;
    let mut y = 0;
    let mut result = HashSet::new();
    for line in input.lines() {
        x = 0;
        for next_char in line.chars() {
            match next_char {
                '#' => {
                    result.insert(P::new(x, y));
                }
                '.' | _ => {}
            }
            x += 1;
        }
        y += 1;
    }

    result
}

fn run_cycle<P: Point + Eq + Hash>(active_set: &HashSet<P>, neighbors: &Vec<P>) -> HashSet<P> {
    // Get all the neighbors of all the currently active ones
    let new_active_set: HashSet<P> = active_set
        .iter()
        .map(|point| neighbors.iter().map(move |neighbor| point.add(neighbor)))
        .flatten()
        .collect();

    new_active_set
        .into_iter()
        .filter(|point| {
            let is_active = active_set.contains(point);
            let active_neighbors = count_active_neighbors(neighbors, active_set, point);

            (is_active && (active_neighbors == 2 || active_neighbors == 3)) // stays active
            || (!is_active && active_neighbors == 3) // becomes active
        })
        .collect()
}

fn count_active_neighbors<P: Point + Eq + Hash>(
    neighbors: &Vec<P>,
    active_set: &HashSet<P>,
    point: &P,
) -> usize {
    neighbors
        .iter()
        .map(|neighbor| point.add(neighbor))
        .filter(|neighbor_point| active_set.contains(neighbor_point))
        .count()
}

fn run_cycles_3d(starting_set: HashSet<Point3>, cycles: usize) -> HashSet<Point3> {
    let mut set = starting_set;
    for _ in 0..cycles {
        set = run_cycle(&set, &NEIGHBOR_COORDS3);
    }

    set
}

fn run_cycles_4d(starting_set: HashSet<Point4>, cycles: usize) -> HashSet<Point4> {
    let mut set = starting_set;
    for _ in 0..cycles {
        set = run_cycle(&set, &NEIGHBOR_COORDS4);
    }

    set
}

pub fn p1() -> usize {
    run_cycles_3d(parse_input(INPUT), 6).len()
}

pub fn p2() -> usize {
    run_cycles_4d(parse_input(INPUT), 6).len()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#".#.
..#
###"#;

    #[test]
    fn p1_example() {
        assert_eq!(112, run_cycles_3d(parse_input(EXAMPLE), 6).len());
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(375, run_cycles_3d(parse_input(INPUT), 6).len())
    }

    #[test]
    fn p2_example() {
        assert_eq!(848, run_cycles_4d(parse_input(EXAMPLE), 6).len());
    }

    #[test]
    fn p2_correct_answer() {
        assert_eq!(2192, run_cycles_4d(parse_input(INPUT), 6).len())
    }
}
