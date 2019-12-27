use intcode::Vm;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::collections::VecDeque;


fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

	let instructions: Vec<i64> = get_instructions(input.clone());
    let mut map: Map = Map::new(Position { x: 2, y: 2 }, Direction::Up, 5, 5);
    println!("{}", map);

    // Part 1

    // Part 2

    return Ok(());
}

fn paint(instructions: Vec<i64>, map: &mut Map) {
    let mut vm: Vm = Vm::new(instructions, VecDeque::from(vec![match map.get_current_position_color() {
		Color::Black => 0_i64,
		Color::White => 1_i64}]);
	
	// TODO: execute VM, parse outputs at the end
}

fn get_instructions(input: &mut String) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(",")
        .flat_map(|e| e.parse::<i64>())
        .collect::<Vec<i64>>();
}

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[derive(Clone)]
enum Color {
    Black = 0,
    White = 1,
}

struct Map {
    current_position: Position,
    current_direction: Direction,
    positions: Vec<Color>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(
        current_position: Position,
        current_direction: Direction,
        width: usize,
        height: usize,
    ) -> Map {
        return Map {
            current_position: current_position,
            current_direction: current_direction,
            width: width,
            height: height,
            positions: vec![Color::Black; width * height],
        };
    }
	
	pub fn get_current_position_color() -> Color {
		return self.positions[self.current_position.x * self.current_position.y];
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
                match self.current_direction {
                    Direction::Left => output.push_str("<"),
                    Direction::Right => output.push_str(">"),
                    Direction::Up => output.push_str("^"),
                    Direction::Down => output.push_str("v"),
                };
            } else {
                match self.positions[i] {
                    Color::Black => output.push_str("."),
                    Color::White => output.push_str("#"),
                };
            }
        }

        write!(f, "{}", output)
    }
}
