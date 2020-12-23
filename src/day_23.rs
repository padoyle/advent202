static INPUT: [usize; 9] = [4, 6, 7, 5, 2, 8, 1, 9, 3];

struct Cups {
    max: usize,
    nodes: Vec<usize>,
}

impl Cups {
    fn from_input(input: &[usize], extend: usize) -> Self {
        let mut nodes = vec![0; input.len() + 1];
        // There will be a dummy value (0) at index 0, which is honestly just easier
        for i in 0..input.len() - 1 {
            nodes[input[i]] = input[i + 1];
        }
        if extend > 0 {
            for i in input.len()..extend - 1 {
                nodes.push(i + 2);
            }
            nodes[input[input.len() - 1]] = input.len() + 1;

            nodes.push(input[0]);
        } else {
            nodes[input[input.len() - 1]] = input[0];
        }

        Self {
            max: std::cmp::max(input.len(), extend),
            nodes,
        }
    }

    fn move_three(&mut self, current: usize) {
        let mut dest = if current > 1 { current - 1 } else { self.max };
        let first = self.nodes[current];
        let second = self.nodes[first];
        let third = self.nodes[second];
        while dest == first || dest == second || dest == third {
            dest -= 1;
            if dest < 1 {
                dest = self.max;
            }
        }

        self.nodes.swap(current, third);
        self.nodes.swap(third, dest);
    }
}

fn play_game(initial_cups: &[usize], extend: usize, moves: usize) -> Cups {
    let mut cups = Cups::from_input(initial_cups, extend);
    let mut current = initial_cups[0];
    for _ in 0..moves {
        cups.move_three(current);
        current = cups.nodes[current];
    }
    cups
}

fn get_result_p1(cups: &Cups) -> String {
    let mut result = String::new();
    let mut next = cups.nodes[1];
    loop {
        result.push_str(next.to_string().as_str());
        next = cups.nodes[next];
        if next == 1 {
            break;
        }
    }
    result
}

fn get_result_p2(cups: &Cups) -> u64 {
    let a = cups.nodes[1];
    let b = cups.nodes[a];
    (a as u64) * (b as u64)
}

pub fn p1() -> String {
    let cups = play_game(&INPUT, 0, 100);
    get_result_p1(&cups)
}

pub fn p2() -> u64 {
    let cups = play_game(&INPUT, 1_000_000, 10_000_000);
    get_result_p2(&cups)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn p1_example() {
        let cups = play_game(&EXAMPLE, 0, 100);
        assert_eq!("67384529", get_result_p1(&cups));
    }

    #[test]
    fn p1_correct_answer() {
        let cups = play_game(&INPUT, 0, 100);
        assert_eq!("43769582", get_result_p1(&cups));
    }

    #[test]
    fn p2_example() {
        let cups = play_game(&EXAMPLE, 1_000_000, 10_000_000);
        assert_eq!(149245887792, get_result_p2(&cups));
    }

    #[test]
    fn p2_correct_answer() {
        let cups = play_game(&INPUT, 1_000_000, 10_000_000);
        assert_eq!(264692662390, get_result_p2(&cups));
    }
}
