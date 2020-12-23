static INPUT: [u8; 9] = [4, 6, 7, 5, 2, 8, 1, 9, 3];

fn move_cups(cc: usize, cups: &mut Vec<u8>) {
    let len = cups.len();
    let max = *cups.iter().max().unwrap();
    let label = cups[cc];
    let picked_up = (
        std::mem::replace(&mut cups[(cc + 1) % len], 0),
        std::mem::replace(&mut cups[(cc + 2) % len], 0),
        std::mem::replace(&mut cups[(cc + 3) % len], 0),
    );
    println!("pick up: {:?}", picked_up);
    // Adjust for 0-index and underflow
    let mut dest = if label == 1 { max } else { label - 1 };
    println!("destination: {}", dest);
    let mut dest_index: Option<usize> = None;
    while dest_index.is_none() {
        if dest == 0 {
            dest = max;
        }

        dest_index = cups.iter().position(|v| *v == dest);
        dest -= 1;
    }
    let dest_index = dest_index.unwrap();
    let mut i = 0;
    while (cc + i) % len != dest_index {
        i += 1;
    }
    let mut counter = 0;
    loop {
        let index = (cc + 1 + counter) % len;
        cups[index] = cups[(index + 3) % len];
        if (index + 3) % len == dest_index {
            cups[(index + 1) % len] = picked_up.0;
            cups[(index + 2) % len] = picked_up.1;
            cups[(index + 3) % len] = picked_up.2;
            break;
        }
        counter += 1;
    }
}

fn play_game(cups: &mut Vec<u8>) {
    for cc in 0..100 {
        println!("-- move {} --", cc + 1);
        println!("cups: {:?}", cups);
        move_cups(cc % cups.len(), cups)
    }
}

fn get_result(cups: &Vec<u8>) -> String {
    let len = cups.len();
    let start = cups.iter().position(|v| *v == 1).unwrap() + 1;
    let mut result = String::new();
    for counter in 0..len - 1 {
        let index = (start + counter) % len;
        result.push_str(cups[index].to_string().as_str());
    }
    result
}

pub fn p1() -> String {
    let mut cups = INPUT.iter().copied().collect();
    play_game(&mut cups);
    get_result(&cups)
}

pub fn p2() -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: [u8; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn p1_example() {
        let mut cups = EXAMPLE.iter().copied().collect();
        play_game(&mut cups);
        assert_eq!("67384529", get_result(&cups));
    }

    #[test]
    fn p1_correct_answer() {
        let mut cups = INPUT.iter().copied().collect();
        play_game(&mut cups);
        assert_eq!("43769582", get_result(&cups));
    }

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
