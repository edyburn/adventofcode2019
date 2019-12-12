use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct MoonAxis {
    p: isize,
    v: isize,
}

impl MoonAxis {
    fn apply_velocity(&mut self) {
        self.p += self.v;
    }

    fn apply_gravity(axis: &mut Vec<Self>, idx1: usize, idx2: usize) {
        let m1 = &axis[idx1];
        let m2 = &axis[idx2];
        match m1.p.cmp(&m2.p) {
            Ordering::Equal => (),
            Ordering::Less => {
                let m1 = axis.get_mut(idx1).unwrap();
                m1.v += 1;
                let m2 = axis.get_mut(idx2).unwrap();
                m2.v -= 1;
            }
            Ordering::Greater => {
                let m1 = axis.get_mut(idx1).unwrap();
                m1.v -= 1;
                let m2 = axis.get_mut(idx2).unwrap();
                m2.v += 1;
            }
        }
    }
}

fn process_axis(axis: &mut Vec<MoonAxis>, pairs: &[(usize, usize)]) -> u64 {
    let mut steps: u64 = 0;
    let mut seen = HashMap::new();
    seen.insert(axis.clone(), 0);
    loop {
        steps += 1;
        for (idx1, idx2) in pairs.iter() {
            MoonAxis::apply_gravity(axis, *idx1, *idx2);
        }
        for moon in axis.iter_mut() {
            moon.apply_velocity();
        }
        if let Some(last) = seen.get(axis) {
            return steps - last;
        }
        seen.insert(axis.clone(), steps);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn main() {
    /* Input:
     *  <x=3, y=2, z=-6>
     *  <x=-13, y=18, z=10>
     *  <x=-8, y=-1, z=13>
     *  <x=5, y=10, z=4>
     */
    // Since there are only 4 moons, I'm not going to bother with parsing...
    let mut x_axis = vec![
        MoonAxis { p: 3, v: 0 },
        MoonAxis { p: -13, v: 0 },
        MoonAxis { p: -8, v: 0 },
        MoonAxis { p: 5, v: 0 },
    ];
    let mut y_axis = vec![
        MoonAxis { p: 2, v: 0 },
        MoonAxis { p: 18, v: 0 },
        MoonAxis { p: -1, v: 0 },
        MoonAxis { p: 10, v: 0 },
    ];
    let mut z_axis = vec![
        MoonAxis { p: -6, v: 0 },
        MoonAxis { p: 10, v: 0 },
        MoonAxis { p: 13, v: 0 },
        MoonAxis { p: 4, v: 0 },
    ];
    // Example
    // <x=-1, y=0, z=2>
    // <x=2, y=-10, z=-7>
    // <x=4, y=-8, z=8>
    // <x=3, y=5, z=-1>
    // let mut x_axis = vec![
    //     MoonAxis { p: -1, v: 0 },
    //     MoonAxis { p: 2, v: 0 },
    //     MoonAxis { p: 4, v: 0 },
    //     MoonAxis { p: 3, v: 0 },
    // ];
    // let mut y_axis = vec![
    //     MoonAxis { p: 0, v: 0 },
    //     MoonAxis { p: -10, v: 0 },
    //     MoonAxis { p: -8, v: 0 },
    //     MoonAxis { p: 5, v: 0 },
    // ];
    // let mut z_axis = vec![
    //     MoonAxis { p: 2, v: 0 },
    //     MoonAxis { p: -7, v: 0 },
    //     MoonAxis { p: 8, v: 0 },
    //     MoonAxis { p: -1, v: 0 },
    // ];
    let mut pairs = Vec::new();
    for idx1 in 0..x_axis.len() {
        for idx2 in 0..x_axis.len() {
            if idx1 < idx2 {
                pairs.push((idx1, idx2));
            }
        }
    }

    let x_period = process_axis(&mut x_axis, &pairs);
    let y_period = process_axis(&mut y_axis, &pairs);
    let z_period = process_axis(&mut z_axis, &pairs);

    println!("result: {}", lcm(lcm(x_period, y_period), z_period));
}
