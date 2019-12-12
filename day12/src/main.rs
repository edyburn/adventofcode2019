use std::cmp::Ordering;

struct Moon {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position = add_triples(self.position, self.velocity);
    }

    fn get_total_energy(&self) -> isize {
        // A moon's potential energy is the sum of the absolute values of its
        // x, y, and z position coordinates.
        let (x, y, z) = self.position;
        let potential = x.abs() + y.abs() + z.abs();
        // A moon's kinetic energy is the sum of the absolute values of its
        // velocity coordinates.
        let (vx, vy, vz) = self.velocity;
        let kinetic = vx.abs() + vy.abs() + vz.abs();
        // The total energy for a single moon is its potential energy
        // multiplied by its kinetic energy.
        potential * kinetic
    }
}

fn add_triples(
    (a1, a2, a3): (isize, isize, isize),
    (b1, b2, b3): (isize, isize, isize),
) -> (isize, isize, isize) {
    (a1 + b1, a2 + b2, a3 + b3)
}

fn axis_velocity_delta(p1: isize, p2: isize) -> (isize, isize) {
    match p1.cmp(&p2) {
        Ordering::Equal => (0, 0),
        Ordering::Less => (1, -1),
        Ordering::Greater => (-1, 1),
    }
}

fn apply_gravity(moons: &mut [Moon], idx1: usize, idx2: usize) {
    let moon1 = &moons[idx1];
    let moon2 = &moons[idx2];
    let (x1, y1, z1) = moon1.position;
    let (x2, y2, z2) = moon2.position;
    let (dvx1, dvx2) = axis_velocity_delta(x1, x2);
    let (dvy1, dvy2) = axis_velocity_delta(y1, y2);
    let (dvz1, dvz2) = axis_velocity_delta(z1, z2);
    let moon1 = moons.get_mut(idx1).unwrap();
    moon1.velocity = add_triples(moon1.velocity, (dvx1, dvy1, dvz1));
    let moon2 = moons.get_mut(idx2).unwrap();
    moon2.velocity = add_triples(moon2.velocity, (dvx2, dvy2, dvz2));
}

fn main() {
    /* Input:
     *  <x=3, y=2, z=-6>
     *  <x=-13, y=18, z=10>
     *  <x=-8, y=-1, z=13>
     *  <x=5, y=10, z=4>
     */
    // Since there are only 4 moons, I'm not going to bother with parsing...
    let mut moons = vec![
        Moon {
            position: (3, 2, -6),
            velocity: (0, 0, 0),
        },
        Moon {
            position: (-13, 18, 10),
            velocity: (0, 0, 0),
        },
        Moon {
            position: (-8, -1, 13),
            velocity: (0, 0, 0),
        },
        Moon {
            position: (5, 10, 4),
            velocity: (0, 0, 0),
        },
    ];

    let mut pairs = Vec::new();
    for idx1 in 0..moons.len() {
        for idx2 in 0..moons.len() {
            if idx1 < idx2 {
                pairs.push((idx1, idx2));
            }
        }
    }

    for _ in 0..1000 {
        for (idx1, idx2) in pairs.iter() {
            apply_gravity(&mut moons, *idx1, *idx2);
        }
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    let total: isize = moons.iter().map(|m| m.get_total_energy()).sum();
    println!("result: {}", total);
}
