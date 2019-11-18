use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Part 1
    let checksum = compute_checksum(Path::new("../input.txt"));
    println!("Checksum: {}", checksum);
}

fn compute_checksum(file: &Path) -> i32 {
    let mut codes_having_letters_twice: i32 = 0;
    let mut codes_having_letters_tree_times: i32 = 0;
    match read_lines(file) {
        Ok(lines) => {
            for line in lines {
                let mut letters_counter = [0; 128];
                for c in line.expect("line failed").chars() {
                    let ascii_code: usize = c as usize;
                    letters_counter[ascii_code] += 1
                }
                if letters_counter.iter().any(|e| e == &2) {
                    codes_having_letters_twice += 1;
                }
                if letters_counter.iter().any(|e| e == &3) {
                    codes_having_letters_tree_times += 1;
                }
            }
            codes_having_letters_tree_times * codes_having_letters_twice
        },
        Err(_) => 0, 
    }
}

fn read_lines(filename: &Path) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
