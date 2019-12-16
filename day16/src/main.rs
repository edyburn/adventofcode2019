use std::iter;

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn fft(signal: &[isize]) -> Vec<isize> {
    signal
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let pos = idx + 1;
            let sum: isize = signal
                .iter()
                .zip(
                    BASE_PATTERN
                        .iter()
                        .flat_map(|x| iter::repeat(x).take(pos))
                        .cycle()
                        .skip(1),
                )
                .map(|(i, p)| i * p)
                .sum();
            (sum % 10).abs()
        })
        .collect()
}

fn main() {
    // After 100 phases of FFT, what are the first eight digits in the final output list?
    let input = include_str!("../input.txt").trim();
    let mut buffer: Vec<isize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect();
    for _phase in 1..=100 {
        buffer = fft(&buffer);
    }
    println!("result: {:?}", &buffer[0..8]);
}
