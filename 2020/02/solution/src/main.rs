use parse_display::FromStr;
use std::io::{self, Error, Read, Write};

#[derive(FromStr, PartialEq, Debug)]
#[display("{min_occurence}-{max_occurence} {item}: {password}")]
struct PasswordEntry {
    min_occurence: i32,
    max_occurence: i32,
    item: char,
    password: String,
}

impl PasswordEntry {
    fn is_valid_policy_1(&self) -> bool {
        let mut item_occurence: i32 = 0;
        for c in self.password.chars() {
            if c == self.item {
                item_occurence += 1;

                if item_occurence > self.max_occurence {
                    return false;
                }
            }
        }

        item_occurence >= self.min_occurence
    }

    fn is_valid_policy_2(&self) -> bool {
        let first_index: usize = self.min_occurence as usize - 1;
        let second_index: usize = self.max_occurence as usize - 1;

        let password_bytes = self.password.as_bytes();

        (password_bytes[first_index] as char == self.item
            && password_bytes[second_index] as char != self.item)
            || (password_bytes[first_index] as char != self.item
                && password_bytes[second_index] as char == self.item)
    }
}

fn get_passwords_list(input: &str) -> Vec<PasswordEntry> {
    input.lines().flat_map(|e| e.parse()).collect()
}

fn get_valid_passwords_list_policy_1(passwords: Vec<PasswordEntry>) -> Vec<PasswordEntry> {
    passwords
        .into_iter()
        .filter(|p| p.is_valid_policy_1())
        .collect()
}

fn get_valid_passwords_list_policy_2(passwords: Vec<PasswordEntry>) -> Vec<PasswordEntry> {
    passwords
        .into_iter()
        .filter(|p| p.is_valid_policy_2())
        .collect()
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let valid_passwords_policy_1 = get_valid_passwords_list_policy_1(get_passwords_list(&input));
    writeln!(
        io::stdout(),
        "Part - 1 / Valid passwords: {}",
        valid_passwords_policy_1.len()
    )?;

    // Part 2
    let valid_passwords_policy_2 = get_valid_passwords_list_policy_2(get_passwords_list(&input));
    writeln!(
        io::stdout(),
        "Part - 2 / Valid passwords: {}",
        valid_passwords_policy_2.len()
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_entry_parsing() {
        // Test for 2020/02
        // Parsing password entry input nominal case
        // https://adventofcode.com/2020/day/2

        // Setup:
        let input: &str = "12-16 x: zrrxjxvvrcnkwklddm";

        // Execute:
        let password_entry: PasswordEntry = input.parse().unwrap();

        // Verify:
        assert_eq!(
            PasswordEntry {
                min_occurence: 12,
                max_occurence: 16,
                item: 'x',
                password: String::from("zrrxjxvvrcnkwklddm")
            },
            password_entry
        );
    }

    #[test]
    fn test_password_entry_is_valid_policy_1_valid() {
        // Test for 2020/02
        // Checking password entry validity for v1 (entry is valid)
        // https://adventofcode.com/2020/day/part-1

        // Setup:
        let entries: Vec<PasswordEntry> = vec![
            "1-3 a: abcde".parse().unwrap(),
            "2-9 c: ccccccccc".parse().unwrap(),
        ];

        // Execute & Verify:
        for entry in entries {
            assert_eq!(true, entry.is_valid_policy_1());
        }
    }

    #[test]
    fn test_password_entry_is_valid_policy_1_invalid() {
        // Test for 2020/02
        // Checking password entry validity for v1 (entry is valid)
        // https://adventofcode.com/2020/day/part-1

        // Setup:
        let entries: Vec<PasswordEntry> = vec!["1-3 b: cdefg".parse().unwrap()];

        // Execute & Verify:
        for entry in entries {
            assert_eq!(false, entry.is_valid_policy_1());
        }
    }

    #[test]
    fn test_password_entry_is_valid_policy_2_valid() {
        // Test for 2020/02
        // Checking password entry validity for v2 (entry is valid)
        // https://adventofcode.com/2020/day/part-2

        // Setup:
        let entries: Vec<PasswordEntry> = vec!["1-3 a: abcde".parse().unwrap()];

        // Execute & Verify:
        for entry in entries {
            assert_eq!(true, entry.is_valid_policy_2());
        }
    }

    #[test]
    fn test_password_entry_is_valid_policy_2_invalid() {
        // Test for 2020/02
        // Checking password entry validity for v2 (entry is valid)
        // https://adventofcode.com/2020/day/part-2

        // Setup:
        let entries: Vec<PasswordEntry> = vec![
            "1-3 b: cdefg".parse().unwrap(),
            "2-9 c: ccccccccc".parse().unwrap(),
        ];

        // Execute & Verify:
        for entry in entries {
            "2-9 c: ccccccccc".parse::<PasswordEntry>().unwrap();
            assert_eq!(false, entry.is_valid_policy_2());
        }
    }
}
