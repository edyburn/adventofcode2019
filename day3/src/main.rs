use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_path(path: &str) -> HashMap<(i32, i32), u32> {
    let mut coord_map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
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
            steps += 1;
            coord_map.entry((x, y)).or_insert(steps);
        }
    }
    coord_map
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.lines().collect();
    let wire1_path = parse_path(lines[0]);
    let wire2_path = parse_path(lines[1]);
    let wire1_keys: HashSet<_> = wire1_path.keys().collect();
    let wire2_keys: HashSet<_> = wire2_path.keys().collect();
    let result = wire1_keys
        // Intersect to find where wires cross
        .intersection(&wire2_keys)
        // Compute combined steps
        .map(|coords| wire1_path.get(coords).unwrap() + wire2_path.get(coords).unwrap())
        // Find minimum
        .min();
    println!("result: {:?}", result);
    Ok(())
}
