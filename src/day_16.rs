use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_16_input.txt");

#[derive(Debug)]
struct RangePair {
    low_range: (u64, u64),
    high_range: (u64, u64),
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
    fn fits(&self, value: u64) -> bool {
        (value >= self.low_range.0 && value <= self.low_range.1)
            || (value >= self.high_range.0 && value <= self.high_range.1)
    }
}

#[derive(Debug)]
struct TicketInfo<'a> {
    field_rules: HashMap<&'a str, RangePair>,
    my_ticket: Vec<u64>,
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

fn parse_fields(fields: &str) -> Vec<u64> {
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

fn get_error_rate(input: &str) -> u64 {
    let info = parse_input(input);

    let invalid_values: Vec<u64> = info
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

fn get_valid_tickets<'a>(info: &TicketInfo<'a>) -> Vec<Vec<u64>> {
    info.nearby_tickets
        .lines()
        .map(parse_fields)
        .filter(|field_values| {
            field_values
                .iter()
                .all(|value| info.field_rules.values().any(|rule| rule.fits(*value)))
        })
        .collect()
}

fn map_fields<'a>(info: &TicketInfo<'a>, tickets: Vec<Vec<u64>>) -> HashMap<&'a str, usize> {
    let mut narrowed_options: HashMap<&'a str, HashSet<usize>> = HashMap::new();
    let initial_set: HashSet<usize> = (0..info.my_ticket.len()).collect();
    for field in info.field_rules.keys() {
        narrowed_options.insert(*field, initial_set.clone());
    }

    // for each valid ticket, find any fields it _doesn't_ match
    for ticket in tickets {
        for (field, range) in info.field_rules.iter() {
            let field_options = narrowed_options.get_mut(field).unwrap();
            for (i, value) in ticket.iter().enumerate() {
                if !range.fits(*value) {
                    // println!("Field {} couldn't possibly be at {}, since value {} doesn't fit range {:?}", field, i, value, range);
                    field_options.remove(&i);
                }
            }
        }
    }

    let mut resolved: HashMap<&str, usize> = HashMap::new();
    while resolved.len() < info.my_ticket.len() {
        let mut found_field: Option<&str> = None;
        let mut found_index: Option<usize> = None;
        for (&field, options) in narrowed_options.iter() {
            if options.len() == 1 {
                found_field = Some(field);
                found_index = Some(options.iter().next().unwrap().to_owned());
            }
        }
        narrowed_options.iter_mut().for_each(|(_, options)| {
            options.remove(found_index.as_ref().unwrap());
        });
        resolved.insert(found_field.unwrap(), found_index.unwrap());
    }

    resolved
}

pub fn p1() -> u64 {
    get_error_rate(INPUT)
}

pub fn p2() -> u64 {
    let ticket_info = parse_input(INPUT);
    let tickets = get_valid_tickets(&ticket_info);
    let fields = map_fields(&ticket_info, tickets);

    fields
        .iter()
        .filter_map(|(field, index)| {
            if field.find("departure").is_some() {
                Some(ticket_info.my_ticket[*index])
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    static EXAMPLE2: &str = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;

    #[test]
    fn p1_example() {
        assert_eq!(71, get_error_rate(EXAMPLE));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(26980, get_error_rate(INPUT));
    }

    #[test]
    fn p2_example() {
        let ticket_info = parse_input(EXAMPLE2);
        let tickets = get_valid_tickets(&ticket_info);
        let fields = map_fields(&ticket_info, tickets);

        assert_eq!(
            hashmap! {
                "row" => 0,
                "class" => 1,
                "seat" => 2,
            },
            fields
        )
    }

    #[test]
    fn p2_correct_answer() {
        assert_eq!(3021381607403, p2());
    }
}
