static INPUT: &'static str = include_str!("assets/day_13_input.txt");

fn get_soonest_bus(start: i32, bus_ids: Vec<i32>) -> (i32, i32) {
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

fn get_mult_time(input: &str) -> i32 {
    let mut input_lines = input.lines();
    let start_time: i32 = input_lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<i32> = input_lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|token| token.parse::<i32>().ok())
        .collect();

    let (id, time) = get_soonest_bus(start_time, bus_ids);
    id * time
}

pub fn p1() -> i32 {
    get_mult_time(INPUT)
}

pub fn p2() -> i32 {
    0
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

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
