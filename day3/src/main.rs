use std::collections::HashSet;
use std::fs;

fn parse_path(path: &str) -> HashSet<(i32, i32)> {
    let mut coord_set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for segment in path.split(',') {
        let (direction, distance) = segment.split_at(1);
        let distance = distance.parse::<i32>().unwrap();
        for _ in 0..distance {
            match direction {
                "R" => x += 1,
                "L" => x -= 1,
                "U" => y += 1,
                "D" => y -= 1,
                _ => panic!("Invalid direction"),
            };
            coord_set.insert((x, y));
        }
    }
    coord_set
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let wire1_path = parse_path(lines[0]);
    let wire2_path = parse_path(lines[1]);
    let result = wire1_path
        // Intersect to find where wires cross
        .intersection(&wire2_path)
        // Compute Manhattan distance from center
        .map(|(x, y)| x.abs() + y.abs())
        // Find minimum
        .min();
    println!("result: {:?}", result);
    Ok(())
}
