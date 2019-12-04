use std::io::{self, Error, Read, Write};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let passwords: Vec<u32> = compute_passwords(lines.next().unwrap().to_string());

    // Part 1
    let matching_passwords: u32 = passwords.iter()
        .filter(|p| password_matches(**p, false))
        .count() as u32;
    writeln!(io::stdout(), "Matching passwords: {}", matching_passwords)?;

    // Part 2
    let matching_passwords2: u32 = passwords.iter()
        .filter(|p| password_matches(**p, true))
        .count() as u32;
    writeln!(io::stdout(), "Matching passwords (part 2): {}", matching_passwords2)?;

    return Ok(());
}

fn compute_passwords(input: String) -> Vec<u32> {
    let input_split: Vec<&str> = input.split("-").collect::<Vec<&str>>();
    let min: u32 = input_split[0].parse::<u32>().unwrap();
    let max: u32 = input_split[1].parse::<u32>().unwrap();
    let mut passwords: Vec<u32> = Vec::new();
    for p in min..max+1 {
        passwords.push(p);
    }
    return passwords;
}

fn password_matches(input: u32, exactly_two_adjacent_digits: bool) -> bool {
    let password: Vec<u32> = input
        .to_string()
        .chars()
        .map(|e| e.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    // It is a six-digit number.
    if password.len() != 6 {
        return false;
    }
    let mut has_two_adjacent_digits: bool = false;
    let mut left_to_right_never_decreases: bool = true;
    let mut same = HashMap::<u32, usize>::new();
    for i in 1..password.len() {
        if password[i-1] == password[i] {
            has_two_adjacent_digits = true;
            *same.entry(password[i]).or_default() += 1;
        }
        if password[i-1] > password[i] {
            left_to_right_never_decreases = false;
        }
    }
    // Two adjacent digits are the same (like 22 in 122345).
    if !has_two_adjacent_digits {
        return false;
    }
    // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679)
    if !left_to_right_never_decreases {
        return false;
    }
    // Part 2
    // two adjacent matching digits are not part of a larger group of matching digits
    if exactly_two_adjacent_digits
        && same.iter().filter(|x| *x.1 == 1).count() == 0 {
        return false;
    }

    return true;
}
