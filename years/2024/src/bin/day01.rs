use std::collections::HashMap;
use std::error::Error as StdError;
use std::io::{self, Read, Write};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum Aoc2024Day1Error {
    #[error("cannot read from stdin")]
    CannotReadFromStdIn,
    #[error("cannot read from stdout")]
    CannotWriteToStdOut,
    #[error("wrong input: {message}")]
    WrongInput { message: String },
}

pub fn get_lists_from_str(input: &str) -> Result<(Vec<i32>, Vec<i32>), Aoc2024Day1Error> {
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.trim().split_ascii_whitespace();
        let first = parts.next().ok_or_else(|| Aoc2024Day1Error::WrongInput {
            message: format!("Line from input is not properly formatted: `{}`", line),
        })?;
        let second = parts.next().ok_or_else(|| Aoc2024Day1Error::WrongInput {
            message: format!("Line from input is not properly formatted: `{}`", line),
        })?;

        list_one.push(first.parse().map_err(|_| Aoc2024Day1Error::WrongInput {
            message: format!("Could not parse number from `{}`", first),
        })?);
        list_two.push(second.parse().map_err(|_| Aoc2024Day1Error::WrongInput {
            message: format!("Could not parse number from `{}`", second),
        })?);
    }

    Ok((list_one, list_two))
}

pub fn get_total_distance(list_two: &[i32], list_one: &[i32]) -> Result<u32, Aoc2024Day1Error> {
    if list_one.len() != list_two.len() {
        return Err(Aoc2024Day1Error::WrongInput {
            message: "Lists must have the same length".to_string(),
        });
    }

    // Not ideal but I rather prefer not to have to receive mutatble references
    let mut list_one = list_one.to_vec();
    list_one.sort_unstable();
    // Not ideal but I rather prefer not to have to receive mutatble references
    let mut list_two = list_two.to_vec();
    list_two.sort_unstable();

    Ok(list_one
        .iter()
        .zip(list_two.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum())
}

pub fn get_similarity(list_one: &[i32], list_two: &[i32]) -> Result<i32, Aoc2024Day1Error> {
    if list_one.len() != list_two.len() {
        return Err(Aoc2024Day1Error::WrongInput {
            message: "Lists must have the same length".to_string(),
        });
    }
    let mut list_one_hashmap = HashMap::new();

    for item in list_two {
        *list_one_hashmap.entry(item).or_insert(0) += 1;
    }

    let mut similarity = 0;
    for item in list_one {
        if list_one_hashmap.contains_key(item) {
            similarity += item * list_one_hashmap.get(item).unwrap_or(&0);
        }
    }

    Ok(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lists_from_str() {
        assert_eq!(
            get_lists_from_str("1   2\n3   4\n").expect("should not fail"),
            (vec![1, 3], vec![2, 4])
        );
    }

    #[test]
    fn test_get_total_distance() {
        assert_eq!(
            get_total_distance(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]).expect("should not fail"),
            11
        );
    }

    #[test]
    fn test_get_similarity() {
        assert_eq!(
            get_similarity(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]).expect("should not fail"),
            31
        );
    }
}

fn main() -> Result<(), Box<dyn StdError>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (list_one, list_two) = get_lists_from_str(&input)?;
    writeln!(
        io::stdout(),
        "Part 1 / Total distance: {}",
        get_total_distance(&list_one, &list_two)?
    )?;
    writeln!(
        io::stdout(),
        "Part 2 / Total similarity: {}",
        get_similarity(&list_one, &list_two)?
    )?;
    Ok(())
}
