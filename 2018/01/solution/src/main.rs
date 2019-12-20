use std::collections::HashSet;
use std::io::{self, Error, Read, Write};
use std::process;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part #1: compute frequency
    match compute_frequency(&input) {
        Ok(frequency) => {
            writeln!(io::stdout(), "Frequency: {}", frequency)?;
        }
        Err(e) => {
            writeln!(io::stderr(), "Problem: {}", e)?;
            process::exit(1);
        }
    };

    // Part #2: find first frequency reached twice
    match find_first_frequency_reached_twice(&input) {
        Ok(frequency) => {
            writeln!(io::stdout(), "First frequency reached twice: {}", frequency)?;
        }
        Err(e) => {
            writeln!(io::stderr(), "Problem: {}", e)?;
            process::exit(1);
        }
    };

    Ok(())
}

fn compute_frequency(input: &str) -> Result<i32, std::io::Error> {
    let mut frequency: i32 = 0;
    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(frequency_change) => {
                frequency += frequency_change;
            }
            Err(e) => {
                eprintln!("error parsing frequency change `{}`: {}", line, e);
            }
        }
    }
    Ok(frequency)
}

fn find_first_frequency_reached_twice(input: &str) -> Result<i32, std::io::Error> {
    let mut frequency: i32 = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(frequency);
    loop {
        // TODO: Find a better way than opening multiple times the file
        // something like cycle over iterator
        for line in input.lines() {
            match line.parse::<i32>() {
                Ok(frequency_change) => {
                    frequency += frequency_change;
                    if frequencies.contains(&frequency) {
                        return Ok(frequency);
                    }
                    frequencies.insert(frequency);
                }
                Err(e) => {
                    eprintln!("error parsing frequency change `{}`: {}", line, e);
                }
            }
        }
    }
}
