use std::collections::HashMap;

static INPUT: &'static str = include_str!("assets/day_14_input.txt");

#[derive(Debug, Clone)]
struct BitMask {
    mask: u64,     // 1s
    inv_mask: u64, // 0s
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
    fn new() -> Self {
        Self {
            mask: 0,
            inv_mask: 0,
        }
    }

    fn set_value(&mut self, power: u32, inv: bool) {
        let to_add = 1 << power;
        if inv {
            self.inv_mask += to_add;
        } else {
            self.mask += to_add;
        }
    }

    fn apply(&self, value: u64) -> u64 {
        let mut result = value;
        result |= self.mask;
        result = !(!result | self.inv_mask);

        result
    }
}

#[derive(Debug)]
struct BitMaskV2 {
    masks: Vec<BitMask>,
}

impl From<&str> for BitMaskV2 {
    fn from(mask_str: &str) -> BitMaskV2 {
        let mut masks = Vec::new();
        masks.push(BitMask::new());
        for (i, mask_char) in mask_str.chars().rev().enumerate() {
            match mask_char {
                '0' => {}
                '1' => {
                    masks
                        .iter_mut()
                        .for_each(|mask| mask.set_value(i as u32, false));
                }
                'X' => {
                    let mut new_masks = masks.clone();
                    // map 0s mask entries onto existing masks
                    masks
                        .iter_mut()
                        .for_each(|mask| mask.set_value(i as u32, true));
                    // map 1s mask entries onto new masks
                    new_masks
                        .iter_mut()
                        .for_each(|mask| mask.set_value(i as u32, false));
                    masks.extend(new_masks);
                }
                _ => panic!("unexpected mask character"),
            }
        }

        BitMaskV2 { masks }
    }
}

impl BitMaskV2 {
    fn new() -> Self {
        Self { masks: Vec::new() }
    }

    fn apply(&self, value: u64) -> Vec<u64> {
        self.masks.iter().map(|mask| mask.apply(value)).collect()
    }
}

struct InitProgram {
    mask: BitMask,
    memory: HashMap<u64, u64>,
}

impl InitProgram {
    fn new() -> Self {
        Self {
            mask: BitMask::new(),
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

struct InitProgramV2 {
    mask: BitMaskV2,
    memory: HashMap<u64, u64>,
}

impl InitProgramV2 {
    fn new() -> Self {
        Self {
            mask: BitMaskV2::new(),
            memory: HashMap::new(),
        }
    }

    fn run_instructions(&mut self, instructions: &str) {
        for instruction in instructions.lines() {
            let mut instruction_tokens = instruction.split_whitespace();
            let left = instruction_tokens.next().unwrap();
            let right = instruction_tokens.last().unwrap();
            // panic!(format!("instruction {}", instruction));
            match (left, right) {
                ("mask", value) => self.mask = BitMaskV2::from(value),
                (mem, value_str) => {
                    let (start, end) = (mem.find('[').unwrap(), mem.find(']').unwrap());
                    let base_addr: u64 = mem[start + 1..end].parse().unwrap();
                    let value: u64 = value_str.parse().unwrap();
                    println!(
                        "base_addr {}, value: {}\n\tmodifying {} addresses...",
                        base_addr,
                        value,
                        self.mask.masks.len()
                    );
                    self.mask.apply(base_addr).iter().for_each(|addr| {
                        self.memory.insert(*addr, value);
                    });
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

fn run_init_program_v2(input: &str) -> u64 {
    let mut program = InitProgramV2::new();
    program.run_instructions(input);
    program.sum_of_values()
}

pub fn p1() -> u64 {
    run_init_program(INPUT)
}

pub fn p2() -> u64 {
    run_init_program_v2(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    static EXAMPLE2: &str = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;

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
    fn p2_parsing_and_masking() {
        let bit_mask = BitMaskV2::from("000000000000000000000000000000X1001X");

        assert_eq!(4, bit_mask.masks.len());
        assert_eq!(vec![26, 27, 58, 59], bit_mask.apply(42));

        let bit_mask = BitMaskV2::from("00000000000000000000000000000000X0XX");

        assert_eq!(8, bit_mask.masks.len());
        assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], bit_mask.apply(26));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(5875750429995, run_init_program(INPUT));
    }

    #[test]
    fn p2_example() {
        assert_eq!(208, run_init_program_v2(EXAMPLE2));
    }

    #[test]
    fn p2_correct_answer() {
        assert_eq!(5272149590143, run_init_program_v2(INPUT));
    }
}
