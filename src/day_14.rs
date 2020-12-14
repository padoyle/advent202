use std::collections::HashMap;

static INPUT: &'static str = include_str!("assets/day_14_input.txt");

#[derive(Debug)]
struct BitMask {
    mask: u64,
    inv_mask: u64,
}

impl From<&str> for BitMask {
    fn from(mask_str: &str) -> BitMask {
        let mut mask: u64 = 0;
        let mut inv_mask: u64 = 0;
        for (i, value) in mask_str.chars().rev().enumerate() {
            match value {
                '1' => mask += (2 as u64).pow(i as u32),
                '0' => inv_mask += (2 as u64).pow(i as u32),
                'X' => {}
                _ => panic!("Invalid element"),
            }
        }

        BitMask { mask, inv_mask }
    }
}

impl BitMask {
    fn apply(&self, value: u64) -> u64 {
        let mut result = value;
        result |= self.mask;
        result = !(!result | self.inv_mask);

        result
    }
}

struct InitProgram {
    mask: BitMask,
    memory: HashMap<u64, u64>,
}

impl InitProgram {
    fn new() -> Self {
        Self {
            mask: BitMask {
                mask: 0,
                inv_mask: 0,
            },
            memory: HashMap::new(),
        }
    }

    fn run_instructions(&mut self, instructions: &str) {
        for instruction in instructions.lines() {
            let mut instruction_tokens = instruction.split_whitespace();
            let left = instruction_tokens.next().unwrap();
            let right = instruction_tokens.last().unwrap();
            match (left, right) {
                ("mask", value) => self.mask = BitMask::from(value),
                (mem, value_str) => {
                    let (start, end) = (mem.find('[').unwrap(), mem.find(']').unwrap());
                    let address: u64 = mem[start + 1..end].parse().unwrap();
                    let value: u64 = value_str.parse().unwrap();
                    self.memory.insert(address, self.mask.apply(value));
                }
            }
        }
    }

    fn sum_of_values(&self) -> u64 {
        self.memory.values().sum()
    }
}

fn run_init_program(input: &str) -> u64 {
    let mut program = InitProgram::new();
    program.run_instructions(input);
    program.sum_of_values()
}

pub fn p1() -> u64 {
    run_init_program(INPUT)
}

pub fn p2() -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    #[test]
    fn parsing_and_masking() {
        let bit_mask = BitMask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(64, bit_mask.mask);
        assert_eq!(2, bit_mask.inv_mask);

        let value = 11;
        let masked = bit_mask.apply(value);

        assert_eq!(73, masked);
    }

    #[test]
    fn p1_example() {
        assert_eq!(165, run_init_program(EXAMPLE));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(5875750429995, run_init_program(INPUT));
    }

    // #[test]
    // fn p2_example() {
    // }

    // #[test]
    // fn p2_correct_answer() {
    // }
}
