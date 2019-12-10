use std::collections::HashSet;

struct Asteroid {
    x: usize,
    y: usize,
    visible: usize,
}

impl Asteroid {
    fn new(x: usize, y: usize) -> Asteroid {
        Asteroid { x, y, visible: 0 }
    }

    fn angle(&self, other: &Asteroid) -> String {
        let x_diff = other.x as f64 - self.x as f64;
        let y_diff = other.y as f64 - self.y as f64;
        y_diff.atan2(x_diff).to_string()
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let mut asteroids: Vec<Asteroid> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some(Asteroid::new(x, y)),
                _ => panic!("unknown map character"),
            })
        })
        .collect();

    for i in 0..asteroids.len() {
        let asteroid = &asteroids[i];
        let slopes: HashSet<_> = asteroids
            .iter()
            .enumerate()
            .filter_map(|(idx, other)| {
                if idx == i {
                    None
                } else {
                    Some(asteroid.angle(other))
                }
            })
            .collect();
        asteroids.get_mut(i).unwrap().visible = slopes.len();
    }
    // Find the best location for a new monitoring station.
    let best = asteroids.iter().max_by_key(|a| a.visible).unwrap();
    // How many other asteroids can be detected from that location?
    println!("result: {}", best.visible);
}
