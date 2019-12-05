use std::fs;

#[allow(clippy::unreadable_literal)]
const TARGET: usize = 19690720;

fn run_program(mut data: Vec<usize>, noun: usize, verb: usize) -> bool {
    let mut cursor: usize = 0;

    data[1] = noun;
    data[2] = verb;

    loop {
        let opcode = data[cursor];
        match opcode {
            // add, step 4
            1 => {
                let sum = data[data[cursor + 1]] + data[data[cursor + 2]];
                let result_pos = data[cursor + 3];
                data[result_pos] = sum;
                // println!("add: {}", sum);
                cursor += 4;
            }
            // multiply, step 4
            2 => {
                let product = data[data[cursor + 1]] * data[data[cursor + 2]];
                let result_pos = data[cursor + 3];
                data[result_pos] = product;
                // println!("mult: {}", product);
                cursor += 4;
            }
            // halt
            99 => break,
            _ => panic!(format!("unknown opcode: {}", opcode)),
        }
    }
    data[0] == TARGET
}

fn try_values(data: Vec<usize>) -> (usize, usize) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(data.clone(), noun, verb) {
                return (noun, verb);
            }
        }
    }
    panic!("No solution!")
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<usize> = input
        .trim_end()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    let (noun, verb) = try_values(data);

    println!("result: {}", 100 * noun + verb);

    Ok(())
}
