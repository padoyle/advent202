use std::collections::HashMap;

static INPUT: &'static str = include_str!("assets/day_10_input.txt");

fn order_adpaters(adapters: &mut Vec<i32>) {
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);
}

fn find_joltage_differences(mut adapters: Vec<i32>) -> usize {
    let (mut diff1, mut diff3) = (0, 0);
    order_adpaters(&mut adapters);

    for i in 1..adapters.len() {
        let (previous, current) = (adapters[i - 1], adapters[i]);
        match current - previous {
            1 => {
                diff1 += 1;
            }
            3 => {
                diff3 += 1;
            }
            _ => {}
        };
    }

    diff1 * diff3
}

fn resolve_sequence_from(
    adapters: &Vec<i32>,
    start_index: usize,
    cache: &mut HashMap<usize, usize>,
) -> usize {
    let mut remaining = std::cmp::min(adapters.len() - 1 - start_index, 3);
    if remaining == 0 {
        return 1;
    }
    let mut possible_sequences = 0;
    let current = adapters[start_index];
    while remaining > 0 {
        let target = start_index + remaining;
        if adapters[target] - current <= 3 {
            if let Some(precalculated) = cache.get(&target) {
                possible_sequences += precalculated;
            } else {
                let new_sequences = resolve_sequence_from(adapters, target, cache);
                cache.insert(target, new_sequences);
                possible_sequences += new_sequences;
            }
        }
        remaining -= 1;
    }

    possible_sequences
}

fn find_all_sequences(adapters: Vec<i32>) -> usize {
    let mut adapters = adapters;
    order_adpaters(&mut adapters);
    let mut cache = HashMap::new();
    resolve_sequence_from(&adapters, 0, &mut cache)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn p1() -> usize {
    find_joltage_differences(parse_input(INPUT))
}

pub fn p2() -> usize {
    find_all_sequences(parse_input(INPUT))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE1: &str = r#"16
10
15
5
1
11
7
19
6
12
4"#;

    static EXAMPLE2: &str = r#"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"#;

    #[test]
    fn p1_example() {
        let ex1 = parse_input(EXAMPLE1);
        let ex2 = parse_input(EXAMPLE2);

        assert_eq!(35, find_joltage_differences(ex1));
        assert_eq!(220, find_joltage_differences(ex2));
    }

    #[test]
    fn p1_correct_answer() {
        let adapters = parse_input(INPUT);

        assert_eq!(1820, find_joltage_differences(adapters));
    }

    #[test]
    fn p2_example() {
        let ex1 = parse_input(EXAMPLE1);
        let ex2 = parse_input(EXAMPLE2);

        assert_eq!(8, find_all_sequences(ex1));
        assert_eq!(19208, find_all_sequences(ex2));
    }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
