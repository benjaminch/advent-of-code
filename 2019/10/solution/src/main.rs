use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;
use std::cmp::Ordering;
use std::io::{self, Error, ErrorKind, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let best_position: PositionDetected = find_asteroid_detecting_most_asteroids(build_map(input.clone()).unwrap()).unwrap();
    writeln!(io::stdout(), "Part 1: {:?}", best_position)?;

    // Part 2
    let map: Map = build_map(input.clone()).unwrap();
    let vaporized_200th = vaporize(&best_position.position, &map.positions)[199];
	writeln!(io::stdout(), "Part 2: {:?}", vaporized_200th)?;

    return Ok(());
}

fn vaporize(asteroid_position: &Position, asteroids_positions: &Vec<Position>) -> Vec<Position> {
	let mut last_angle: Option<f64> = None;
	let mut vaporized: Vec<Position> = Vec::new();
    let mut vaporized_hs: HashSet<Position> = HashSet::new();
	let mut detected_asteroids: Vec<ExploredAsteroid> = get_detected_asteroids(&asteroid_position, &asteroids_positions)
		.iter()
		.map(|(k, v)| v.clone())
		.flatten()
		.collect();

	// Sort by angle then by distance
	detected_asteroids.sort();

	while vaporized.len() < detected_asteroids.len() {
		for asteroid in detected_asteroids.clone() {
			if let Some(angle) = last_angle {
					println!("{} / {}", angle, asteroid.angle_from_source);
				if angle - asteroid.angle_from_source < 0.1 {
					continue;
				}
			}
			if vaporized_hs.contains(&asteroid.position) {
				continue;
			}
			last_angle = Some(asteroid.angle_from_source);
			vaporized_hs.insert(asteroid.position);
			vaporized.push(asteroid.position);
		}
	}
	
    return vaporized;
}

fn get_detected_asteroids(
    asteroid_position: &Position,
    asteroids_positions: &Vec<Position>,
) -> HashMap<i64, Vec<ExploredAsteroid>> {
    let mut detected_asteroids: HashMap<i64, Vec<ExploredAsteroid>> = HashMap::new();

    for other_asteroid_position in asteroids_positions.iter() {
        if other_asteroid_position.asteroid.is_some()
            && other_asteroid_position != asteroid_position
        {
            let mut angle: f64 = (((other_asteroid_position.y as f64 - asteroid_position.y as f64)
                as f64)
                .atan2(other_asteroid_position.x as f64 - asteroid_position.x as f64)
                .to_degrees())
                .round();
			// make sure angles start at 12 O'Clock
			angle = (angle + 270.0) % 360.0; 
            let other_asteroid_distance: i64 = (asteroid_position.x as i64 - other_asteroid_position.x as i64).abs() + (asteroid_position.y as i64 - other_asteroid_position.y as i64).abs();

            if let Some(detected_asteroids) = detected_asteroids.get_mut(&((angle*1000.0).round() as i64)) {
                    detected_asteroids.push(ExploredAsteroid { position: *other_asteroid_position, angle_from_source: angle, distance_from_source: other_asteroid_distance });
            } else {
                detected_asteroids.insert((angle * 1000.0).round() as i64, vec![ExploredAsteroid { position: *other_asteroid_position, angle_from_source: angle, distance_from_source: other_asteroid_distance }]);
            }
        }
    }

    return detected_asteroids;
}

fn find_asteroid_detecting_most_asteroids(map: Map) -> Option<PositionDetected> {
    let mut result: Option<PositionDetected> = None;

    for position in map.positions.iter() {
        if position.asteroid.is_none() {
            continue;
        }

        let detected_asteroids: usize = get_detected_asteroids(&position, &map.positions).len();
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

    return Ok(Map {
        width: width,
        height: height,
        positions: positions,
    });
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Asteroid {}

#[derive(Debug, Clone)]
struct ExploredAsteroid {
    position: Position,
    angle_from_source: f64,
    distance_from_source: i64,
}

impl Ord for ExploredAsteroid {
    fn cmp(&self, other: &Self) -> Ordering {
		((self.angle_from_source * 1000.0).round() as i64, self.distance_from_source).cmp(&((other.angle_from_source * 1000.0).round() as i64, other.distance_from_source))
    }
}

impl PartialOrd for ExploredAsteroid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExploredAsteroid {
    fn eq(&self, other: &Self) -> bool {
        (self.angle_from_source, self.distance_from_source) == (other.angle_from_source, other.distance_from_source)
    }
}

impl Eq for ExploredAsteroid { }

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
