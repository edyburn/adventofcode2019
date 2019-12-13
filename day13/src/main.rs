use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use intcode::{InstructionResult, Program};
use std::collections::HashMap;

fn render(screen: &HashMap<(isize, isize), isize>) {
    let &max_x = screen.keys().map(|(x, _)| x).max().unwrap();
    let &min_x = screen.keys().map(|(x, _)| x).min().unwrap();
    let &max_y = screen.keys().map(|(_, y)| y).max().unwrap();
    let &min_y = screen.keys().map(|(_, y)| y).min().unwrap();
    let mut buffer: Vec<Vec<char>> = (min_y..=max_y)
        .map(|_| (min_x..=max_x).map(|_| ' ').collect())
        .collect();
    for ((x, y), tile_id) in screen.iter() {
        let c = match tile_id {
            0 => ' ',
            1 => '█',
            2 => '⊠',
            3 => '=',
            4 => '⊙',
            _ => panic!("unknown tile id"),
        };
        buffer[(y - min_x) as usize][(x - min_x) as usize] = c;
    }
    for line in buffer.iter() {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let mut program = Program::from_str(include_str!("../input.txt"), Vec::new());
    // pretend to insert quarters
    program.memory[0] = 2;

    let mut screen = HashMap::new();
    let mut output_buffer = Vec::new();
    let mut score = 0;
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => {
                /*
                If the joystick is in the neutral position, provide 0.
                If the joystick is tilted to the left, provide -1.
                If the joystick is tilted to the right, provide 1.
                */
                render(&screen);
                enable_raw_mode().unwrap();
                let input = loop {
                    match read().unwrap() {
                        Event::Key(event) => match event.code {
                            KeyCode::Char('a') => break -1,
                            KeyCode::Char('d') => break 1,
                            KeyCode::Char('s') => break 0,
                            _ => continue,
                        },
                        Event::Mouse(_) => continue,
                        Event::Resize(_, _) => continue,
                    }
                };
                disable_raw_mode().unwrap();
                program.input.push(input);
            }
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
                    if x == -1 && y == 0 {
                        score = tile_id;
                    } else {
                        screen.insert((x, y), tile_id);
                    }
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
    // What is your score after the last block is broken?
    println!("result: {}", score);
}
