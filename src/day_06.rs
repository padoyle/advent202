use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_06_input.txt");

fn get_group_total(input: &str) -> usize {
    let letters_seen: HashSet<char> = input.chars().filter(char::is_ascii_lowercase).collect();
    letters_seen.len()
}

fn get_totals(input: &str) -> Vec<usize> {
    input.split("\n\n").map(get_group_total).collect()
}

pub fn p1() -> usize {
    get_totals(INPUT).iter().sum()
}

fn get_unanimous_group_total(input: &str) -> usize {
    let group_size = input.lines().count();
    let mut letters: HashMap<char, usize> = HashMap::new();
    input
        .chars()
        .filter(char::is_ascii_lowercase)
        .for_each(|letter| {
            letters.insert(letter, letters.get(&letter).unwrap_or(&0) + 1);
        });
    letters
        .iter()
        .filter(|(_, &count)| count == group_size)
        .count()
}

fn get_unanimous_totals(input: &str) -> Vec<usize> {
    input.split("\n\n").map(get_unanimous_group_total).collect()
}

pub fn p2() -> usize {
    get_unanimous_totals(INPUT).iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &'static str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn p1_example() {
        let actual = get_totals(EXAMPLE);
        let expected = vec![3, 3, 3, 1, 1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn p1_correct_answer() {
        let sum: usize = get_totals(INPUT).iter().sum();

        assert_eq!(sum, 6633);
    }

    #[test]
    fn p2_example() {
        let actual = get_unanimous_totals(EXAMPLE);
        let expected = vec![3, 0, 1, 1, 1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn p2_correct_answer() {
        let sum: usize = get_unanimous_totals(INPUT).iter().sum();

        assert_eq!(sum, 3202);
    }
}
