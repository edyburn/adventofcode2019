fn main() {
    let input = include_str!("../input.txt").trim();
    // The image you received is 25 pixels wide and 6 pixels tall.
    let layers = input.chars().fold(vec![Vec::<u32>::new()], |mut l, c| {
        if l.last().unwrap().len() == 25 * 6 {
            l.push(Vec::new());
        }
        l.last_mut().unwrap().push(c.to_digit(10).unwrap());
        l
    });
    // find the layer that contains the fewest 0 digits.
    let layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&d| *d == 0).count())
        .unwrap();
    // On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
    let ones = layer.iter().filter(|&d| *d == 1).count();
    let twos = layer.iter().filter(|&d| *d == 2).count();
    let result = ones * twos;
    println!("result: {}", result);
}
