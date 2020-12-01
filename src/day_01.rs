static INPUT: &'static str = include_str!("assets/day_01_input.txt");

fn process_input() -> Vec<u32> {
    INPUT
        .lines()
        .map(|line| line.parse::<u32>().expect("malformed input"))
        .collect()
}

fn sum_2020(inputs: &Vec<u32>) -> (u32, u32) {
    let length = inputs.len();
    // Extremely naive approach o.O
    for i in 0..length {
        for j in 0..length {
            if i == j {
                continue;
            }
            let (first, second) = (inputs[i], inputs[j]);
            if first + second == 2020 {
                return (first, second);
            }
        }
    }

    unreachable!("Input contained no valid answer");
}

pub fn get_p1_answer() -> u32 {
    let inputs = process_input();
    let (a, b) = sum_2020(&inputs);

    a * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_simple() {
        let inputs = vec![123, 1897];
        let (a, b) = sum_2020(&inputs);

        assert_eq!(a * b, 123 * 1897);
    }

    #[test]
    fn p1_no_reuse() {
        let inputs = vec![1010, 2000, 20];
        let (a, b) = sum_2020(&inputs);

        assert_eq!(a * b, 2000 * 20);
    }

    #[test]
    fn p1_example() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = sum_2020(&inputs);

        assert_eq!(a * b, 514579);
    }

    #[test]
    fn p1_correct_answer() {
        let inputs = process_input();
        let (a, b) = sum_2020(&inputs);

        assert_eq!(a * b, 224436);
    }
}
