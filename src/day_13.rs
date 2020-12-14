use std::collections::HashMap;

static INPUT: &'static str = include_str!("assets/day_13_input.txt");

fn get_soonest_bus(start: i64, bus_ids: Vec<i64>) -> (i64, i64) {
    let mut min_id = -1;
    let mut min_time = -1;
    for bus_id in bus_ids {
        let next_bus_time = bus_id - (start % bus_id);
        if min_id == -1 || next_bus_time < min_time {
            min_id = bus_id;
            min_time = next_bus_time;
        }
    }

    (min_id, min_time)
}

fn get_mult_time(input: &str) -> i64 {
    let mut input_lines = input.lines();
    let start_time: i64 = input_lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<i64> = input_lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|token| token.parse::<i64>().ok())
        .collect();

    let (id, time) = get_soonest_bus(start_time, bus_ids);
    id * time
}

fn naive_mmi(target: i64, modulo: i64) -> i64 {
    let mut i = 1;
    loop {
        if (target * i) % modulo == 1 {
            break i;
        }
        i += 1;
    }
}

fn calculate_crt(value_mod_pairs: &HashMap<i64, i64>) -> i64 {
    let prod: i64 = value_mod_pairs.keys().product();
    let mut result = 0;
    for (&id, &modulo) in value_mod_pairs.iter() {
        let y = prod / id;
        let z = naive_mmi(y, id);

        result += modulo * y * z;
    }

    result % prod
}

fn get_earliest_departure_sequence(bus_ids_input: &str) -> i64 {
    let bus_ids: HashMap<usize, i64> = bus_ids_input
        .split(',')
        .enumerate()
        .filter_map(|(i, id)| id.parse::<i64>().ok().map(|id| (i, id)))
        .collect();

    let with_modulos: HashMap<i64, i64> = bus_ids
        .iter()
        .map(|(index, id)| (*id, id - (*index as i64)))
        .collect();

    calculate_crt(&with_modulos)
}

pub fn p1() -> i64 {
    get_mult_time(INPUT)
}

pub fn p2() -> i64 {
    let bus_ids_input = INPUT.lines().skip(1).next().unwrap();
    get_earliest_departure_sequence(bus_ids_input)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn p1_example() {
        assert_eq!(295, get_mult_time(EXAMPLE));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(104, get_mult_time(INPUT));
    }

    #[test]
    fn p2_crt_math() {
        let earliest = calculate_crt(&hashmap! {
            3 => 2,
            4 => 3,
            5 => 1
        });

        assert_eq!(11, earliest);
    }

    #[test]
    fn p2_example() {
        let earliest = get_earliest_departure_sequence("7,13,x,x,59,x,31,19");

        assert_eq!(1068781, earliest);
    }

    #[test]
    fn p2_correct_answer() {
        let earliest = get_earliest_departure_sequence(INPUT.lines().skip(1).next().unwrap());

        assert_eq!(842186186521918, earliest)
    }
}
