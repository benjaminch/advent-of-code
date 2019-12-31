use intcode::Vm;
use std::collections::VecDeque;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<i64> = get_instructions(&mut input.clone());
    let mut map: Map = Map::new(Position { x: 2, y: 2 }, Way::new(Direction::Up), 5, 5);

    // Part 1
    map.paint(instructions);

    // Part 2

    return Ok(());
}

fn get_instructions(input: &mut String) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(",")
        .flat_map(|e| e.parse::<i64>())
        .collect::<Vec<i64>>();
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn make_move(&mut self, direction: Direction, distance: u32) {
        match direction {
            Direction::Right => self.x += distance as usize,
            Direction::Down => self.y -= distance as usize,
            Direction::Left => self.x -= distance as usize,
            Direction::Up => self.y += distance as usize,
            _ => (),
        }
    }
}

#[derive(Clone)]
enum Direction {
    Unknown = -1,
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

fn get_direction(direction_raw: i64) -> Direction {
    return match direction_raw {
        0 => Direction::Left,
        1 => Direction::Right,
        2 => Direction::Up,
        3 => Direction::Down,
        _ => Direction::Unknown,
    };
}

fn direction_to_degrees(direction: Direction) -> i32 {
    return match direction {
        Direction::Right => 0,
        Direction::Down => 270,
        Direction::Left => 180,
        Direction::Up => 90,
        _ => -1,
    };
}

fn degrees_to_direction(angle: i32) -> Direction {
    let normalized_angle: i32 = angle % 360;
    if angle >= 0 && angle < 90 {
        return Direction::Right;
    } else if angle >= 90 && angle < 180 {
        return Direction::Up;
    } else if angle >= 180 && angle < 270 {
        return Direction::Left;
    } else {
        return Direction::Down;
    }
}

struct Way {
    direction: Direction,
}

impl Way {
    pub fn new(direction: Direction) -> Way {
        return Way {
            direction: direction,
        };
    }

    pub fn turn(&mut self, direction: Direction) {
        self.direction = degrees_to_direction(
            direction_to_degrees(direction) + direction_to_degrees(self.direction.clone()),
        );
    }
}

#[derive(Clone)]
enum Color {
    Unknown = -1,
    Black = 0,
    White = 1,
}

struct Map {
    current_position: Position,
    current_way: Way,
    positions: Vec<Color>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(current_position: Position, current_way: Way, width: usize, height: usize) -> Map {
        return Map {
            current_position: current_position,
            current_way: current_way,
            width: width,
            height: height,
            positions: vec![Color::Black; width * height],
        };
    }

    fn get_current_position_color(&self) -> Color {
        return self.positions[self.current_position.x * self.current_position.y].clone();
    }

    fn paint_current_position(&mut self, color: Color) {
        self.positions[self.current_position.x * self.current_position.y] = color;
    }

    pub fn turn_and_move(&mut self, direction: Direction, distance: u32) {
        self.current_way.turn(direction.clone());
        self.current_position.make_move(self.current_way.direction.clone(), distance);
    }

    pub fn paint(&mut self, instructions: Vec<i64>) {
        let vm = &mut Vm::new(instructions, VecDeque::new());
        for i in 0..10 {
            vm.reset();
            vm.add_input(match self.get_current_position_color() {
                Color::Black => 0,
                Color::White => 1,
                Color::Unknown => -1,
            });
            vm.run();

            if let [color_raw, direction_raw] = vm.outputs()[..].as_ref()  {
                self.paint_current_position(
                    match color_raw {
                        0 => Color::Black,
                        1 => Color::White,
                        _ => Color::Unknown,
                    });
                self.turn_and_move(get_direction(*direction_raw), 1);
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::with_capacity(self.width * self.height + self.height);
        for i in 0..self.positions.len() {
            if i % (self.width) == 0 {
                output.push_str("\n");
            }
            if i % self.width == self.current_position.x
                && (i / self.width) % self.height == self.current_position.y
            {
                match self.current_way.direction {
                    Direction::Left => output.push_str("<"),
                    Direction::Right => output.push_str(">"),
                    Direction::Up => output.push_str("^"),
                    Direction::Down => output.push_str("v"),
                    Direction::Unknown => output.push_str("x"),
                };
            } else {
                match self.positions[i] {
                    Color::Black => output.push_str("."),
                    Color::White => output.push_str("#"),
                    Color::Unknown => output.push_str(" "),
                };
            }
        }

        write!(f, "{}", output)
    }
}
