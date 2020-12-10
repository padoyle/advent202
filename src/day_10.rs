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

fn resolve_sequence(adapters: &Vec<i32>, start_index: usize, cache: &mut Vec<u64>) -> u64 {
    // base case, we've found a complete sequence
    if start_index == adapters.len() - 1 {
        return 1;
    }
    let mut possible_sequences = 0;
    let last_index = std::cmp::min(start_index + 3, adapters.len() - 1);
    for i in (start_index + 1)..=last_index {
        if adapters[i] - adapters[start_index] > 3 {
            break;
        }
        if cache[i] == 0 {
            cache[i] = resolve_sequence(adapters, i, cache);
        }
        possible_sequences += cache[i];
    }

    possible_sequences
}

fn find_all_sequences(adapters: Vec<i32>) -> u64 {
    let mut adapters = adapters;
    order_adpaters(&mut adapters);
    let mut cache: Vec<u64> = vec![0; adapters.len()];
    resolve_sequence(&adapters, 0, &mut cache)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn p1() -> usize {
    find_joltage_differences(parse_input(INPUT))
}

pub fn p2() -> u64 {
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

    #[test]
    fn p2_correct_answer() {
        let adapters = parse_input(INPUT);

        assert_eq!(3454189699072, find_all_sequences(adapters));
    }
}
