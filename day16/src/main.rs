fn fft(signal: Vec<isize>) -> Vec<isize> {
    signal
        .iter()
        .scan(0, |sum, i| {
            *sum += i;
            Some(sum.abs() % 10)
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let mut buffer: Vec<isize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .cycle()
        .take(10_000 * input.len())
        .collect();
    let message_offset: usize = (&input[0..7]).parse().unwrap();
    assert!(message_offset > (buffer.len() / 2));
    let suffix = buffer.len() - message_offset;
    buffer = buffer.into_iter().rev().take(suffix).collect();
    for _phase in 1..=100 {
        buffer = fft(buffer);
    }
    println!(
        "result: {:?}",
        buffer.into_iter().rev().take(8).collect::<Vec<isize>>()
    );
}
