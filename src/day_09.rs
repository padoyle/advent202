use std::collections::HashSet;

static INPUT: &'static str = include_str!("assets/day_09_input.txt");

fn has_sum(lookback: &[i64], target: i64) -> bool {
    let lookback_set: HashSet<&i64> = lookback.iter().collect();
    for value in lookback_set.iter() {
        let matching_value = target - *value;
        if lookback_set.contains(&matching_value) {
            return true;
        }
    }
    false
}

fn find_first_invalid_value(sequence: &Vec<i64>, lookback: usize) -> i64 {
    let mut index = lookback;
    while index < sequence.len() {
        let lookback_sequence = &sequence[(index - lookback)..index];
        if !has_sum(lookback_sequence, sequence[index]) {
            return sequence[index];
        }
        index += 1;
    }
    panic!("no invalid values");
}

fn find_encryption_weakness(sequence: &Vec<i64>, target_sum: i64) -> i64 {
    let mut min_ptr = 0;
    let mut max_ptr = 1;
    loop {
        let range = &sequence[min_ptr..max_ptr];
        let sum: i64 = range.iter().sum();
        if sum == target_sum {
            return range.iter().min().unwrap() + range.iter().max().unwrap();
        }
        if sum > target_sum {
            min_ptr += 1;
        }
        if sum < target_sum {
            max_ptr += 1;
        }
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn p1() -> i64 {
    let sequence = parse_input(INPUT);
    find_first_invalid_value(&sequence, 25)
}

pub fn p2() -> i64 {
    let sequence = parse_input(INPUT);
    let invalid_value = find_first_invalid_value(&sequence, 25);
    find_encryption_weakness(&sequence, invalid_value)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"#;

    #[test]
    fn p1_example() {
        let sequence = parse_input(EXAMPLE);

        assert_eq!(127, find_first_invalid_value(&sequence, 5));
    }

    #[test]
    fn p1_correct_answer() {
        let sequence = parse_input(INPUT);

        assert_eq!(2089807806, find_first_invalid_value(&sequence, 25))
    }

    #[test]
    fn p2_example() {
        let sequence = parse_input(EXAMPLE);

        assert_eq!(62, find_encryption_weakness(&sequence, 127));
    }

    #[test]
    fn p2_correct_answer() {
        let sequence = parse_input(INPUT);

        assert_eq!(245848639, find_encryption_weakness(&sequence, 2089807806))
    }
}
