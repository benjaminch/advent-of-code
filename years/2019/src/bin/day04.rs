use std::collections::HashMap;
use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let bounds: Vec<u32> = extract_bounds(lines.next().unwrap().to_string());

    // Part 1
    let matching_passwords: u32 = find_matching_passwords(bounds[0], bounds[1], false).len() as u32;
    writeln!(io::stdout(), "Matching passwords: {}", matching_passwords)?;

    // Part 2
    let matching_passwords2: u32 = find_matching_passwords(bounds[0], bounds[1], true).len() as u32;
    writeln!(
        io::stdout(),
        "Matching passwords (part 2): {}",
        matching_passwords2
    )?;

    Ok(())
}

fn find_matching_passwords(from: u32, to: u32, exactly_two_adjacent_digits: bool) -> Vec<u32> {
    let mut passwords: Vec<u32> = Vec::new();
    let mut current: u32 = from;
    while current <= to {
        let mut from_digits: Vec<u8> = to_digits(current);
        let mut digit_to_set: Option<u8> = None;

        // Initialize making digits from left to right increasing
        for i in 0..from_digits.len() - 1 {
            if from_digits[i] > from_digits[i + 1] && digit_to_set.is_none() {
                digit_to_set = Some(from_digits[i]);
            }
            if let Some(digit) = digit_to_set {
                from_digits[i + 1] = digit;
            }
        }
        current = to_number(&from_digits);
        if current > to {
            break;
        }

        let mut has_two_adjacent_digits: bool = false;
        let mut same = HashMap::<u8, usize>::new();
        for i in 1..from_digits.len() {
            if from_digits[i - 1] == from_digits[i] {
                has_two_adjacent_digits = true;
                *same.entry(from_digits[i - 1]).or_default() += 1;
            }
        }

        if (!exactly_two_adjacent_digits && has_two_adjacent_digits)
            || (exactly_two_adjacent_digits && same.iter().filter(|x| *x.1 == 1).count() > 0)
        {
            passwords.push(current);
        }

        current += 1;
    }
    passwords
}

fn to_number(digits: &[u8]) -> u32 {
    digits
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u32>()
        .unwrap()
}

fn to_digits(number: u32) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .map(|e| e.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn extract_bounds(input: String) -> Vec<u32> {
    let input_split: Vec<&str> = input.split('-').collect::<Vec<&str>>();
    let min: u32 = input_split[0].parse::<u32>().unwrap();
    let max: u32 = input_split[1].parse::<u32>().unwrap();
    vec![min, max]
}
