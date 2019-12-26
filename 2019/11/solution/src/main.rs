use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut map: Map = Map::new(Position { x: 2, y: 2 }, Direction::Up, 5, 5);
    println!("{}", map);

    // Part 1

    // Part 2

    return Ok(());
}

// fn paint(position: &mut Position, map: &mut Vec<Vec<u8>>) {
//
// }

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
