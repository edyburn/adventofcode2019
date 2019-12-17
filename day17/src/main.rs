use intcode::{InstructionResult, Program};

fn main() {
    let mut program = Program::from_str(include_str!("../input.txt"), Vec::new());
    let mut buffer = String::new();
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => unimplemented!(),
            InstructionResult::Output(value) => {
                buffer.push(value as u8 as char);
            }
        }
    }
    let lines: Vec<Vec<char>> = buffer.split('\n').map(|l| l.chars().collect()).collect();
    let mut intersections = Vec::<(usize, usize)>::new();
    for (y, line) in lines.iter().enumerate() {
        if y == 0 || y == lines.len() - 1 {
            continue;
        }
        for (x, &c) in line.iter().enumerate() {
            if c != '#' || x == 0 || x == line.len() - 1 {
                continue;
            }
            if line[x - 1] == '#'
                && line[x + 1] == '#'
                && lines[y - 1][x] == '#'
                && lines[y + 1][x] == '#'
            {
                intersections.push((x, y));
            }
        }
    }

    // its alignment parameter is the distance between its left edge and the
    // left edge of the view multiplied by the distance between its top edge
    // and the top edge of the view

    // What is the sum of the alignment parameters for the scaffold intersections?
    let result: usize = intersections.iter().map(|(x, y)| x * y).sum();
    println!("result: {}", result);
}
