use intcode::{InstructionResult, Program};
use std::collections::HashMap;

const SIZE: isize = 100;
fn run() -> (isize, isize) {
    let mut last_per_col = HashMap::new();
    let mut x = SIZE;
    let mut min_y = 0;
    loop {
        let mut y = min_y;
        let mut last = -1;
        loop {
            let mut program = Program::from_str(include_str!("../input.txt"), vec![x, y]);
            let mut output = 0;
            loop {
                match program.execute_instruction() {
                    InstructionResult::None => continue,
                    InstructionResult::Halt => break,
                    // Input: x, y position where to deploy drone (non-negative)
                    InstructionResult::InputBlocked => unimplemented!(),
                    // Output: stationary (0) or being pulled by something (1)
                    InstructionResult::Output(value) => {
                        output = value;
                    }
                }
            }
            if output == 1 {
                if last == -1 {
                    min_y = y;
                }
                last = y;
                let other_x = x - (SIZE - 1);
                if let Some(other_last) = last_per_col.get(&other_x) {
                    if other_last - y == (SIZE - 1) {
                        return (other_x, y);
                    }
                }
            } else if last != -1 {
                last_per_col.insert(x, last);
                break;
            }
            y += 1;
        }
        x += 1;
    }
}
fn main() {
    let (x, y) = run();

    /*
    Find the 100x100 square closest to the emitter that fits entirely within
    the tractor beam; within that square, find the point closest to the
    emitter. What value do you get if you take that point's X coordinate,
    multiply it by 10000, then add the point's Y coordinate?
    */
    println!("result: {}", x * 10000 + y);
}
