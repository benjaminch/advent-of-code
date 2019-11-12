use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;
use std::process;

fn main() {
    // Part #1: compute frequency
    match compute_frequency(Path::new("../input.txt")) {
        Ok(frequency) => {
            println!("Frequency: {}", frequency);
        },
        Err(e) => {
            eprintln!("Problem: {}", e);
            process::exit(1);
        },
    };

    // Part #2: find first frequency reached twice
    match find_first_frequency_reached_twice(Path::new("../input.txt")) {
        Ok(frequency) => {
            println!("First frequency reached twice: {}", frequency);
        },
        Err(e) => {
            eprintln!("Problem: {}", e);
            process::exit(1);
        },
    };
}

fn compute_frequency(file: &Path) -> Result<i32, std::io::Error> {
    match read_lines(file) {
        Ok(lines) => {
            let mut frequency: i32 = 0;
            for line in lines {
                if let Ok(frequency_change_str) = line {
                    match frequency_change_str.parse::<i32>() {
                        Ok(frequency_change) => {
                            frequency += frequency_change;
                        },
                        Err(e) => {
                            eprintln!("Problem parsing frequency change `{}`: {}", frequency_change_str, e);
                        }
                    }
                }
            }
            Ok(frequency)
        },
        Err(e) => Err(e),
    }
}

fn find_first_frequency_reached_twice(file: &Path) -> Result<i32, std::io::Error> {
    match read_lines(file) {
        Ok(lines) => {
            let mut frequency: i32 = 0;
            let mut frequencies = HashSet::new();
            for line in lines {
                if let Ok(frequency_change_str) = line {
                    match frequency_change_str.parse::<i32>() {
                        Ok(frequency_change) => {
                            frequency += frequency_change;
                            if frequencies.contains(&frequency) {
                                return Ok(frequency);
                            }
                            frequencies.insert(frequency);
                        },
                        Err(e) => {
                            eprintln!("Problem parsing frequency change `{}`: {}", frequency_change_str, e);
                        }
                    }
                }
            }
            Err(Error::new(ErrorKind::Other, "doesn't seem to have any frequency appearing twice"))
        },
        Err(e) => Err(e),
    }
}

fn read_lines(filename: &Path) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
    let f = File::open(filename);
    match f {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => return Err(e),
    }
}
