use std::collections::HashSet;

static INPUT: &'static str = include_str!("assets/day_08_input.txt");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc { value: i32 },
    Jmp { offset: i32 },
    Nop { value: i32 },
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let tokens: Vec<&str> = s.split_whitespace().collect();
        match tokens.as_slice() {
            ["acc", value_str] => Instruction::Acc {
                value: value_str.parse().unwrap(),
            },
            ["jmp", offset_str] => Instruction::Jmp {
                offset: offset_str.parse().unwrap(),
            },
            ["nop", value_str] => Instruction::Nop {
                value: value_str.parse().unwrap(),
            },
            _ => panic!("invalid op"),
        }
    }
}

#[derive(Debug)]
struct Program {
    acc_value: i32,
    did_terminate: bool,
    instructions: Vec<Instruction>,
}

impl From<&str> for Program {
    fn from(s: &str) -> Self {
        Self {
            acc_value: 0,
            did_terminate: false,
            instructions: s.lines().map(Instruction::from).collect(),
        }
    }
}

impl Program {
    fn reset(&mut self) {
        self.acc_value = 0;
        self.did_terminate = false;
    }

    fn run_until_loop(&mut self) {
        self.reset();
        let mut instructions_hit: HashSet<usize> = HashSet::new();
        let mut ptr: usize = 0;
        while ptr < self.instructions.len() {
            if instructions_hit.contains(&ptr) {
                return;
            }
            instructions_hit.insert(ptr);
            match self.instructions[ptr] {
                Instruction::Acc { value } => {
                    self.acc_value += value;
                }
                Instruction::Jmp { offset } => {
                    ptr = (ptr as i32 + offset) as usize;
                    continue;
                }
                Instruction::Nop { .. } => {}
            }
            ptr += 1;
        }

        self.did_terminate = true;
    }

    fn read_acc(&self) -> i32 {
        self.acc_value
    }

    fn fix_corruption_and_run(&mut self) {
        let mut modify_ptr: usize = 0;
        let instruction_count = self.instructions.len();
        while modify_ptr < instruction_count {
            let op: Instruction = self.instructions[modify_ptr];
            match op {
                Instruction::Nop { value } => {
                    self.instructions[modify_ptr] = Instruction::Jmp { offset: value };
                    self.run_until_loop();
                    if self.did_terminate {
                        println!("Modified instruction at {}", modify_ptr);
                        return;
                    }
                    // restore
                    self.instructions[modify_ptr] = Instruction::Nop { value: value };
                }
                Instruction::Jmp { offset } => {
                    self.instructions[modify_ptr] = Instruction::Nop { value: offset };
                    self.run_until_loop();
                    if self.did_terminate {
                        println!("Modified instruction at {}", modify_ptr);
                        return;
                    }
                    // restore
                    self.instructions[modify_ptr] = Instruction::Jmp { offset: offset };
                }
                Instruction::Acc { .. } => {}
            }
            modify_ptr += 1;
        }
    }
}

pub fn p1() -> i32 {
    let mut program = Program::from(INPUT);
    program.run_until_loop();

    program.read_acc()
}

pub fn p2() -> i32 {
    let mut program = Program::from(INPUT);
    program.fix_corruption_and_run();

    program.read_acc()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn p1_example() {
        let mut program = Program::from(EXAMPLE);
        program.run_until_loop();

        assert_eq!(5, program.read_acc())
    }

    #[test]
    fn p1_correct_answer() {
        let mut program = Program::from(INPUT);
        program.run_until_loop();

        assert_eq!(1675, program.read_acc())
    }

    #[test]
    fn p2_example() {
        let mut program = Program::from(EXAMPLE);
        program.fix_corruption_and_run();

        assert_eq!(8, program.read_acc())
    }

    #[test]
    fn p2_correct_answer() {
        let mut program = Program::from(INPUT);
        program.fix_corruption_and_run();

        assert_eq!(1532, program.read_acc())
    }
}
