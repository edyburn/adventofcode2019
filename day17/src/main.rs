use intcode::{InstructionResult, Program};

fn ascii_to_mem(s: &str) -> Vec<isize> {
    s.chars().map(|c| c as isize).collect()
}

fn main() {
    let mut input = Vec::new();
    // first, main movement routine
    // - movement functions A, B, C
    input.extend(ascii_to_mem("A,B,A,C,A,B,A,C,B,C\n"));
    // then, each movement function
    // - L to turn left, R to turn right, or a number to move forward)
    input.extend(ascii_to_mem("R,4,L,12,L,8,R,4\n"));
    input.extend(ascii_to_mem("L,8,R,10,R,10,R,6\n"));
    input.extend(ascii_to_mem("R,4,R,10,L,12\n"));
    // finally, want to see a continuous video feed
    input.extend(ascii_to_mem("n\n"));
    let mut program = Program::from_str(include_str!("../input.txt"), input);
    program.memory[0] = 2;
    let mut buffer = String::new();
    let mut dust = 0;
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => unimplemented!(),
            InstructionResult::Output(value) => {
                if value > 127 {
                    dust = value;
                } else {
                    buffer.push(value as u8 as char);
                }
            }
        }
    }

    // println!("{}", buffer);
    // how much dust does the vacuum robot report it has collected?
    println!("dust: {}", dust);
}
