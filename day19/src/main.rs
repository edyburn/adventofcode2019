use intcode::{InstructionResult, Program};
fn main() {
    let mut output = Vec::new();

    for x in 0..50 {
        for y in 0..50 {
            let mut program = Program::from_str(include_str!("../input.txt"), vec![x, y]);
            loop {
                match program.execute_instruction() {
                    InstructionResult::None => continue,
                    InstructionResult::Halt => break,
                    // Input: x, y position where to deploy drone (non-negative)
                    InstructionResult::InputBlocked => unimplemented!(),
                    // Output: stationary (0) or being pulled by something (1)
                    InstructionResult::Output(value) => {
                        output.push(value);
                    }
                }
            }
        }
    }

    // How many points are affected by the tractor beam in the 50x50 area
    // closest to the emitter?
    println!("{}", output.iter().filter(|&a| *a == 1).count());
}
