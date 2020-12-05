static INPUT: &'static str = include_str!("assets/day_05_input.txt");

fn get_seat_id(boarding_pass: &str) -> usize {
    let mut row: usize = 0;
    let mut col: usize = 0;
    for c in boarding_pass[..7].chars() {
        if c == 'B' {
            row += 1;
        }
        row <<= 1;
    }
    for c in boarding_pass[7..].chars() {
        if c == 'R' {
            col += 1;
        }
        col <<= 1;
    }
    // We don't actually want the last shift, so it's easy enough to just unshift here
    (row >> 1) * 8 + (col >> 1)
}

fn highest_seat_id(input: &str) -> usize {
    input.lines().map(get_seat_id).max().expect("invalid input")
}

fn find_missing_seat(input: &str) -> usize {
    let mut filled_seats: Vec<usize> = input.lines().map(get_seat_id).collect();
    filled_seats.sort();

    // I'm certain there's a better fit for this than `fold` *shrug*
    let seat_before_missing = filled_seats.iter().fold(filled_seats[0], |prev, &value| {
        if prev + 1 == value {
            value
        } else {
            prev
        }
    });
    seat_before_missing + 1
}

pub fn p1() -> usize {
    highest_seat_id(INPUT)
}

pub fn p2() -> usize {
    find_missing_seat(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        assert_eq!(357, get_seat_id("FBFBBFFRLR"));
        assert_eq!(567, get_seat_id("BFFFBBFRRR"));
        assert_eq!(119, get_seat_id("FFFBBBFRRR"));
        assert_eq!(820, get_seat_id("BBFFBBFRLL"));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(826, highest_seat_id(INPUT))
    }

    #[test]
    fn p2_simple() {
        let values = r#"FBFBBFFLLL
FBFBBFFLLR
FBFBBFFLRL
FBFBBFFRLL
FBFBBFFRLR
FBFBBFFRRR
"#;

        assert_eq!(355, find_missing_seat(values));
    }

    #[test]
    fn p2_correct_answer() {
        assert_eq!(678, find_missing_seat(INPUT));
    }
}
