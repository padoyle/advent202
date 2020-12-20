use std::collections::HashMap;

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
    rules: HashMap<usize, Rule>,
}

// This solution was more or less borrowed wholesale from
// https://github.com/gokberkkocak/adventofcode/blob/master/src/aoc2020/day19.rs,
// after I struggled for quite a while to get a similar idea working!
impl RuleSet {
    fn resolve_sequence<'a>(&self, message: &'a str, sequence: &Vec<usize>) -> Vec<&'a str> {
        let mut candidates = vec![message];
        for rule in sequence {
            let mut new_candidates = Vec::new();
            for candidate in candidates {
                new_candidates.append(&mut self.resolve_message(candidate, *rule))
            }
            candidates = new_candidates;
        }
        candidates
    }

    fn resolve_message<'a>(&self, message: &'a str, rule: usize) -> Vec<&'a str> {
        if message.is_empty() {
            return vec![];
        }
        match self.rules.get(&rule).unwrap() {
            Rule::Literal(value) => message
                .strip_prefix(*value)
                .and_then(|sub| Some(vec![sub]))
                .unwrap_or_else(|| Vec::new()),
            Rule::Sequence(rules) => self.resolve_sequence(message, rules),
            Rule::SeqChoice(rules_l, rules_r) => {
                let mut candidates = Vec::new();
                candidates.append(&mut self.resolve_sequence(message, rules_l));
                candidates.append(&mut self.resolve_sequence(message, rules_r));
                candidates
            }
        }
    }
}

fn parse_sequence(seq_str: &str) -> Vec<usize> {
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
                        Rule::Sequence(parse_sequence(one_value))
                    }
                }
                [opt_a, opt_b] => Rule::SeqChoice(parse_sequence(opt_a), parse_sequence(opt_b)),
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

    messages
        .lines()
        .flat_map(|message| rule_set.resolve_message(message, 0))
        .filter(|resolved| resolved.is_empty())
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

    #[test]
    fn p2_correct_answer() {
        assert_eq!(343, count_valid_messages(INPUT2));
    }
}
