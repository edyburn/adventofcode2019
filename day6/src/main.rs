use std::collections::HashMap;

fn count_orbits(orbits: &HashMap<&str, Vec<&str>>, root: &str, depth: u32) -> u32 {
    let sat_count: u32 = match orbits.get(root) {
        Some(satellites) => satellites
            .iter()
            .map(|satellite| count_orbits(orbits, satellite, depth + 1))
            .sum(),
        None => 0,
    };
    depth + sat_count
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let pieces: Vec<_> = line.split(')').collect();
        let satellites = orbits.entry(pieces[0]).or_default();
        satellites.push(pieces[1]);
    }

    let result = count_orbits(&orbits, "COM", 0);
    println!("result: {}", result);
}
