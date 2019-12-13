use intcode::{InstructionResult, Program};
use std::collections::HashMap;

fn main() {
    let mut program = Program::from_str(include_str!("../input.txt"), Vec::new());

    let mut screen = HashMap::new();
    let mut output_buffer = Vec::new();
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => panic!("no input connected"),
            InstructionResult::Output(value) => {
                /*
                every three output instructions specify:
                - x position (distance from the left)
                - y position (distance from the top)
                - tile id
                */
                output_buffer.push(value);
                if output_buffer.len() == 3 {
                    let tile_id = output_buffer.pop().unwrap();
                    let y = output_buffer.pop().unwrap();
                    let x = output_buffer.pop().unwrap();
                    screen.insert((x, y), tile_id);
                }
            }
        }
    }
    /*
    The tile id is interpreted as follows:
    0 is an empty tile. No game object appears in this tile.
    1 is a wall tile. Walls are indestructible barriers.
    2 is a block tile. Blocks can be broken by the ball.
    3 is a horizontal paddle tile. The paddle is indestructible.
    4 is a ball tile. The ball moves diagonally and bounces off objects.
    */
    // How many block tiles are on the screen when the game exits?
    let result = screen.values().filter(|&t| *t == 2).count();
    println!("result: {}", result);
}
