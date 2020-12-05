static INPUT: &'static str = include_str!("assets/day_05_input.txt");

fn get_seat_id(boarding_pass: &str) -> u32 {
    let mut row: u32 = 0;
    let mut col: u32 = 0;
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

fn highest_seat_id(input: &str) -> u32 {
    input
        .lines()
        .map(|boarding_pass| get_seat_id(boarding_pass))
        .max()
        .expect("invalid input")
}

pub fn p1() -> u32 {
    highest_seat_id(INPUT)
}

pub fn p2() -> usize {
    0
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

    // #[test]
    // fn p2_example() {}

    // #[test]
    // fn p2_correct_answer() {}
}
