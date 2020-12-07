use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    const SUM: i32 = 2020;
    let numbers: Vec<i32> = get_numbers_list_from_str(&input);

    // Part 1
    match find_two_numbers_summing_to(&numbers, SUM) {
        Some((a, b)) => {
            writeln!(io::stdout(), "Part - 1 / Result: {} x {} = {}", a, b, a * b)?;
        }
        None => eprintln!("Part - 1 / No number found for sum = {}", SUM),
    };

    // Part 2
    match find_three_numbers_summing_to(&numbers, SUM) {
        Some((a, b, c)) => {
            writeln!(
                io::stdout(),
                "Part - 2 / Result: {} x {} x {} = {}",
                a,
                b,
                c,
                a * b * c
            )?;
        }
        None => eprintln!("Part - 2 / No number found for sum = {}", SUM),
    };

    Ok(())
}

fn get_numbers_list_from_str(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|e| e.trim().parse::<i32>())
        .collect::<Vec<i32>>()
}

fn find_two_numbers_summing_to(numbers: &Vec<i32>, target_sum: i32) -> Option<(i32, i32)> {
    if !numbers.is_empty() {
        // Sort vec
        let mut sorted_numbers = numbers.clone();
        sorted_numbers.sort_unstable();

        let mut lower_bound_index: usize = 0;
        let mut higher_bound_index: usize = sorted_numbers.len() - 1;

        // Finding numbers suming to sum
        while lower_bound_index < higher_bound_index {
            let computed_sum: i32 =
                sorted_numbers[lower_bound_index] + sorted_numbers[higher_bound_index];

            if computed_sum == target_sum {
                return Some((
                    sorted_numbers[lower_bound_index],
                    sorted_numbers[higher_bound_index],
                ));
            }
            if computed_sum < target_sum {
                lower_bound_index += 1;
            }
            if computed_sum > target_sum {
                higher_bound_index -= 1;
            }
        }
    }

    None
}

fn find_three_numbers_summing_to(numbers: &Vec<i32>, target_sum: i32) -> Option<(i32, i32, i32)> {
    for n in numbers {
        let sum = target_sum - n;
        if let Some((a, b)) = find_two_numbers_summing_to(numbers, sum) {
            return Some((*n, a, b));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_list_from_str() {
        // Some tests parsing raw input into integer vector.

        // Setup:
        let raw_input = "1721\n979\n366\n299\n675\n1456";
        let expected = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = get_numbers_list_from_str(raw_input);

        // Verify:
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_numbers_list_from_str_trim() {
        // Some tests parsing raw input into integer vector.

        // Setup:
        let raw_input = " 1721 \n979 \n    366\n 299 \n 675\n  1456";
        let expected = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = get_numbers_list_from_str(raw_input);

        // Verify:
        assert_eq!(expected, result);
    }

    #[test]
    fn test_find_three_numbers_summing_to() {
        // Test for 2020/01/part-2
        // Example case
        // https://adventofcode.com/2020/day/1#part2

        // Setup:
        const SUM: i32 = 2020;
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = find_three_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(Some((979, 366, 675)), result);
    }

    #[test]
    fn test_find_three_numbers_summing_to_empty_numbers() {
        // Test for 2020/01/part-2
        // Edge case, empty numbers
        // https://adventofcode.com/2020/day/1#part2

        // Setup:
        const SUM: i32 = 2020;
        let numbers = Vec::new();

        // Execute:
        let result = find_three_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_three_numbers_summing_to_unknown_sum() {
        // Test for 2020/01/part-2
        // Edge case, not found sum
        // https://adventofcode.com/2020/day/1#part2

        // Setup:
        const SUM: i32 = 202000;
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = find_three_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_two_numbers_summing_to() {
        // Test for 2020/01/part-1
        // Example case
        // https://adventofcode.com/2020/day/1#part1

        // Setup:
        const SUM: i32 = 2020;
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = find_two_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(Some((299, 1721)), result);
    }

    #[test]
    fn test_find_two_numbers_summing_to_empty_numbers() {
        // Test for 2020/01/part-1
        // Edge case, empty numbers
        // https://adventofcode.com/2020/day/1#part1

        // Setup:
        const SUM: i32 = 2020;
        let numbers = Vec::new();

        // Execute:
        let result = find_two_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(None, result);
    }

    #[test]
    fn test_find_two_numbers_summing_to_unknown_sum() {
        // Test for 2020/01/part-1
        // Edge case, not found sum
        // https://adventofcode.com/2020/day/1#part1

        // Setup:
        const SUM: i32 = 20245000;
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        // Execute:
        let result = find_two_numbers_summing_to(&numbers, SUM);

        // Verify:
        assert_eq!(None, result);
    }
}
