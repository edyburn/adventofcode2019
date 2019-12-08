fn main() {
    let input = include_str!("../input.txt").trim();
    // The image you received is 25 pixels wide and 6 pixels tall.
    let width = 25;
    let height = 6;
    let layers = input.chars().fold(vec![Vec::<u32>::new()], |mut l, c| {
        if l.last().unwrap().len() == width * height {
            l.push(Vec::new());
        }
        l.last_mut().unwrap().push(c.to_digit(10).unwrap());
        l
    });
    // 0 is black, 1 is white, and 2 is transparent
    let flattened: String = (0..layers[0].len())
        .map(|i| {
            for layer in layers.iter() {
                match layer[i] {
                    0 => return '▮',
                    1 => return ' ',
                    2 => continue,
                    _ => panic!("unknown color"),
                }
            }
            panic!("no color at pixel");
        })
        .collect();
    for line in flattened.chars().collect::<Vec<char>>().chunks(width) {
        println!("{}", line.iter().collect::<String>());
    }
}
