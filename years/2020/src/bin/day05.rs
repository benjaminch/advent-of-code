use std::io::{self, Error, Read, Write};
use std::str::FromStr;

#[derive(Debug)]
struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn new(row: u32, column: u32) -> Seat {
        Seat { row, column }
    }

    fn id(&self) -> u32 {
        (self.row * 8) + self.column
    }
}

impl FromStr for Seat {
    type Err = Error;

    fn from_str(input: &str) -> Result<Seat, Self::Err> {
        const BINARY_SIZE: usize = 10;

        if input.len() != BINARY_SIZE {
            return Err(Error::other("input must be 10 chars long"));
        }

        let mut row_max: u32 = 127;
        let mut row_min: u32 = 0;
        let mut column_max: u32 = 7;
        let mut column_min: u32 = 0;

        for (i, c) in input.chars().enumerate() {
            // row
            if i <= 6 {
                if c != 'B' && c != 'F' {
                    return Err(Error::other(
                        "first 7 chars must be either 'B' or 'F'",
                    ));
                }

                // lower half
                if c == 'F' {
                    row_max -= (row_max - row_min + 1).div_euclid(2);
                }
                // upper half
                else {
                    row_min += (row_max - row_min + 1).div_euclid(2);
                }
            }
            // column
            else {
                if c != 'L' && c != 'R' {
                    return Err(Error::other(
                        "last 3 chars must be either 'L' or 'R'",
                    ));
                }

                // lower half
                if c == 'L' {
                    column_max -= (column_max - column_min + 1).div_euclid(2);
                }
                // upper half
                else {
                    column_min += (column_max - column_min + 1).div_euclid(2);
                }
            }
        }

        Ok(Seat::new(row_min, column_min))
    }
}

fn get_missing_seat_id(seat_ids: Vec<u32>) -> Option<u32> {
    let mut ids = seat_ids;
    ids.sort_unstable();

    let mut min_seat_id: u32 = 0;

    for (i, seat_id) in ids.iter().enumerate() {
        if i == 0 {
            min_seat_id = *seat_id;
        }
        if (i + min_seat_id as usize) as u32 != *seat_id {
            return Some(*seat_id - 1);
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let part_1_max_seat_id: u32 = input
        .lines()
        .map(|x| Seat::from_str(x).unwrap().id())
        .max()
        .unwrap_or(0u32);

    writeln!(
        io::stdout(),
        "Part - 1 / Max Seat ID: {:?}",
        part_1_max_seat_id
    )?;

    // Part 2
    let part_2_missing_seat_id: u32 = get_missing_seat_id(
        input
            .lines()
            .map(|x| Seat::from_str(x).unwrap().id())
            .collect(),
    )
    .unwrap_or(0u32);

    writeln!(
        io::stdout(),
        "Part - 2 / Missing Seat ID: {:?}",
        part_2_missing_seat_id
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {}
