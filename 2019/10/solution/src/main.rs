use std::io::{self, Error, ErrorKind,  Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
	writeln!(io::stdout(), "{}", build_map(input.clone()).unwrap())?;

    // Part 2

    return Ok(());
}

fn get_detected_asteroids(asteroid_position: Position, asteroids_positions: Vec<Position>) -> i32 {
	let base_vector: (Position, Position) = (Position { x: 0, y: 0 }, Position { x: 0, y: 10 });
	let mut HashMap<f32, Position> detected_asteroids = HashMap::new();

	for asteroid in asteroids_positions {
		// TODO: implement here
	}
}

fn find_asteroid_detecting_more_asteroids(map: Map) -> Option<<Position, i32>> {
	let mut result: Option<(Position, i32)> = None;

	for position in map.positions {
		let detected_asteroids: i32	= get_detected_asteroids(&position);
		if result.is_none() || detected_asteroids > result.unwrap().1 {
			result = Some((position, detected_asteroids))
		} 
	}

	return result;	
}

fn build_map(input: String) -> Result<Map, Error> {
    let mut positions: Vec<Position> = Vec::new();
    let mut width: usize = 0;
    let mut height: usize = 0;

    for row in input.lines() {
        for cell in row.chars() {
			positions.push(Position {
				x: width,
				y: height,
				asteroid: match cell {
					'#' => Some(Asteroid {}),
					'.' => None,
					_ => return Err(Error::new(ErrorKind::InvalidInput, "unknown char found!")),
				}
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

struct Asteroid {}

struct Position {
    x: usize,
    y: usize,
	asteroid: Option<Asteroid>,
}

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
            output.push_str(
				match &self.positions[i].asteroid {
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
