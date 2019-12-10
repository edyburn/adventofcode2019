use std::collections::{HashMap, HashSet};

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
        let mut degrees = y_diff.atan2(x_diff).to_degrees();
        // rotate 90 degrees since laser starts up
        degrees += 90.0;
        // make all angles positive
        if degrees < 0.0 {
            degrees += 360.0;
        }
        degrees.to_string()
    }
    fn distance(&self, other: &Asteroid) -> f64 {
        let x_diff = other.x as f64 - self.x as f64;
        let y_diff = other.y as f64 - self.y as f64;
        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
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
        let angles: HashSet<_> = asteroids
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
        asteroids.get_mut(i).unwrap().visible = angles.len();
    }
    // Find the best location for a new monitoring station.
    let (best_idx, _) = asteroids
        .iter()
        .enumerate()
        .max_by_key(|(_, a)| a.visible)
        .unwrap();
    let best = asteroids.remove(best_idx);

    let mut angles = Vec::new();
    let mut map: HashMap<String, Vec<&Asteroid>> =
        asteroids.iter().fold(HashMap::new(), |mut m, a| {
            let angle = best.angle(a);
            angles.push(angle.clone());
            let entry = m.entry(angle).or_default();
            entry.push(a);
            entry.sort_by(|a, b| {
                let a_dist = best.distance(a);
                let b_dist = best.distance(b);
                a_dist.partial_cmp(&b_dist).unwrap()
            });
            // reverse sort so can pop closest asteroid
            entry.reverse();
            m
        });
    angles.sort_by(|a, b| {
        let a = a.parse::<f64>().unwrap();
        let b = b.parse::<f64>().unwrap();
        a.partial_cmp(&b).unwrap()
    });
    angles.dedup();

    let mut vaporized = 0;
    for angle in angles.iter().cycle() {
        let entry = map.get_mut(angle).unwrap();
        if entry.is_empty() {
            continue;
        }
        let asteroid = entry.pop().unwrap();
        vaporized += 1;
        println!("destroying #{}: {},{}", vaporized, asteroid.x, asteroid.y);
        if vaporized == 200 {
            // which will be the 200th asteroid to be vaporized?
            // multiply its X coordinate by 100 and then add its Y coordinate
            let result = (asteroid.x * 100) + asteroid.y;
            println!("done: {}", result);
            break;
        }
    }
}
