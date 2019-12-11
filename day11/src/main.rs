use intcode::{InstructionResult, Program};
use std::collections::HashMap;
use std::iter;

type Position = (isize, isize);
enum Orientation {
    U,
    R,
    D,
    L,
}

impl Orientation {
    fn rotate(&self, direction: isize) -> Orientation {
        // 0 -> turn left 90 degrees, 1 -> turn right 90 degrees
        match direction {
            0 => match self {
                Orientation::U => Orientation::L,
                Orientation::R => Orientation::U,
                Orientation::D => Orientation::R,
                Orientation::L => Orientation::D,
            },
            1 => match self {
                Orientation::U => Orientation::R,
                Orientation::R => Orientation::D,
                Orientation::D => Orientation::L,
                Orientation::L => Orientation::U,
            },
            _ => panic!("unknown direction"),
        }
    }
}

struct Robot {
    position: Position,
    orientation: Orientation,
}

impl Robot {
    fn new() -> Robot {
        // robot starts at origin facing up
        Robot {
            position: (0, 0),
            orientation: Orientation::U,
        }
    }

    fn move_panels(&mut self, direction: isize) {
        // rotate
        self.orientation = self.orientation.rotate(direction);
        let (x, y) = self.position;
        // move forward 1 panel
        self.position = match self.orientation {
            Orientation::U => (x, y + 1),
            Orientation::R => (x + 1, y),
            Orientation::D => (x, y - 1),
            Orientation::L => (x - 1, y),
        };
    }
}

fn main() {
    let mut program = Program::from_str(include_str!("../input.txt"), Vec::new());
    let mut robot = Robot::new();
    // 0 -> black panel, 1 -> white panel
    let mut panels = HashMap::<Position, isize>::new();
    // start on a white panel
    panels.insert((0, 0), 1);
    let mut outputing_color = true;
    loop {
        match program.execute_instruction() {
            InstructionResult::None => continue,
            InstructionResult::Halt => break,
            InstructionResult::InputBlocked => {
                // input is the color of the current panel; panels start black
                program
                    .input
                    .push(*panels.get(&robot.position).unwrap_or(&0))
            }
            InstructionResult::Output(value) => {
                if outputing_color {
                    // first output: color to paint current panel
                    panels.insert(robot.position.clone(), value);
                    outputing_color = false;
                } else {
                    // second output: direction to turn
                    robot.move_panels(value);
                    outputing_color = true;
                }
            }
        }
    }
    // determine grid size
    let min_x = *panels.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *panels.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *panels.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *panels.keys().map(|(_, y)| y).max().unwrap();
    // fill grid with 0s
    let mut grid: Vec<Vec<_>> = (min_y..=max_y)
        .map(|_| iter::repeat(0).take((max_x - min_x + 1) as usize).collect())
        .collect();
    // apply panel colors to grid
    for ((x, y), color) in panels.iter() {
        grid[(y - min_y) as usize][(x - min_x) as usize] = *color;
    }
    // what registration identifier does it paint (eight capital letters)
    for row in grid.iter().rev() {
        println!(
            "{}",
            row.iter()
                .map(|i| match i {
                    0 => ' ',
                    1 => '▮',
                    _ => panic!("unknown color in grid"),
                })
                .collect::<String>()
        );
    }
}
