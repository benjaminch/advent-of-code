use intcode::{State, Vm};
use std::collections::VecDeque;
use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<i64> = get_instructions(&mut input);
    let mut map: Map = Map::new(Position { x: 2, y: 2 }, Way::new(Direction::Up), 5, 5);

    // Part 1
    map.paint(instructions);

    // Part 2

    Ok(())
}

fn get_instructions(input: &mut String) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(',')
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
    match direction_raw {
        0 => Direction::Left,
        1 => Direction::Right,
        2 => Direction::Up,
        3 => Direction::Down,
        _ => Direction::Unknown,
    }
}

fn direction_to_degrees(direction: Direction) -> i32 {
    match direction {
        Direction::Right => 0,
        Direction::Down => 270,
        Direction::Left => 180,
        Direction::Up => 90,
        _ => -1,
    }
}

fn degrees_to_direction(angle: i32) -> Direction {
    let _normalized_angle: i32 = angle % 360;
    if (0..90).contains(&angle) {
        Direction::Right
    } else if (90..180).contains(&angle) {
        Direction::Up
    } else if (180..270).contains(&angle) {
        Direction::Left
    } else {
        Direction::Down
    }
}

struct Way {
    direction: Direction,
}

impl Way {
    pub fn new(direction: Direction) -> Way {
        Way { direction }
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
        Map {
            current_position,
            current_way,
            width,
            height,
            positions: vec![Color::Black; width * height],
        }
    }

    fn get_current_position_color(&self) -> Color {
        self.positions[self.current_position.x * self.current_position.y].clone()
    }

    fn paint_current_position(&mut self, color: Color) {
        self.positions[self.current_position.x * self.current_position.y] = color;
    }

    pub fn turn_and_move(&mut self, direction: Direction, distance: u32) {
        self.current_way.turn(direction);
        self.current_position
            .make_move(self.current_way.direction.clone(), distance);
    }

    pub fn paint(&mut self, instructions: Vec<i64>) {
        let vm = &mut Vm::new(instructions, VecDeque::new());
        let mut outputs: Vec<i64> = Vec::new();

        println!("{}", self);

        while vm.state() != &State::Stopped {
            vm.reset();
            vm.add_input(match self.get_current_position_color() {
                Color::Black => 0,
                Color::White => 1,
                Color::Unknown => -1,
            });

            // TODO: refactor
            vm.run(true);
            outputs.push(vm.outputs()[0]);
            vm.run(true);
            outputs.push(vm.outputs()[1]);

            if let [color_raw, direction_raw] = outputs[..] {
                self.paint_current_position(match color_raw {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => Color::Unknown,
                });
                // TODO: there is likely an issue reading / writting data to the map
                // coordinates, to be checked
                self.turn_and_move(get_direction(direction_raw), 1);
            }

            println!("{:?}", outputs);
            println!("{}", self);

            outputs.clear();
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::with_capacity(self.width * self.height + self.height);
        for i in 0..self.positions.len() {
            if i % (self.width) == 0 {
                output.push('\n');
            }
            if i % self.width == self.current_position.x
                && (i / self.width) % self.height == self.current_position.y
            {
                match self.current_way.direction {
                    Direction::Left => output.push('<'),
                    Direction::Right => output.push('>'),
                    Direction::Up => output.push('^'),
                    Direction::Down => output.push('v'),
                    Direction::Unknown => output.push('x'),
                };
            } else {
                match self.positions[i] {
                    Color::Black => output.push('.'),
                    Color::White => output.push('#'),
                    Color::Unknown => output.push(' '),
                };
            }
        }

        write!(f, "{}", output)
    }
}
