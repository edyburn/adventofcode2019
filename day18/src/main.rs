use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    coords: (usize, usize),
    keys: Vec<char>,
}

fn find_entrance(maze: &[Vec<char>]) -> (usize, usize) {
    for (y, line) in maze.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '@' {
                return (x, y);
            }
        }
    }
    panic!("didn't find an entrance");
}

fn adjacent_coords(maze: &[Vec<char>], coords: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = coords;
    let mut adjacent = Vec::new();
    if x > 0 {
        adjacent.push((x - 1, y));
    }
    if x < maze[0].len() - 1 {
        adjacent.push((x + 1, y));
    }
    if y > 0 {
        adjacent.push((x, y - 1));
    }
    if y < maze.len() - 1 {
        adjacent.push((x, y + 1));
    }
    adjacent
}

fn search(maze: &[Vec<char>], initial_state: State) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((initial_state.clone(), 0));
    let mut discovered = HashSet::new();
    discovered.insert(initial_state.clone());
    let mut sector_keys = HashSet::new();
    // find all keys in the sector
    while let Some((s, _)) = queue.pop_front() {
        let (x, y) = s.coords;
        let c = maze[y][x];
        if c == '#' {
            // hit a wall
            continue;
        }
        let mut keys = s.keys.clone();
        if c.is_ascii_lowercase() && !s.keys.contains(&c) {
            keys.push(c);
            keys.sort();
            sector_keys.insert(c);
        };
        for (x, y) in adjacent_coords(maze, s.coords) {
            let new_state = State {
                coords: (x, y),
                keys: keys.clone(),
            };
            if discovered.contains(&new_state) {
                continue;
            }
            discovered.insert(new_state.clone());
            queue.push_back((new_state, 0));
        }
    }
    // find the shortest path
    queue.clear();
    queue.push_back((initial_state.clone(), 0));
    discovered.clear();
    discovered.insert(initial_state);
    let mut min = usize::max_value();
    while let Some((s, steps)) = queue.pop_front() {
        if steps > min {
            continue;
        }
        let (x, y) = s.coords;
        let c = maze[y][x];
        if c == '#'
            || (c.is_ascii_uppercase()
                && sector_keys.contains(&c.to_ascii_lowercase())
                && !s.keys.contains(&c.to_ascii_lowercase()))
        {
            // hit a wall or a door and don't have the key
            continue;
        }
        let mut keys = s.keys.clone();
        if c.is_ascii_lowercase() && !s.keys.contains(&c) {
            keys.push(c);
            keys.sort();
            if keys.len() == sector_keys.len() {
                min = steps;
                continue;
            }
        };
        for (x, y) in adjacent_coords(maze, s.coords) {
            let new_state = State {
                coords: (x, y),
                keys: keys.clone(),
            };
            if discovered.contains(&new_state) {
                continue;
            }
            discovered.insert(new_state.clone());
            queue.push_back((new_state, steps + 1));
        }
    }
    min
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let mut maze: Vec<Vec<char>> = input.split('\n').map(|l| l.chars().collect()).collect();
    let y_size = maze.len();
    let x_size = maze[0].len();
    // entrance (@)
    // open passages (.)
    // stone walls (#)
    // keys (lowercase letters) and doors (uppercase letters)

    // split into sections (there is probably a better way to do this...)
    let (ent_x, ent_y) = find_entrance(&maze);
    maze[ent_y - 1][ent_x - 1] = '@';
    maze[ent_y - 1][ent_x] = '#';
    maze[ent_y - 1][ent_x + 1] = '@';
    maze[ent_y][ent_x - 1] = '#';
    maze[ent_y][ent_x] = '#';
    maze[ent_y][ent_x + 1] = '#';
    maze[ent_y + 1][ent_x - 1] = '@';
    maze[ent_y + 1][ent_x] = '#';
    maze[ent_y + 1][ent_x + 1] = '@';

    // fill in dead ends
    let mut dead_end = true;
    while dead_end {
        dead_end = false;
        for y in 1..y_size - 1 {
            for x in 1..x_size - 1 {
                if maze[y][x] != '.' {
                    continue;
                }
                let adjacent = vec![
                    maze[y - 1][x],
                    maze[y + 1][x],
                    maze[y][x - 1],
                    maze[y][x + 1],
                ];
                if adjacent.iter().filter(|&c| c == &'#').count() == 3 {
                    maze[y][x] = '#';
                    dead_end = true;
                }
            }
        }
    }
    let result: usize = vec![
        (ent_x - 1, ent_y - 1),
        (ent_x - 1, ent_y + 1),
        (ent_x + 1, ent_y - 1),
        (ent_x + 1, ent_y + 1),
    ]
    .into_iter()
    .map(|coords| {
        let initial_state = State {
            coords,
            keys: Vec::new(),
        };
        search(&maze, initial_state)
    })
    .sum();

    // How many steps is the shortest path that collects all of the keys?
    println!("result: {:?}", result);
    // for line in maze.iter() {
    //     println!("{}", line.iter().collect::<String>());
    // }
}
