use std::collections::{HashMap, HashSet};

fn get_path<'a>(orbits: &HashMap<&str, &'a str>, start: &'a str) -> HashSet<&'a str> {
    let mut path = HashSet::new();
    let mut cursor = start;
    while cursor != "COM" {
        cursor = orbits.get(cursor).unwrap();
        path.insert(cursor);
    }
    path
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let orbits: HashMap<&str, &str> = input
        .lines()
        .map(|line| {
            let pieces: Vec<_> = line.split(')').collect();
            (pieces[1], pieces[0])
        })
        .collect();

    let you_path = get_path(&orbits, "YOU");
    let san_path = get_path(&orbits, "SAN");

    let result = you_path.symmetric_difference(&san_path).count();
    println!("result: {}", result);
}
