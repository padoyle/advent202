use std::collections::HashMap;

lazy_static! {
    static ref INPUT: Vec<usize> = vec![14, 3, 1, 0, 9, 5];
}

fn find_nth_number(starting_numbers: &Vec<usize>, target: usize) -> usize {
    let mut occurences: HashMap<usize, usize> = HashMap::new();
    for (i, value) in starting_numbers.iter().enumerate() {
        occurences.insert(*value, i);
    }
    let mut index = starting_numbers.len();
    let mut number = starting_numbers.last().unwrap().to_owned();
    while index < target {
        let prev_number = number;
        let last_occurence = occurences.get(&prev_number).copied().unwrap_or(index - 1);
        if last_occurence == index - 1 {
            // did not occur before index - 1, but is occurring now;
            number = 0;
        } else {
            // previously occurred (before index - 1);
            // update number to the (index - 1) - last_index (before index - 1)
            number = (index - 1) - last_occurence;
        }
        occurences.insert(prev_number, index - 1);
        // println!("sequence[{}] = {}", index, number);
        index += 1;
    }

    number
}

pub fn p1() -> usize {
    find_nth_number(&INPUT, 2020)
}

pub fn p2() -> usize {
    find_nth_number(&INPUT, 30000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        assert_eq!(0, find_nth_number(&vec![0, 3, 6], 10));
        assert_eq!(1, find_nth_number(&vec![1, 3, 2], 2020));
        assert_eq!(10, find_nth_number(&vec![2, 1, 3], 2020));
        assert_eq!(27, find_nth_number(&vec![1, 2, 3], 2020));
        assert_eq!(78, find_nth_number(&vec![2, 3, 1], 2020));
        assert_eq!(438, find_nth_number(&vec![3, 2, 1], 2020));
        assert_eq!(1836, find_nth_number(&vec![3, 1, 2], 2020));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(614, find_nth_number(&INPUT, 2020));
    }

    // These tests take a while! Re-enable if the solution is improved.

    #[test]
    #[ignore]
    fn p2_example() {
        assert_eq!(2578, find_nth_number(&vec![1, 3, 2], 30000000));
        assert_eq!(3544142, find_nth_number(&vec![2, 1, 3], 30000000));
        assert_eq!(261214, find_nth_number(&vec![1, 2, 3], 30000000));
        assert_eq!(6895259, find_nth_number(&vec![2, 3, 1], 30000000));
        assert_eq!(18, find_nth_number(&vec![3, 2, 1], 30000000));
        assert_eq!(362, find_nth_number(&vec![3, 1, 2], 30000000));
    }

    #[test]
    #[ignore]
    fn p2_correct_answer() {
        assert_eq!(1065, find_nth_number(&INPUT, 30000000));
    }
}
