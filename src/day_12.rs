static INPUT: &'static str = include_str!("assets/day_12_input.txt");

struct Ship {
    x: f64,
    y: f64,
    way_x: f64,
    way_y: f64,
    dir: i64,
}

fn to_rect(r: f64, theta: f64) -> (f64, f64) {
    let x = r * theta.cos();
    let y = r * theta.sin();

    (x, y)
}

fn rotate_xy(x: f64, y: f64, value: i32) -> (f64, f64) {
    match value {
        90 | -270 => (-y, x),
        180 | -180 => (-x, -y),
        270 | -90 => (y, -x),
        _ => unreachable!(),
    }
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            way_x: 10.0,
            way_y: 1.0,
            dir: 0,
        }
    }

    fn navigate(&mut self, nav_action: &str) {
        let instruction = &nav_action[..1];
        let value: i64 = nav_action[1..].parse().unwrap();
        match instruction {
            "N" => self.y += value as f64,
            "S" => self.y -= value as f64,
            "E" => self.x += value as f64,
            "W" => self.x -= value as f64,
            "L" => self.dir += value,
            "R" => self.dir -= value,
            "F" => {
                let theta = (self.dir as f64).to_radians();
                let (x, y) = to_rect(value as f64, theta);
                self.x += x;
                self.y += y;
            }
            _ => panic!("unsupported instruction"),
        }
    }

    fn manhattan_dist_to_origin(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    fn follow_path(&mut self, instructions: &str) -> i32 {
        for line in instructions.lines() {
            self.navigate(line);
        }
        self.manhattan_dist_to_origin() as i32
    }

    fn navigate_waypoint(&mut self, nav_action: &str) {
        let instruction = &nav_action[..1];
        let value: i32 = nav_action[1..].parse().unwrap();
        match instruction {
            "N" => self.way_y += value as f64,
            "S" => self.way_y -= value as f64,
            "E" => self.way_x += value as f64,
            "W" => self.way_x -= value as f64,
            "L" => {
                let (x, y) = rotate_xy(self.way_x, self.way_y, value);
                self.way_x = x;
                self.way_y = y;
            }
            "R" => {
                let (x, y) = rotate_xy(self.way_x, self.way_y, -value);
                self.way_x = x;
                self.way_y = y;
            }
            "F" => {
                self.x += self.way_x * (value as f64);
                self.y += self.way_y * (value as f64);
            }
            _ => panic!("unsupported instruction"),
        }
    }

    fn follow_path_with_waypoint(&mut self, instructions: &str) -> i32 {
        for line in instructions.lines() {
            self.navigate_waypoint(line);
        }
        self.manhattan_dist_to_origin() as i32
    }
}

pub fn p1() -> i32 {
    Ship::new().follow_path(INPUT)
}

pub fn p2() -> i32 {
    Ship::new().follow_path_with_waypoint(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn p1_example() {
        let distance = Ship::new().follow_path(EXAMPLE);

        assert_eq!(25, distance);
    }

    #[test]
    fn p1_correct_answer() {
        let distance = Ship::new().follow_path(INPUT);

        assert_eq!(420, distance);
    }

    #[test]
    fn p2_example() {
        let distance = Ship::new().follow_path_with_waypoint(EXAMPLE);

        assert_eq!(286, distance);
    }

    #[test]
    fn p2_correct_answer() {
        let distance = Ship::new().follow_path_with_waypoint(INPUT);

        assert_eq!(42073, distance);
    }
}
