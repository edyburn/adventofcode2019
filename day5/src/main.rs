use std::fs;

fn memory_lookup(memory: &[isize], index: usize, mode: usize) -> isize {
    let value = memory[index];
    if mode == 0 {
        memory[value as usize]
    } else {
        value
    }
}

fn run_program(mut memory: Vec<isize>, mut input: Vec<isize>) {
    let mut cursor: usize = 0;

    loop {
        let instruction = memory[cursor];
        let opcode = instruction % 100;
        let params = instruction as usize / 100;
        // println!("instruction: {}", instruction);
        match opcode {
            // add, step 4
            1 => {
                let a = memory_lookup(&memory, cursor + 1, params & 1);
                let b = memory_lookup(&memory, cursor + 2, (params / 10) & 1);
                let sum = a + b;
                let result_pos = memory[cursor + 3] as usize;
                memory[result_pos] = sum;
                // println!("add: {}", sum);
                cursor += 4;
            }
            // multiply, step 4
            2 => {
                let a = memory_lookup(&memory, cursor + 1, params & 1);
                let b = memory_lookup(&memory, cursor + 2, (params / 10) & 1);
                let product = a * b;
                let result_pos = memory[cursor + 3] as usize;
                memory[result_pos] = product;
                // println!("mult: {}", product);
                cursor += 4;
            }
            // input, step 2
            3 => {
                let input_value = input.remove(0);
                let result_pos = memory[cursor + 1] as usize;
                memory[result_pos] = input_value;
                // println!("save: {}", value);
                cursor += 2;
            }
            // output, step 2
            4 => {
                let output_value = memory_lookup(&memory, cursor + 1, params & 1);
                println!("output: {}", output_value);
                cursor += 2;
            }
            // jump-if-true
            5 => {
                let value = memory_lookup(&memory, cursor + 1, params & 1);
                if value != 0 {
                    cursor = memory_lookup(&memory, cursor + 2, (params / 10) & 1) as usize;
                } else {
                    cursor += 3;
                }
            }
            // jump-if-false
            6 => {
                let value = memory_lookup(&memory, cursor + 1, params & 1);
                if value == 0 {
                    cursor = memory_lookup(&memory, cursor + 2, (params / 10) & 1) as usize;
                } else {
                    cursor += 3;
                }
            }
            // less than
            7 => {
                let a = memory_lookup(&memory, cursor + 1, params & 1);
                let b = memory_lookup(&memory, cursor + 2, (params / 10) & 1);
                let result_pos = memory[cursor + 3] as usize;
                memory[result_pos] = if a < b { 1 } else { 0 };
                cursor += 4;
            }
            // equal
            8 => {
                let a = memory_lookup(&memory, cursor + 1, params & 1);
                let b = memory_lookup(&memory, cursor + 2, (params / 10) & 1);
                let result_pos = memory[cursor + 3] as usize;
                memory[result_pos] = if a == b { 1 } else { 0 };
                cursor += 4;
            }
            // halt
            99 => break,
            _ => panic!(format!("unknown opcode: {}", opcode)),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let data: Vec<isize> = input
        .trim_end()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    run_program(data, vec![5]);
    Ok(())
}
