use std::collections::HashMap;

static INPUT: &'static str = include_str!("assets/day_16_input.txt");

#[derive(Debug)]
struct RangePair {
    low_range: (u32, u32),
    high_range: (u32, u32),
}

impl From<(&str, &str)> for RangePair {
    fn from(input: (&str, &str)) -> Self {
        let mut low_range_tokens = input.0.split("-");
        let low_range = (
            low_range_tokens.next().unwrap().parse().unwrap(),
            low_range_tokens.next().unwrap().parse().unwrap(),
        );
        let mut high_range_tokens = input.1.split("-");
        let high_range = (
            high_range_tokens.next().unwrap().parse().unwrap(),
            high_range_tokens.next().unwrap().parse().unwrap(),
        );

        Self {
            low_range,
            high_range,
        }
    }
}

impl RangePair {
    fn fits(&self, value: u32) -> bool {
        (value >= self.low_range.0 && value <= self.low_range.1)
            || (value >= self.high_range.0 && value <= self.high_range.1)
    }
}

#[derive(Debug)]
struct TicketInfo<'a> {
    field_rules: HashMap<&'a str, RangePair>,
    my_ticket: Vec<u32>,
    nearby_tickets: &'a str,
}

fn parse_field_rule<'a>(input: &'a str) -> (&'a str, RangePair) {
    let mut first_split = input.split(": ");
    let field = first_split.next().unwrap();
    let mut remaining_split = first_split.next().unwrap().split_whitespace();
    let low_range_str = remaining_split.next().unwrap();
    remaining_split.next().unwrap(); // skip 'or'
    let high_range_str = remaining_split.next().unwrap();

    (field, RangePair::from((low_range_str, high_range_str)))
}

fn parse_fields(fields: &str) -> Vec<u32> {
    fields
        .split(',')
        .filter_map(|value_str| value_str.trim().parse().ok())
        .collect()
}

fn parse_input<'a>(input: &'a str) -> TicketInfo<'a> {
    let mut first_split = input.split("\nnearby tickets:\n");
    let preamble = first_split.next().unwrap();
    let nearby_tickets = first_split.next().unwrap();
    let mut second_split = preamble.split("\nyour ticket:\n");
    let field_rules = second_split
        .next()
        .unwrap()
        .lines()
        .map(parse_field_rule)
        .collect();
    let my_ticket = parse_fields(second_split.next().unwrap());

    TicketInfo {
        field_rules,
        my_ticket,
        nearby_tickets,
    }
}

fn get_error_rate(input: &str) -> u32 {
    let info = parse_input(input);

    let invalid_values: Vec<u32> = info
        .nearby_tickets
        .lines()
        .map(|line| {
            let field_values = parse_fields(line);
            field_values.into_iter().filter(|field_value| {
                !info
                    .field_rules
                    .values()
                    .any(|rule| rule.fits(*field_value))
            })
        })
        .flatten()
        .collect();

    invalid_values.iter().sum()
}

pub fn p1() -> u32 {
    get_error_rate(INPUT)
}

pub fn p2() -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
two lines: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    #[test]
    fn p1_example() {
        assert_eq!(71, get_error_rate(EXAMPLE));
    }

    // #[test]
    // fn p1_correct_answer() {
    // }

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
