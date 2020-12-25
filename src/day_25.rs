static INPUT: (usize, usize) = (2069194, 16426071);

fn apply_transform(value: usize, subject: usize) -> usize {
    let value = value * subject;
    value % 20201227
}

fn find_loop_size(subject: usize, target: usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    while value != target {
        value = apply_transform(value, subject);
        loop_size += 1;
    }
    loop_size
}

fn get_encryption_key(key_1: usize, key_2: usize) -> usize {
    let loop_2 = find_loop_size(7, key_2);
    let mut value = 1;
    for _ in 0..loop_2 {
        value = apply_transform(value, key_1);
    }
    value
}

pub fn p1() -> usize {
    get_encryption_key(INPUT.0, INPUT.1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example() {
        assert_eq!(8, find_loop_size(7, 5764801));
        assert_eq!(11, find_loop_size(7, 17807724));

        assert_eq!(14897079, get_encryption_key(5764801, 17807724));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(11576351, get_encryption_key(INPUT.0, INPUT.1));
    }
}
