use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_07_input.txt");

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    bag_color: &'a str,
    contains: HashMap<&'a str, usize>,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        let rule_parts: Vec<&str> = s.splitn(2, " bags contain ").collect();
        match rule_parts.as_slice() {
            [bag_color, contains_str] => {
                if *contains_str == "no other bags." {
                    Rule {
                        bag_color,
                        contains: HashMap::new(),
                    }
                } else {
                    let contains: HashMap<&str, usize> = contains_str
                        .split(',')
                        .map(|rule| {
                            let rule = rule.trim();
                            let count: usize =
                                rule.split_whitespace().next().unwrap().parse().unwrap();
                            let end_index = rule.find(" bag").unwrap();

                            (&rule[2..end_index], count)
                        })
                        .collect();
                    Rule {
                        bag_color,
                        contains,
                    }
                }
            }
            _ => panic!("invalid input"),
        }
    }
}

#[derive(Debug)]
struct RuleGraph<'a> {
    rules: HashMap<&'a str, HashMap<&'a str, usize>>,
}

impl<'a> From<&'a str> for RuleGraph<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            rules: s
                .lines()
                .map(Rule::from)
                .map(|rule| (rule.bag_color, rule.contains))
                .collect(),
        }
    }
}

impl<'a> RuleGraph<'a> {
    fn is_possible_container(&self, bag_color: &'a str, container: &'a str) -> bool {
        for (child_container, _) in self.rules.get(container).unwrap() {
            if *child_container == bag_color {
                return true;
            }

            let recurse = self.is_possible_container(bag_color, child_container);
            if recurse {
                return true;
            }
        }
        false
    }

    fn find_possible_containers(&self, bag_color: &'a str) -> usize {
        let possible_containers: HashSet<&'a str> = self
            .rules
            .keys()
            .filter(|container| self.is_possible_container(bag_color, container))
            // dunno how to elegantly avoid this deref mapping
            .map(|value| *value)
            .collect();

        possible_containers.len()
    }

    fn count_contained_bags(&self, bag_color: &'a str) -> usize {
        let contained = self.rules.get(bag_color).unwrap();

        // inefficient solution
        let result = contained
            .iter()
            .map(|(child, count)| count * (1 + self.count_contained_bags(child)))
            .sum();

        result
    }
}

pub fn p1() -> usize {
    RuleGraph::from(INPUT).find_possible_containers("shiny gold")
}

pub fn p2() -> usize {
    RuleGraph::from(INPUT).count_contained_bags("shiny gold")
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    static EXAMPLE2: &str = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;

    #[test]
    fn rule_parsing() {
        let rule_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let empty_rule_str = "dotted black bags contain no other bags.";

        assert_eq!(
            Rule {
                bag_color: "light red",
                contains: hashmap! {
                    "bright white" => 1,
                    "muted yellow" => 2,
                }
            },
            Rule::from(rule_str)
        );

        assert_eq!(
            Rule {
                bag_color: "dotted black",
                contains: hashmap! {},
            },
            Rule::from(empty_rule_str)
        );
    }

    #[test]
    fn p1_example() {
        let rule_graph = RuleGraph::from(EXAMPLE);

        assert_eq!(4, rule_graph.find_possible_containers("shiny gold"));
    }

    #[test]
    fn p1_correct_answer() {
        let rule_graph = RuleGraph::from(INPUT);

        assert_eq!(169, rule_graph.find_possible_containers("shiny gold"));
    }

    #[test]
    fn p2_example() {
        let rule_graph = RuleGraph::from(EXAMPLE);
        let rule_graph2 = RuleGraph::from(EXAMPLE2);

        assert_eq!(32, rule_graph.count_contained_bags("shiny gold"));
        assert_eq!(126, rule_graph2.count_contained_bags("shiny gold"));
    }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
