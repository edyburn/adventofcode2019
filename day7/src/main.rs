use permute::permute;

struct Program {
    memory: Vec<isize>,
    pointer: usize,
    input: Vec<isize>,
    halted: bool,
}

enum InstructionResult {
    None,
    InputBlocked,
    Output(isize),
    Halt,
}

impl Program {
    fn new(memory: Vec<isize>, input: Vec<isize>) -> Program {
        Program {
            memory,
            pointer: 0,
            input,
            halted: false,
        }
    }

    fn lookup(&self, index: usize, mode: usize) -> isize {
        let value = self.memory[index];
        if mode == 0 {
            self.memory[value as usize]
        } else {
            value
        }
    }

    fn execute_instruction(&mut self) -> InstructionResult {
        if self.halted {
            return InstructionResult::Halt;
        }
        let instruction = self.memory[self.pointer];
        let opcode = instruction % 100;
        let params = instruction as usize / 100;
        match opcode {
            // add, step 4
            1 => {
                let a = self.lookup(self.pointer + 1, params & 1);
                let b = self.lookup(self.pointer + 2, (params / 10) & 1);
                let sum = a + b;
                let result_pos = self.memory[self.pointer + 3] as usize;
                self.memory[result_pos] = sum;
                self.pointer += 4;
                InstructionResult::None
            }
            // multiply, step 4
            2 => {
                let a = self.lookup(self.pointer + 1, params & 1);
                let b = self.lookup(self.pointer + 2, (params / 10) & 1);
                let product = a * b;
                let result_pos = self.memory[self.pointer + 3] as usize;
                self.memory[result_pos] = product;
                self.pointer += 4;
                InstructionResult::None
            }
            // input, step 2
            3 => {
                if self.input.is_empty() {
                    return InstructionResult::InputBlocked;
                }
                let input_value = self.input.remove(0);
                let result_pos = self.memory[self.pointer + 1] as usize;
                self.memory[result_pos] = input_value;
                self.pointer += 2;
                InstructionResult::None
            }
            // output, step 2
            4 => {
                let output_value = self.lookup(self.pointer + 1, params & 1);
                self.pointer += 2;
                InstructionResult::Output(output_value)
            }
            // jump-if-true
            5 => {
                let value = self.lookup(self.pointer + 1, params & 1);
                if value != 0 {
                    self.pointer = self.lookup(self.pointer + 2, (params / 10) & 1) as usize;
                } else {
                    self.pointer += 3;
                }
                InstructionResult::None
            }
            // jump-if-false
            6 => {
                let value = self.lookup(self.pointer + 1, params & 1);
                if value == 0 {
                    self.pointer = self.lookup(self.pointer + 2, (params / 10) & 1) as usize;
                } else {
                    self.pointer += 3;
                }
                InstructionResult::None
            }
            // less than
            7 => {
                let a = self.lookup(self.pointer + 1, params & 1);
                let b = self.lookup(self.pointer + 2, (params / 10) & 1);
                let result_pos = self.memory[self.pointer + 3] as usize;
                self.memory[result_pos] = if a < b { 1 } else { 0 };
                self.pointer += 4;
                InstructionResult::None
            }
            // equal
            8 => {
                let a = self.lookup(self.pointer + 1, params & 1);
                let b = self.lookup(self.pointer + 2, (params / 10) & 1);
                let result_pos = self.memory[self.pointer + 3] as usize;
                self.memory[result_pos] = if a == b { 1 } else { 0 };
                self.pointer += 4;
                InstructionResult::None
            }
            // halt
            99 => {
                self.halted = true;
                InstructionResult::Halt
            }
            _ => panic!(format!("unknown opcode: {}", opcode)),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let data: Vec<isize> = input
        .trim_end()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    let mut max = 0;
    for permutation in permute(vec![5, 6, 7, 8, 9]) {
        let mut programs: Vec<Program> = permutation
            .iter()
            .map(|phase| Program::new(data.clone(), vec![*phase as isize]))
            .collect();
        programs[0].input.push(0);
        let mut active = 0;
        let mut final_output = 0;
        loop {
            match programs[active].execute_instruction() {
                InstructionResult::None => continue,
                InstructionResult::Halt => {
                    // Break out when all halted
                    if programs.iter().all(|p| p.halted) {
                        break;
                    }
                    // Run the next program
                    if active == 4 {
                        active = 0;
                    } else {
                        active += 1;
                    }
                }
                InstructionResult::InputBlocked => {
                    // Run the next program
                    if active == 4 {
                        active = 0;
                    } else {
                        active += 1;
                    }
                }
                InstructionResult::Output(value) => {
                    if active == 4 {
                        final_output = value;
                        programs[0].input.push(value);
                    } else {
                        programs[active + 1].input.push(value);
                    }
                }
            }
        }
        if final_output > max {
            max = final_output;
        }
    }
    println!("result: {}", max);
}
