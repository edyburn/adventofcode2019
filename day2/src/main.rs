use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let mut data: Vec<usize> = input
        .trim_end()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .collect();
    let mut cursor: usize = 0;

    // before running the program, replace position 1 with the value 12 and
    // replace position 2 with the value 2.
    data[1] = 12;
    data[2] = 2;

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

    // What value is left at position 0 after the program halts?
    println!("result: {}", data[0]);

    Ok(())
}
