use std::{collections::HashMap, fmt};

static INPUT: &'static str = include_str!("assets/day_11_input.txt");

lazy_static! {
    static ref ADJACENT: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
}

#[derive(Debug, Clone, PartialEq)]
enum Seat {
    Occupied,
    Empty,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self {
            Seat::Occupied => "#",
            Seat::Empty => "L",
        };

        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct SeatGrid {
    width: i32,
    height: i32,
    seats: Vec<Option<Seat>>,
}

impl fmt::Display for SeatGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for (i, seat) in self.seats.iter().enumerate() {
            if (i as i32) % self.width == 0 {
                result.push('\n');
            }
            match seat {
                Some(value) => result.push_str(value.to_string().as_str()),
                None => result.push('.'),
            };
        }

        write!(f, "{}", result)
    }
}

impl SeatGrid {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let height = input.lines().count();
        let seats = input
            .lines()
            .map(str::chars)
            .flatten()
            .map(|char| match char {
                'L' => Some(Seat::Empty),
                '#' => Some(Seat::Occupied),
                _ => None,
            })
            .collect();

        Self {
            width: width as i32,
            height: height as i32,
            seats,
        }
    }

    fn find_visible(&self, x: i32, y: i32, dir: &(i32, i32)) -> Option<usize> {
        let mut mult = 1;
        while mult < self.width && mult < self.height {
            let (target_x, target_y) = (x + mult * dir.0, y + mult * dir.1);
            if target_x < 0 || target_y < 0 || target_x >= self.width || target_y >= self.height {
                break;
            }
            let target_index = target_y * self.width + target_x;
            if self.seats[target_index as usize].is_some() {
                return Some(target_index as usize);
            }
            mult += 1;
        }

        None
    }

    fn get_visibility_graph(&self) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let index = (y * self.width + x) as usize;
                let visible = ADJACENT
                    .iter()
                    .filter_map(|dir| self.find_visible(x, y, dir))
                    .collect();
                result.insert(index, visible);
            }
        }
        result
    }

    fn count_occupied_adjacent(&self, x: i32, y: i32) -> usize {
        ADJACENT
            .iter()
            .filter_map(|&(other_x, other_y)| {
                let (target_x, target_y) = (x + other_x, y + other_y);
                if target_x < 0 || target_x >= self.width || target_y < 0 || target_y >= self.height
                {
                    None
                } else {
                    let index = target_y * self.width + target_x;
                    self.seats[index as usize].as_ref()
                }
            })
            .filter(|seat| seat == &&Seat::Occupied)
            .count()
    }

    fn total_occupied(&self) -> usize {
        self.seats
            .iter()
            .filter(|&seat| seat == &Some(Seat::Occupied))
            .count()
    }

    fn apply_round_adjacent(&self) -> Self {
        let mut new_seats = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let current = &self.seats[index];
                if current.is_none() {
                    new_seats.insert(index, current.to_owned());
                    continue;
                }
                let adjacent_occupied = self.count_occupied_adjacent(x, y);
                if current == &Some(Seat::Empty) && adjacent_occupied == 0 {
                    new_seats.insert(index, Some(Seat::Occupied));
                } else if current == &Some(Seat::Occupied) && adjacent_occupied >= 4 {
                    new_seats.insert(index, Some(Seat::Empty));
                } else {
                    new_seats.insert(index, current.to_owned());
                }
            }
        }

        Self {
            width: self.width,
            height: self.height,
            seats: new_seats,
        }
    }

    fn apply_round_visible(&self, visibility_graph: &HashMap<usize, Vec<usize>>) -> Self {
        let mut new_seats = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y * self.width + x) as usize;
                let current = &self.seats[index];
                if current.is_none() {
                    new_seats.insert(index, current.to_owned());
                    continue;
                }
                let visible_occupied = visibility_graph[&index]
                    .iter()
                    .filter(|&&visible| self.seats[visible] == Some(Seat::Occupied))
                    .count();
                if current == &Some(Seat::Empty) && visible_occupied == 0 {
                    new_seats.insert(index, Some(Seat::Occupied));
                } else if current == &Some(Seat::Occupied) && visible_occupied >= 5 {
                    new_seats.insert(index, Some(Seat::Empty));
                } else {
                    new_seats.insert(index, current.to_owned());
                }
            }
        }

        Self {
            width: self.width,
            height: self.height,
            seats: new_seats,
        }
    }
}

fn apply_rounds_until_stable_adjacent(seat_grid: SeatGrid) -> SeatGrid {
    let mut next = seat_grid.apply_round_adjacent();
    let mut prev = seat_grid;
    while next != prev {
        prev = next.clone();
        next = prev.apply_round_adjacent();
    }

    next
}

fn apply_rounds_until_stable_visible(seat_grid: SeatGrid) -> SeatGrid {
    let visibility_graph = seat_grid.get_visibility_graph();
    let mut next = seat_grid.apply_round_visible(&visibility_graph);
    let mut prev = seat_grid;
    while next != prev {
        prev = next.clone();
        next = prev.apply_round_visible(&visibility_graph);
    }

    next
}

pub fn p1() -> usize {
    apply_rounds_until_stable_adjacent(SeatGrid::parse(INPUT)).total_occupied()
}

pub fn p2() -> usize {
    apply_rounds_until_stable_visible(SeatGrid::parse(INPUT)).total_occupied()
}

#[cfg(test)]
mod test {
    use super::*;

    static SIMPLE: &str = r#"........
.LLL.LLL
###.###.
.LLL.LLL
###.###."#;

    static EXAMPLE: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"#;

    #[test]
    fn p1_simple() {
        let grid = SeatGrid::parse(SIMPLE);

        assert_eq!(2, grid.count_occupied_adjacent(3, 1));
        assert_eq!(6, grid.count_occupied_adjacent(1, 3));
        assert_eq!(1, grid.count_occupied_adjacent(6, 4));
    }

    #[test]
    fn p1_example() {
        let stable = apply_rounds_until_stable_adjacent(SeatGrid::parse(EXAMPLE));

        assert_eq!(37, stable.total_occupied());
    }

    #[test]
    fn p1_correct_answer() {
        let stable = apply_rounds_until_stable_adjacent(SeatGrid::parse(INPUT));

        assert_eq!(2319, stable.total_occupied());
    }

    #[test]
    fn p2_example() {
        let stable = apply_rounds_until_stable_visible(SeatGrid::parse(EXAMPLE));

        assert_eq!(26, stable.total_occupied());
    }

    #[test]
    fn p2_correct_answer() {
        let stable = apply_rounds_until_stable_visible(SeatGrid::parse(INPUT));

        assert_eq!(2117, stable.total_occupied());
    }
}
