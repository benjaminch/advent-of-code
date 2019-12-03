use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut wires = input.lines();
    let mut wire_1: String = wires.next().unwrap().to_string();
    let mut wire_2: String = wires.next().unwrap().to_string();

    // Part 1
    println!("{:?}", compute_path(wire_1));

    // Part 2
    
    return Ok(());
}

fn compute_path(input: String) -> Vec<Position> {
   let moves: Vec<Move> = input
        .split(",")
        .map(|e| get_move(e.to_string()))
        .collect::<Vec<Move>>();

   let positions: Vec<Position> = Vec::new(); 
   positions.push(Position { x: 0, y: 0 });
   for m in moves {
       positions.push(get_position(m, positions.last().unwrap()));
   }

   return positions;
} 

fn get_position(m: Move, position: Position) -> Position {
    match m.direction {
        Direction::Up => Position { x: position.x, y: position.y + m.distance },
        Direction::Down => Position { x: position.x, y: position.y - m.distance },
        Direction::Left => Position { x: position.x - m.distance, y: position.y },
        Direction::Right => Position { x: position.x + m.distance, y: position.y },
    }
}

fn get_direction(symbol: char) -> Option<Direction> {
    match symbol {
        'U' => Some(Direction::Up),
        'D' => Some(Direction::Down),
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Right),
        _ => None,
    }
}

fn get_move(input: String) -> Move {
    let mut chars = input.chars();
    return Move {
        direction: get_direction(chars.next().unwrap()).unwrap(),
        distance: chars.as_str().parse::<i32>().unwrap(),
    };
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: i32,
}
