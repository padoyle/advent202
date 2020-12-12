static INPUT: &'static str = include_str!("assets/day_12_input.txt");

struct Ship {
    x: f64,
    y: f64,
    dir: i64,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
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
                self.x += (value as f64) * theta.cos();
                self.y += (value as f64) * theta.sin();
            }
            _ => panic!("unsupported instruction"),
        }
    }

    fn manhattan_dist_to_origin(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    fn follow_path(&mut self, instructions: &str) -> i64 {
        for line in instructions.lines() {
            self.navigate(line);
        }
        self.manhattan_dist_to_origin() as i64
    }
}

pub fn p1() -> i64 {
    Ship::new().follow_path(INPUT)
}

pub fn p2() -> f64 {
    0.0
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

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
