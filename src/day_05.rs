static INPUT: &'static str = include_str!("assets/day_04_input.txt");

pub fn p1() -> usize {
    count_valid_p1(INPUT)
}

pub fn p2() -> usize {
    count_valid_p2(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn p1_example() {}

    // #[test]
    // fn p1_correct_answer() {}

    // #[test]
    // fn p2_example() {}

    // #[test]
    // fn p2_correct_answer() {}
}
