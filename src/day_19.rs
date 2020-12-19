use std::collections::{BTreeMap, HashMap};

static INPUT: &str = include_str!("assets/day_19_input.txt");
static INPUT2: &str = include_str!("assets/day_19_input2.txt");

#[derive(Debug)]
enum Rule {
    Literal(char),
    Sequence(Vec<usize>),
    SeqChoice(Vec<usize>, Vec<usize>),
}

#[derive(Debug)]
struct RuleSet {
    rules: BTreeMap<usize, Rule>,
}

impl RuleSet {
    fn is_message_valid(&self, message: &str) -> bool {
        let (check, last_index) = self.check_message(message.as_bytes(), 0, 0);
        println!("{}; {} / {}", check, last_index, message.len());
        check && last_index == message.len()
    }

    fn check_message(&self, message: &[u8], index: usize, rule_id: usize) -> (bool, usize) {
        if index >= message.len() {
            return (false, index);
        }

        let rule = self.rules.get(&rule_id).unwrap();
        match rule {
            Rule::Literal(value) => ((message[index] as char) == *value, index + 1),
            Rule::Sequence(seq_rules) => {
                let mut next_msg_index = index;
                for sub_index in 0..seq_rules.len() {
                    let (next_match, next_index) =
                        self.check_message(message, next_msg_index, seq_rules[sub_index]);
                    if !next_match {
                        println!(
                            "[{}] Sequence {:?} failed match at {}",
                            rule_id, seq_rules, sub_index
                        );
                        return (false, index);
                    }
                    next_msg_index = next_index;
                }
                // println!(
                //     "[{}] Sequence {:?} at {}: true\n\tjump to {}",
                //     rule_id, seq_rules, index, next_msg_index
                // );
                (true, next_msg_index)
            }
            Rule::SeqChoice(rules_left, rules_right) => {
                let mut matches = true;
                let mut next_msg_index = index;
                for sub_index in 0..rules_left.len() {
                    let (next_match, next_index) =
                        self.check_message(message, next_msg_index, rules_left[sub_index]);
                    matches = next_match;
                    if !matches {
                        break;
                    }
                    next_msg_index = next_index;
                }
                if matches {
                    // println!(
                    //     "[{}] Left sequence {:?} at {}: true\n\tjump to {}",
                    //     rule_id, rules_left, index, next_msg_index
                    // );
                    return (true, next_msg_index);
                }

                matches = true;
                next_msg_index = index;
                for sub_index in 0..rules_right.len() {
                    let (next_match, next_index) =
                        self.check_message(message, next_msg_index, rules_right[sub_index]);
                    matches = next_match;
                    if !matches {
                        break;
                    }
                    next_msg_index = next_index;
                }
                if matches {
                    // println!(
                    //     "[{}] Right sequence {:?} at {}: true\n\tjump to {}",
                    //     rule_id, rules_right, index, next_msg_index
                    // );
                    return (true, next_msg_index);
                }

                println!(
                    "[{}] Both {:?} and {:?} at {}: false",
                    rule_id, rules_left, rules_right, index
                );
                (false, index)
            }
        }
    }
}

fn get_sequence(seq_str: &str) -> Vec<usize> {
    seq_str
        .trim()
        .split_whitespace()
        .map(|value| value.parse().unwrap())
        .collect()
}

fn parse_rule_set(input: &str) -> RuleSet {
    let rules = input
        .lines()
        .map(|line| {
            let map_split: Vec<&str> = line.split(':').collect();
            let rule_index: usize = map_split[0].parse().unwrap();

            let rule_tokens: Vec<&str> = map_split[1].trim().split('|').collect();
            let rule = match rule_tokens.as_slice() {
                [one_value] => {
                    if one_value.starts_with("\"") {
                        Rule::Literal(one_value.chars().nth(1).unwrap())
                    } else {
                        Rule::Sequence(get_sequence(one_value))
                    }
                }
                [opt_a, opt_b] => Rule::SeqChoice(get_sequence(opt_a), get_sequence(opt_b)),
                _ => panic!("Bad input"),
            };

            (rule_index, rule)
        })
        .collect();

    RuleSet { rules }
}

fn parse_input(input: &str) -> (RuleSet, &str) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert!(parts.len() == 2);

    (parse_rule_set(parts[0]), parts[1])
}

pub fn count_valid_messages(input: &str) -> usize {
    let (rule_set, messages) = parse_input(input);

    for (k, v) in rule_set.rules.iter() {
        println!("{}: {:?}", k, v);
    }
    println!("\n");
    messages
        .lines()
        .filter(|message| {
            println!("Message: {}", message);
            let check = rule_set.is_message_valid(message);
            println!("{}: {}", message, check);
            check
        })
        .count()
}

pub fn p1() -> usize {
    count_valid_messages(INPUT)
}

pub fn p2() -> usize {
    count_valid_messages(INPUT2)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    static EXAMPLE2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    #[test]
    fn p1_example() {
        assert_eq!(2, count_valid_messages(EXAMPLE));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(111, count_valid_messages(INPUT));
    }

    #[test]
    fn p2_example() {
        assert_eq!(12, count_valid_messages(EXAMPLE2));
    }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
