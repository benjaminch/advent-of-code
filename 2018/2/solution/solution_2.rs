use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Part 1
    let checksum = compute_checksum(Path::new("../input.txt"));
    println!("Checksum: {}", checksum);
    
    // Part 2
    let packages_id = find_packages_id_having_n_diff(Path::new("../input.txt"), 1);
    println!("Package IDs: {:?}", packages_id);
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

fn find_packages_id_having_n_diff(filename: &Path, diff_count: u32) -> HashSet<&str> {
    let mut result = HashSet::new();
    match read_lines(filename) {
        Ok(lines) => {
            let mut seen_packages = HashSet::new();
            for line in lines {
                if let Ok(l) = line && (seen_packages.len() > 0) {
                    // compare current string with everyone already seen
                    for seen in seen_packages {
                        if count_letters_diff(l, seen) == diff_count {
                            result.insert(seen);
                            result.insert(l);
                        }
                    }
                    seen_packages.insert(l);
                }
            }
            return result; 
        },
        Err(_) => result,
    }
}

fn count_letters_diff(w1: &str, w2: &str) -> u32 {
    let mut diff = (w1.len() as i32 - w2.len() as i32).abs() as u32;
    let w1_chars = w1.chars();
    let w2_chars = w2.chars();
    for i in 0..(w1.len() - diff as usize) {
        if w1_chars.nth(i).unwrap() != w2_chars.nth(i).unwrap() {
            diff += 1;
        }
    }
    return diff;
}

fn read_lines(filename: &Path) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
