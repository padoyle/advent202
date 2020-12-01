static INPUT: &'static str = include_str!("assets/day_01_input.txt");

fn process_input() -> Vec<u32> {
    INPUT
        .lines()
        .map(|line| line.parse::<u32>().expect("malformed input"))
        .collect()
}

fn sum2_2020(inputs: &Vec<u32>) -> (u32, u32) {
    let length = inputs.len();
    // Extremely naive approach o.O
    for i in 0..length {
        for j in 0..length {
            if i == j {
                continue;
            }
            let (a, b) = (inputs[i], inputs[j]);
            if a + b == 2020 {
                return (a, b);
            }
        }
    }

    unreachable!("Input contained no valid answer");
}

fn sum3_2020(inputs: &Vec<u32>) -> (u32, u32, u32) {
    let length = inputs.len();
    // Extremely naive approach o.O
    for i in 0..length {
        for j in 0..length {
            for k in 0..length {
                if i == j || j == k || i == k {
                    continue;
                }
                let (a, b, c) = (inputs[i], inputs[j], inputs[k]);
                if a + b + c == 2020 {
                    return (a, b, c);
                }
            }
        }
    }

    unreachable!("Input contained no valid answer");
}

pub fn p1() -> u32 {
    let inputs = process_input();
    let (a, b) = sum2_2020(&inputs);

    a * b
}

pub fn p2() -> u32 {
    let inputs = process_input();
    let (a, b, c) = sum3_2020(&inputs);

    a * b * c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_simple() {
        let inputs = vec![123, 1897];
        let (a, b) = sum2_2020(&inputs);

        assert_eq!(a * b, 123 * 1897);
    }

    #[test]
    fn p1_no_reuse() {
        let inputs = vec![1010, 2000, 20];
        let (a, b) = sum2_2020(&inputs);

        assert_eq!(a * b, 2000 * 20);
    }

    #[test]
    fn p1_example() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b) = sum2_2020(&inputs);

        assert_eq!(a * b, 514579);
    }

    #[test]
    fn p1_correct_answer() {
        let inputs = process_input();
        let (a, b) = sum2_2020(&inputs);

        assert_eq!(a * b, 224436);
    }

    #[test]
    fn p2_simple() {
        let inputs = vec![123, 1800, 97];
        let (a, b, c) = sum3_2020(&inputs);

        assert_eq!(a * b * c, 123 * 1800 * 97);
    }

    #[test]
    fn p2_example() {
        let inputs = vec![1721, 979, 366, 299, 675, 1456];
        let (a, b, c) = sum3_2020(&inputs);

        assert_eq!(a * b * c, 241861950);
    }
}
