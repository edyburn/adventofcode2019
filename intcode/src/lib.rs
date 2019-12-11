pub struct Program {
    memory: Vec<isize>,
    pointer: usize,
    relative_base: isize,
    pub input: Vec<isize>,
    halted: bool,
}

pub enum InstructionResult {
    None,
    InputBlocked,
    Output(isize),
    Halt,
}

impl Program {
    pub fn new(memory: Vec<isize>, input: Vec<isize>) -> Program {
        Program {
            memory,
            pointer: 0,
            relative_base: 0,
            input,
            halted: false,
        }
    }

    pub fn from_str(s: &str, input: Vec<isize>) -> Program {
        let memory = s
            .trim_end()
            .split(',')
            .map(|i| i.parse::<isize>().unwrap())
            .collect();
        Program::new(memory, input)
    }

    fn _mem_get(&self, index: usize) -> isize {
        if index >= self.memory.len() {
            0
        } else {
            self.memory[index]
        }
    }

    fn lookup(&self, offset: usize, params: usize) -> isize {
        let index = self.pointer + offset;
        let mode = (params / (10usize.pow((offset - 1) as u32))) % 10;
        let value = self._mem_get(index);
        match mode {
            // "position" mode
            0 => self._mem_get(value as usize),
            // "immediate" mode
            1 => value,
            // "relative" mode
            2 => self._mem_get((value + self.relative_base) as usize),
            _ => panic!("unknown mode"),
        }
    }

    fn set(&mut self, offset: usize, params: usize, value: isize) {
        let index = self.pointer + offset;
        let mode = (params / (10usize.pow((offset - 1) as u32))) % 10;
        let pos = match mode {
            0 => self._mem_get(index) as usize,
            2 => (self._mem_get(index) + self.relative_base) as usize,
            _ => panic!("unknown mode"),
        };
        if pos >= self.memory.len() {
            self.memory.resize(pos + 1, 0);
        }
        self.memory[pos] = value;
    }

    pub fn execute_instruction(&mut self) -> InstructionResult {
        if self.halted {
            return InstructionResult::Halt;
        }
        let instruction = self._mem_get(self.pointer);
        let opcode = instruction % 100;
        let params = instruction as usize / 100;
        match opcode {
            // add, step 4
            1 => {
                let a = self.lookup(1, params);
                let b = self.lookup(2, params);
                let sum = a + b;
                self.set(3, params, sum);
                self.pointer += 4;
                InstructionResult::None
            }
            // multiply, step 4
            2 => {
                let a = self.lookup(1, params);
                let b = self.lookup(2, params);
                let product = a * b;
                self.set(3, params, product);
                self.pointer += 4;
                InstructionResult::None
            }
            // input, step 2
            3 => {
                if self.input.is_empty() {
                    return InstructionResult::InputBlocked;
                }
                let input_value = self.input.remove(0);
                self.set(1, params, input_value);
                self.pointer += 2;
                InstructionResult::None
            }
            // output, step 2
            4 => {
                let output_value = self.lookup(1, params);
                self.pointer += 2;
                InstructionResult::Output(output_value)
            }
            // jump-if-true
            5 => {
                let value = self.lookup(1, params);
                if value != 0 {
                    self.pointer = self.lookup(2, params) as usize;
                } else {
                    self.pointer += 3;
                }
                InstructionResult::None
            }
            // jump-if-false
            6 => {
                let value = self.lookup(1, params);
                if value == 0 {
                    self.pointer = self.lookup(2, params) as usize;
                } else {
                    self.pointer += 3;
                }
                InstructionResult::None
            }
            // less than
            7 => {
                let a = self.lookup(1, params);
                let b = self.lookup(2, params);
                self.set(3, params, if a < b { 1 } else { 0 });
                self.pointer += 4;
                InstructionResult::None
            }
            // equal
            8 => {
                let a = self.lookup(1, params);
                let b = self.lookup(2, params);
                self.set(3, params, if a == b { 1 } else { 0 });
                self.pointer += 4;
                InstructionResult::None
            }
            // adjusts the relative base
            9 => {
                self.relative_base += self.lookup(1, params);
                self.pointer += 2;
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
