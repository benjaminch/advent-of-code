use std::collections::HashSet;
use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let checksum = compute_checksum(&input);
    writeln!(io::stdout(), "Checksum: {}", checksum)?;

    // Part 2
    let packages = find_packages_id_having_n_diff(&input, 1);
    writeln!(io::stdout(), "Packages: {:?}", packages)?;

    Ok(())
}

fn compute_checksum(input: &str) -> i32 {
    let mut codes_having_letters_twice: i32 = 0;
    let mut codes_having_letters_tree_times: i32 = 0;

    for line in input.lines() {
        let mut letters_counter = [0; 128];
        for c in line.chars() {
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
}

#[derive(Debug)]
struct MatchingPackages {
    packages: HashSet<String>,
    common_letters: String,
    diff_count: u32,
}

fn find_packages_id_having_n_diff(input: &str, diff_count: u32) -> Vec<MatchingPackages> {
    let mut result = Vec::new();
    let mut seen_packages: HashSet<String> = HashSet::new();

    for line in input.lines() {
        let package_id = line.clone();
        if !seen_packages.is_empty() {
            // compare current string with everyone already seen
            for seen in seen_packages.clone() {
                let (diffs, common_letters) =
                    get_letters_diff(package_id.to_string(), seen.clone());
                if diffs == diff_count {
                    result.push(MatchingPackages {
                        packages: [seen, package_id.to_string()].iter().cloned().collect(),
                        diff_count: diffs,
                        common_letters,
                    });
                }
            }
        }
        seen_packages.insert(package_id.to_string());
    }

    result
}

fn get_letters_diff(w1: String, w2: String) -> (u32, String) {
    let mut diff = (w1.len() as i32 - w2.len() as i32).abs() as u32;
    let mut common_letters: Vec<String> = Vec::new();
    let w1_chars = &mut w1.chars();
    let w2_chars = &mut w2.chars();
    while let (Some(w1_char), Some(w2_char)) = (w1_chars.next(), w2_chars.next()) {
        if w1_char != w2_char {
            diff += 1;
        } else {
            common_letters.push(w1_char.to_string());
        }
    }
    (diff, common_letters.join(""))
}
