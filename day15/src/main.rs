use intcode::{InstructionResult, Program};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use std::collections::HashMap;

/*
movement commands: north (1), south (2), west (3), and east (4)

status codes:
0: The repair droid hit a wall. Its position has not changed.
1: The repair droid has moved one step in the requested direction.
2: The repair droid has moved one step in the requested direction; its new
position is the location of the oxygen system.
*/

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    System,
}

#[derive(Clone, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::West, Dir::East];

impl From<isize> for Dir {
    fn from(i: isize) -> Self {
        match i {
            1 => Self::North,
            2 => Self::South,
            3 => Self::West,
            4 => Self::East,
            _ => panic!("invalid direction"),
        }
    }
}

impl Into<isize> for Dir {
    fn into(self) -> isize {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(isize, isize);

impl Pos {
    fn adjacent(&self, dir: &Dir) -> Self {
        match dir {
            Dir::North => Self(self.0, self.1 + 1),
            Dir::South => Self(self.0, self.1 - 1),
            Dir::West => Self(self.0 - 1, self.1),
            Dir::East => Self(self.0 + 1, self.1),
        }
    }
}

fn pick_direction(map: &HashMap<Pos, Tile>, pos: &Pos) -> Dir {
    // Check for unexplored at current location
    let mut tiles = Vec::new();
    for dir in DIRS.iter() {
        let adjacent = map.get(&pos.adjacent(dir));
        match adjacent {
            None => return dir.clone(),
            Some(tile) => {
                tiles.push(tile);
            }
        }
    }
    // Try a random empty direction (not efficient, but simple)
    let mut rng = thread_rng();
    tiles
        .iter()
        .enumerate()
        .filter(|(_, &t)| t == &Tile::Empty)
        .map(|(i, _)| ((i + 1) as isize).into())
        .choose(&mut rng)
        .unwrap()
}

fn render(map: &HashMap<Pos, Tile>) {
    let max_x = map.keys().map(|p| p.0).max().unwrap();
    let min_x = map.keys().map(|p| p.0).min().unwrap();
    let max_y = map.keys().map(|p| p.1).max().unwrap();
    let min_y = map.keys().map(|p| p.1).min().unwrap();
    let mut buffer: Vec<Vec<char>> = (min_y..=max_y)
        .map(|_| (min_x..=max_x).map(|_| ' ').collect())
        .collect();
    for (pos, tile) in map.iter() {
        let c = if pos == &Pos(0, 0) {
            'x'
        } else {
            match tile {
                Tile::Empty => '.',
                Tile::Wall => '█',
                Tile::System => '⊙',
            }
        };
        buffer[(pos.1 - min_y) as usize][(pos.0 - min_x) as usize] = c;
    }
    for line in buffer.iter() {
        println!("{}", line.iter().collect::<String>());
    }
}

fn main() {
    let mut program = Program::from_str(include_str!("../input.txt"), Vec::new());
    let mut map = HashMap::<Pos, Tile>::new();
    map.insert(Pos(0, 0), Tile::Empty);
    let mut last_dir = Dir::North;
    let mut pos = Pos(0, 0);
    let mut last_len = 0;
    let mut iterations = 30000;
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => {
                // In order to uncover the whole map, keep going until no new
                // map tiles are uncovered for 30k iterations...
                if map.len() != last_len {
                    iterations = 30000;
                    last_len = map.len();
                } else {
                    iterations -= 1;
                    if iterations == 0 {
                        break;
                    }
                }
                last_dir = pick_direction(&map, &pos);
                program.input.push(last_dir.clone().into());
            }
            InstructionResult::Output(value) => match value {
                0 => {
                    map.insert(pos.adjacent(&last_dir), Tile::Wall);
                }
                1 => {
                    pos = pos.adjacent(&last_dir);
                    map.insert(pos, Tile::Empty);
                }
                2 => {
                    pos = pos.adjacent(&last_dir);
                    map.insert(pos, Tile::System);
                }
                _ => panic!("unknown status code"),
            },
        }
    }

    /* What is the fewest number of movement commands required to move the
    repair droid from its starting position to the location of the oxygen
    system? */
    render(&map);
}
