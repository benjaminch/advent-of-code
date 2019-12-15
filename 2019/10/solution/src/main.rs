
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let map: Map = build_map(input.clone()).unwrap();

    // Part 1
    let best_position: PositionDetected = find_asteroid_detecting_most_asteroids(map).unwrap();
    writeln!(io::stdout(), "Part 1: {:?}", best_position)?;

    // Part 2

    return Ok(());
}

fn get_detected_asteroids(
    asteroid_position: &Position,
    asteroids_positions: &Vec<Position>,
) -> usize {
    let mut detected_asteroids: HashMap<i64, (Position, f32)> = HashMap::new();

    for other_asteroid_position in asteroids_positions.iter().filter(|p| p.asteroid.is_some()) {
        if other_asteroid_position != asteroid_position {
            let angle: i64 = (((other_asteroid_position.y as i32 - asteroid_position.y as i32)
                as f64)
                .atan2(other_asteroid_position.x as f64 - asteroid_position.x as f64)
                .to_degrees()
                * 1000000.0)
                .round() as i64;
            let other_asteroid_distance: f32 = ((other_asteroid_position.x as i32
                - asteroid_position.x as i32)
                .pow(2) as f32
                + (other_asteroid_position.y as i32 - asteroid_position.y as i32).pow(2) as f32)
                .sqrt();

            if let Some(detected_asteroid) = detected_asteroids.get(&angle) {
                if detected_asteroid.1 > other_asteroid_distance {
                    detected_asteroids
                        .insert(angle, (*other_asteroid_position, other_asteroid_distance));
                }
            } else {
                detected_asteroids
                    .insert(angle, (*other_asteroid_position, other_asteroid_distance));
            }
        }
    }

    return detected_asteroids.len();
}

fn find_asteroid_detecting_most_asteroids(map: Map) -> Option<PositionDetected> {
    let mut result: Option<PositionDetected> = None;

    for position in map.positions.iter().filter(|p| p.asteroid.is_some()) {
        let detected_asteroids: usize = get_detected_asteroids(&position, &map.positions);
        if result.is_none() || detected_asteroids > result.as_ref().unwrap().detected {
            result = Some(PositionDetected {
                position: *position,
                detected: detected_asteroids,
            });
        }
    }

    return result;
}

fn build_map(input: String) -> Result<Map, Error> {
    let mut positions: Vec<Position> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    for row in input.lines() {
        height = 0;
        for cell in row.chars() {
            positions.push(Position {
                x: height,
                y: width,
                asteroid: match cell {
                    '#' => Some(Asteroid {}),
                    '.' => None,
                    _ => return Err(Error::new(ErrorKind::InvalidInput, "unknown char found!")),
                },
            });
            height += 1;
        }
        width += 1;
    }

    // println!("{:?}", positions);

    return Ok(Map {
        width: width,
        height: height,
        positions: positions,
    });
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Asteroid {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
    asteroid: Option<Asteroid>,
}

#[derive(Debug)]
struct PositionDetected {
    position: Position,
    detected: usize,
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    positions: Vec<Position>,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::with_capacity(self.width * self.height + self.height);
        for i in 0..self.positions.len() {
            if i % (self.width) == 0 {
                output.push_str("\n");
            }
            output.push_str(match &self.positions[i].asteroid {
                Some(_) => "#",
                None => ".",
            });
        }

        write!(f, "{}", output)
    }
}

// 1 - Compute angles between all points => put in a for each angle dictionary <angle, asteroid>
// 2 - When adding, if there is already a point, then we should keep only the one having the
//   shortest distance with the point observed
// 3 - Result is the point having the more entries in the dic
