use std::str::FromStr;

static INPUT: &'static str = include_str!("assets/day_02_input.txt");

struct Rule {
    min: usize,
    max: usize,
    letter: char,
}

impl Rule {
    fn new(min: usize, max: usize, letter: char) -> Self {
        Self { min, max, letter }
    }
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split(' ').collect();
        if let [range, letter] = tokens.as_slice() {
            let range_tokens: Vec<&str> = range.split('-').collect();
            if let [min, max] = range_tokens.as_slice() {
                return Ok(Self {
                    min: min.parse().unwrap(),
                    max: max.parse().unwrap(),
                    letter: letter.parse().unwrap(),
                });
            }
        }

        Err("invalid formatting")
    }
}

fn process_input(input: &'static str) -> Vec<(Rule, &'static str)> {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(": ").collect();
            if let [rule, pass] = tokens.as_slice() {
                return (Rule::from_str(*rule).unwrap(), *pass);
            }

            panic!("invalid formatting")
        })
        .collect()
}

fn is_valid_p1(rule: &Rule, pass: &str) -> bool {
    let count = pass.chars().filter(|c| c == &rule.letter).count();

    count >= rule.min && count <= rule.max
}

fn get_valid_passwords_p1(passwords: Vec<(Rule, &'static str)>) -> usize {
    passwords
        .iter()
        .filter_map(|(rule, pass)| {
            if is_valid_p1(rule, pass) {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn is_valid_p2(rule: &Rule, pass: &str) -> bool {
    let chars: Vec<char> = pass.chars().collect();
    (chars[rule.min - 1] == rule.letter) ^ (chars[rule.max - 1] == rule.letter)
}

fn get_valid_passwords_p2(passwords: Vec<(Rule, &'static str)>) -> usize {
    passwords
        .iter()
        .filter_map(|(rule, pass)| {
            if is_valid_p2(rule, pass) {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

pub fn p1() -> usize {
    let input = process_input(INPUT);
    get_valid_passwords_p1(input)
}

pub fn p2() -> usize {
    let input = process_input(INPUT);
    get_valid_passwords_p2(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        let input = vec![
            (Rule::new(1, 3, 'a'), "abcde"),
            (Rule::new(1, 3, 'b'), "cdefg"),
            (Rule::new(2, 9, 'c'), "ccccccccc"),
        ];

        assert_eq!(2, get_valid_passwords_p1(input));
    }

    #[test]
    fn p1_more_cases() {
        let input = vec![
            (Rule::new(1, 1, 'a'), "abcde"),
            (Rule::new(1, 1, 'b'), "cdebbfg"),
            (Rule::new(1, 9, 'c'), "cccccccccc"),
        ];

        assert_eq!(1, get_valid_passwords_p1(input));
    }

    #[test]
    fn p1_correct_answer() {
        let input = process_input(INPUT);

        assert_eq!(542, get_valid_passwords_p1(input));
    }

    #[test]
    fn p2_example() {
        let input = vec![
            (Rule::new(1, 3, 'a'), "abcde"),
            (Rule::new(1, 3, 'b'), "cdefg"),
            (Rule::new(2, 9, 'c'), "ccccccccc"),
        ];

        assert_eq!(1, get_valid_passwords_p2(input));
    }

    #[test]
    fn p2_range_edges() {
        let input = vec![
            (Rule::new(1, 3, 'a'), "abcde"),
            (Rule::new(1, 3, 'a'), "bcade"),
            (Rule::new(1, 3, 'a'), "bcdae"),
        ];

        assert_eq!(2, get_valid_passwords_p2(input));
    }

    #[test]
    fn p2_correct_answer() {
        let input = process_input(INPUT);

        assert_eq!(360, get_valid_passwords_p2(input));
    }
}
