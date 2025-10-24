use std::error::Error as StdError;
use std::io::{self, Read, Write};

use aoc_2024::day01::{get_lists_from_str, get_similarity, get_total_distance};

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
