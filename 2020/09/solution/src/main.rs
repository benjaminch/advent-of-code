use std::io::{self, Error, Read, Write};

fn get_numbers_list_from_str(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|e| e.trim().parse::<i64>())
        .collect::<Vec<i64>>()
}

fn get_first_invalid_entry(data: &Vec<i64>, preambule_length: usize) -> Option<i64> {
    if preambule_length >= data.len() {
        return None;
    }

    let mut index: usize = preambule_length;

    while index < data.len() {
        if find_two_numbers_summing_to(&data[index - preambule_length..index], data[index])
            .is_none()
        {
            return Some(data[index]);
        }
        index += 1;
    }

    None
}

fn find_two_numbers_summing_to(numbers: &[i64], target_sum: i64) -> Option<(i64, i64)> {
    if !numbers.is_empty() {
        // Sort vec
        let mut sorted_numbers = numbers.to_vec();
        sorted_numbers.sort_unstable();

        let mut lower_bound_index: usize = 0;
        let mut higher_bound_index: usize = sorted_numbers.len() - 1;

        // Finding numbers suming to sum
        while lower_bound_index < higher_bound_index {
            let computed_sum: i64 =
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

fn get_subset_summing_to(numbers: Vec<i64>, target_sum: i64) -> Option<Vec<i64>> {
    let mut from_index: usize = 0;
    let mut to_index: usize = 1;
    let mut current_sum: i64 = 0;

    while from_index < numbers.len() {
        current_sum = numbers[from_index..to_index + 1].iter().sum();

        if current_sum == target_sum {
            return Some(numbers[from_index..to_index + 1].to_vec());
        } else if current_sum < target_sum {
            to_index += 1;
        } else {
            from_index += 1;
            to_index = from_index + 1;
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    const PREAMBULE_SIZE: usize = 25;
    let part_1_first_invalid_entry =
        get_first_invalid_entry(&get_numbers_list_from_str(&input), PREAMBULE_SIZE);
    writeln!(
        io::stdout(),
        "Part - 1 / First invalid entry = {:?}",
        part_1_first_invalid_entry
    )?;

    // Part 2
    let part_2_subset_sum = get_subset_summing_to(
        get_numbers_list_from_str(&input),
        part_1_first_invalid_entry.unwrap(),
    )
    .unwrap();
    let part_2_subset_min_value = part_2_subset_sum.iter().min().unwrap();
    let part_2_subset_max_value = part_2_subset_sum.iter().max().unwrap();
    writeln!(
        io::stdout(),
        "Part - 2 / Result: {} + {} = {}",
        part_2_subset_min_value,
        part_2_subset_max_value,
        part_2_subset_min_value + part_2_subset_max_value
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_invalid_entry() {
        // Setup:
        const PREAMBULE_LENGTH: usize = 5;
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        // Execute:
        let result = get_first_invalid_entry(&input, PREAMBULE_LENGTH);

        // Verify:
        assert_eq!(Some(127), result);
    }

    #[test]
    fn test_get_subset_summing_to() {
        // Setup:
        const number_target: i64 = 127;
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        // Execute:
        let result = get_subset_summing_to(input, number_target);

        // Verify:
        assert_eq!(Some(vec![15, 25, 47, 40]), result);
    }
}
