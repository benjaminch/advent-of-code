use std::collections::HashSet;
use std::io::{self, Error, Read, Write};
use std::iter::FromIterator;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut wires = input.lines();
    let wire_1: Vec<Position> = compute_path(wires.next().unwrap().to_string());
    let wire_2: Vec<Position> = compute_path(wires.next().unwrap().to_string());

    // Part 1
    writeln!(
        io::stdout(),
        "Min intersection distance: {:?}",
        find_min_intersection_distance(&wire_1, &wire_2).unwrap()
    )?;

    // Part 2
    writeln!(
        io::stdout(),
        "Min steps intersection: {:?}",
        find_min_steps_intersection(&wire_1, &wire_2).unwrap()
    )?;

    Ok(())
}

fn find_min_intersection_distance(wire_1: &Vec<Position>, wire_2: &Vec<Position>) -> Option<i32> {
    let mut min_distance: Option<i32> = None;
    for p in get_intersections(&wire_1, &wire_2) {
        let distance: i32 = p.x.abs() + p.y.abs();
        if min_distance.is_none() || min_distance.unwrap() > distance {
            min_distance = Some(distance);
        }
    }
    min_distance
}

fn find_min_steps_intersection(wire_1: &Vec<Position>, wire_2: &Vec<Position>) -> Option<u32> {
    let mut min_steps: Option<u32> = None;
    for intersection in get_intersections(&wire_1, &wire_2) {
        if let (Some(steps_w1), Some(steps_w2)) = (
            steps_to_position(intersection, &wire_1),
            steps_to_position(intersection, &wire_2),
        ) {
            let steps: u32 = steps_w1 + steps_w2;
            if min_steps.is_none() || steps < min_steps.unwrap() {
                min_steps = Some(steps)
            }
        }
    }
    min_steps
}

fn steps_to_position(position: Position, wire: &Vec<Position>) -> Option<u32> {
    let mut steps: u32 = 0;
    for p in wire {
        if position == *p {
            return Some(steps);
        }
        steps += 1;
    }
    None
}

fn get_intersections(wire_1: &Vec<Position>, wire_2: &Vec<Position>) -> Vec<Position> {
    let w1: HashSet<Position> = HashSet::from_iter(wire_1.iter().skip(1).cloned());
    let w2: HashSet<Position> = HashSet::from_iter(wire_2.iter().skip(1).cloned());
    return w1
        .intersection(&w2)
        .map(|p| Position { x: p.x, y: p.y })
        .collect::<Vec<Position>>();
}

fn compute_path(input: String) -> Vec<Position> {
    let moves: Vec<Move> = input
        .split(',')
        .map(|e| get_move(e.to_string()))
        .collect::<Vec<Move>>();

    let mut positions: Vec<Position> = Vec::new();
    positions.push(Position { x: 0, y: 0 });
    for m in moves {
        let last_position: Position = *positions.last().unwrap();
        for p in get_positions(m, last_position) {
            positions.push(p);
        }
    }

    positions
}

fn get_positions(m: Move, p: Position) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let mut moves: i32 = 1;
    let mut position: Position = p;
    while moves <= m.distance {
        position = match m.direction {
            Direction::Up => Position {
                x: position.x,
                y: position.y + 1,
            },
            Direction::Down => Position {
                x: position.x,
                y: position.y - 1,
            },
            Direction::Left => Position {
                x: position.x - 1,
                y: position.y,
            },
            Direction::Right => Position {
                x: position.x + 1,
                y: position.y,
            },
        };
        positions.push(position);
        moves += 1;
    }
    positions
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Move {
    direction: Direction,
    distance: i32,
}
