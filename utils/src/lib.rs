//! Common utilities for Advent of Code solutions across all years

use std::fs;

/// Read input file from the inputs directory
///
/// # Arguments
/// * `day` - Day number (1-25)
///
/// # Example
/// ```ignore
/// let input = aoc_utils::read_input(1);
/// ```
pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Read input and split into lines
///
/// # Arguments
/// * `day` - Day number (1-25)
///
/// # Returns
/// Vector of strings, one per line
pub fn read_lines(day: u8) -> Vec<String> {
    read_input(day).lines().map(|s| s.to_string()).collect()
}

/// Parse input lines into a specific type
///
/// # Arguments
/// * `day` - Day number (1-25)
///
/// # Returns
/// Vector of parsed values
///
/// # Panics
/// Panics if any line cannot be parsed
pub fn parse_lines<T>(day: u8) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_lines(day)
        .iter()
        .map(|s| s.parse::<T>().expect("Failed to parse line"))
        .collect()
}

/// Read input as a single string, trimming whitespace
pub fn read_input_trimmed(day: u8) -> String {
    read_input(day).trim().to_string()
}

/// Split input by blank lines (useful for grouped data)
pub fn read_groups(day: u8) -> Vec<String> {
    read_input(day)
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}

/// Parse input as comma-separated values
pub fn read_csv<T>(day: u8) -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read_input_trimmed(day)
        .split(',')
        .map(|s| s.trim().parse::<T>().expect("Failed to parse CSV value"))
        .collect()
}

/// Read input as a 2D grid of characters
pub fn read_char_grid(day: u8) -> Vec<Vec<char>> {
    read_lines(day)
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

/// Read input as a 2D grid of digits (0-9)
pub fn read_digit_grid(day: u8) -> Vec<Vec<u8>> {
    read_lines(day)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_numbers() {
        let numbers: Vec<i32> = ["1", "2", "3"].iter().map(|s| s.parse().unwrap()).collect();
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_char_grid() {
        let lines = ["abc".to_string(), "def".to_string()];
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        assert_eq!(grid, vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
    }
}
