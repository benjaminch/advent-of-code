use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Part 1
    let checksum = compute_checksum(Path::new("../input.txt"));
    println!("Checksum: {}", checksum);
    
    // Part 2
    let packages = find_packages_id_having_n_diff(Path::new("../input.txt"), 1);
    println!("Packages: {:?}", packages);
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

#[derive(Debug)]
struct MatchingPackages {
    packages: HashSet<String>,
    common_letters: String,
    diff_count: u32,
}

fn find_packages_id_having_n_diff(filename: &Path, diff_count: u32) -> Vec<MatchingPackages> {
    let mut result = Vec::new();
    match read_lines(filename) {
        Ok(lines) => {
            let mut seen_packages: HashSet<String> = HashSet::new();
            for line in lines {
                if let Ok(l) = line  {
                    let package_id = l.clone();
                    if seen_packages.len() > 0 {
                        // compare current string with everyone already seen
                        for seen in seen_packages.clone() {
                            let (diffs, common_letters) = get_letters_diff(package_id.to_string(), seen.clone());
                            if diffs == diff_count {
                                result.push(MatchingPackages{
                                    packages: [seen, package_id.to_string()].iter().cloned().collect(),
                                    diff_count: diffs,
                                    common_letters: common_letters,
                                });
                            }
                        }
                    }
                    seen_packages.insert(package_id.to_string());
                }
            }
            return result; 
        },
        Err(_) => result,
    }
}

fn get_letters_diff(w1: String, w2: String) -> (u32, String) {
    let mut diff = (w1.len() as i32 - w2.len() as i32).abs() as u32;
    let mut common_letters: Vec<String> = Vec::new();
    let w1_chars = &mut w1.chars();
    let w2_chars = &mut w2.chars();
    while let (Some(w1_char), Some(w2_char)) = (w1_chars.next(), w2_chars.next()) {
        if w1_char != w2_char {
            diff += 1;
        }
        else {
            common_letters.push(w1_char.to_string());
        }
    }
    return (diff, common_letters.join(""));
}

fn read_lines(filename: &Path) -> std::result::Result<std::io::Lines<std::io::BufReader<std::fs::File>>, std::io::Error> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
