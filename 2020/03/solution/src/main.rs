use std::io::{self, Error, ErrorKind, Read, Write};
use std::str::FromStr;

#[derive(PartialEq)]
enum Geology {
    Tree,
    Free,
    Unknown,
}

impl FromStr for Geology {
    type Err = Error;

    fn from_str(input: &str) -> Result<Geology, Self::Err> {
        match input {
            "#" => Ok(Geology::Tree),
            "." => Ok(Geology::Free),
            _ => Ok(Geology::Unknown),
        }
    }
}

struct Cell {
    geology: Geology,
}

impl FromStr for Cell {
    type Err = Error;

    fn from_str(input: &str) -> Result<Cell, Self::Err> {
        Ok(Cell::new(Geology::from_str(input)?))
    }
}

impl Cell {
    fn new(geology: Geology) -> Cell {
        Cell { geology }
    }
}

struct Map {
    cells: Vec<Cell>,
    map_width: usize,
    map_height: usize,
    current_cell_x: usize,
    current_cell_y: usize,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Map, Self::Err> {
        let mut cells: Vec<Cell> = Vec::new();
        let mut map_width: usize = 0;
        let mut map_height: usize = 0;

        for (i, line) in input.lines().enumerate() {
            for c in line.chars() {
                if i == 0 {
                    map_width += 1;
                }

                cells.push(Cell::from_str(c.to_string().as_str())?);
            }

            map_height += 1;
        }

        Ok(Map::new(map_width, map_height, cells))
    }
}

impl Map {
    fn new(map_width: usize, map_height: usize, cells: Vec<Cell>) -> Map {
        Map {
            cells,
            current_cell_x: 0,
            current_cell_y: 0,
            map_width,
            map_height,
        }
    }

    fn reset_position(&mut self) {
        self.current_cell_x = 0;
        self.current_cell_y = 0;
    }

    fn has_next(&self, mov: Move) -> bool {
        // It's ok to overflow on the right, but it's not to overflow on the bottom
        self.current_cell_y + mov.bottom < self.map_height
    }

    fn next(&mut self, mov: Move) -> Result<&Cell, Error> {
        if !self.has_next(mov) {
            return Err(Error::new(ErrorKind::Other, "no more items"));
        }

        let next_x_index = (self.current_cell_x + mov.right).rem_euclid(self.map_width);
        let next_y_index = self.current_cell_y + mov.bottom;
        let next_index = next_x_index + (next_y_index * self.map_width);

        self.current_cell_x = next_x_index;
        self.current_cell_y = next_y_index;

        println!("x {} y {}", self.current_cell_x, self.current_cell_y);

        Ok(self.cells.get(next_index).unwrap())
    }
}

#[derive(Clone, Copy)]
struct Move {
    right: usize,
    bottom: usize,
}

impl Move {
    fn new(right: usize, bottom: usize) -> Move {
        Move { right, bottom }
    }
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut map = Map::from_str(&input)?;

    // Part 1
    let part_1_move = Move::new(3, 1);
    let mut part_1_trees_count: i32 = 0;

    while map.has_next(part_1_move) {
        if map.next(part_1_move)?.geology == Geology::Tree {
            part_1_trees_count += 1;
        }
    }
    map.reset_position();

    writeln!(io::stdout(), "Part - 1 / Trees: {}", part_1_trees_count)?;

    // Part 2

    let mut part_2_trees_count = 0;
    let mut part_2_partial_result: i64 = 0;
    let part_2_moves: Vec<Move> = vec![
        Move::new(1, 1),
        Move::new(3, 1),
        Move::new(5, 1),
        Move::new(7, 1),
        Move::new(1, 2),
    ];

    for (i, m) in part_2_moves.iter().enumerate() {
        map.reset_position();
        part_2_partial_result = 0;

        while map.has_next(*m) {
            if map.next(*m)?.geology == Geology::Tree {
                part_2_partial_result += 1;
            }
        }

        if i == 0 {
            part_2_trees_count = part_2_partial_result;
        } else {
            part_2_trees_count *= part_2_partial_result;
        }
    }

    writeln!(io::stdout(), "Part - 2 / Trees: {}", part_2_trees_count)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
