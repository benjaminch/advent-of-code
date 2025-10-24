use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<usize> = get_numbers_list_from_str(&input);

    let digit_length = 12;

    // Part 1
    let gamma_rate = get_gamma_rate(numbers.clone(), digit_length);
    let epsilon_rate = get_epsilon_rate(numbers.clone(), digit_length);
    writeln!(
        io::stdout(),
        "Part 1 / Gamma rate: {} Epsilon rate: {}, Result: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    )?;

    // Part 2
    let oxygen_generator_rating = get_oxygen_generator_rating(input.as_str(), digit_length);
    let co2_scrubber_rating = get_co2_scrubber_rating(input.as_str(), digit_length);
    writeln!(
        io::stdout(),
        "Part 2 / Oxygen generator rating: {} CO2 scrubber rating: {}, Result: {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    )?;

    Ok(())
}

fn get_numbers_list_from_str(input: &str) -> Vec<usize> {
    input
        .replace('\n', "")
        .chars()
        .flat_map(|e| e.to_string().parse::<usize>())
        .collect::<Vec<usize>>()
}

#[derive(Clone)]
enum MaskType {
    MostCommonInColumns,
    LeastCommonInColumns,
}

fn get_rate(input: Vec<usize>, digit_length: usize, mask_type: MaskType) -> usize {
    usize::from_str_radix(
        get_mask(input.clone(), digit_length, mask_type)
            .iter()
            .map(|&e| {
                if e {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
        2,
    )
    .unwrap()
}

fn get_gamma_rate(input: Vec<usize>, digit_length: usize) -> usize {
    get_rate(input, digit_length, MaskType::MostCommonInColumns)
}

fn get_epsilon_rate(input: Vec<usize>, digit_length: usize) -> usize {
    get_rate(input, digit_length, MaskType::LeastCommonInColumns)
}

fn get_rating(input: &str, digit_length: usize, mask_type: MaskType) -> usize {
    let mut index = 0;
    let mut filtered: Vec<&str> = input.lines().collect::<Vec<&str>>();

    while filtered.len() > 1 {
        let most_common_per_columns = get_mask(
            filtered
                .iter()
                .flat_map(|&e| e.chars().map(|c| if c.to_string() == "1" { 1 } else { 0 }))
                .collect(),
            digit_length,
            mask_type.clone(),
        );

        filtered.retain(|e| {
            e.chars().nth(index).unwrap().to_string()
                == if most_common_per_columns[index] {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
        });
        index += 1;
    }

    usize::from_str_radix(filtered.first().unwrap(), 2).unwrap()
}

fn get_oxygen_generator_rating(input: &str, digit_length: usize) -> usize {
    get_rating(input, digit_length, MaskType::MostCommonInColumns)
}

fn get_co2_scrubber_rating(input: &str, digit_length: usize) -> usize {
    get_rating(input, digit_length, MaskType::LeastCommonInColumns)
}

fn get_mask(input: Vec<usize>, digit_length: usize, mask_type: MaskType) -> Vec<bool> {
    let mut computed: Vec<i32> = vec![0; digit_length];

    for (i, &b) in input.iter().enumerate() {
        computed[i % digit_length] += if b == 1 { 1 } else { -1 };
    }

    computed
        .iter()
        .map(|&e| match mask_type {
            MaskType::MostCommonInColumns => e >= 0,
            MaskType::LeastCommonInColumns => e < 0,
        })
        .collect::<Vec<bool>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_numbers_list_from_str() {
        // setup:
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        // execute:
        let result = get_numbers_list_from_str(input);

        // verify:
        assert_eq!(
            vec![
                0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1,
                1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0,
                1, 0, 1, 0
            ],
            result
        );
    }

    #[test]
    fn test_get_gamma_rate() {
        // setup:
        let input = vec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0,
        ];

        // execute:
        let result = get_gamma_rate(input, 5);

        // verify:
        assert_eq!(22, result);
    }

    #[test]
    fn test_get_epsilon_rate() {
        // setup:
        let input = vec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0,
        ];

        // execute:
        let result = get_epsilon_rate(input, 5);

        // verify:
        assert_eq!(9, result);
    }

    #[test]
    fn test_get_oxygen_generator_rating() {
        // setup:
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        // execute:
        let result = get_oxygen_generator_rating(input, 5);

        // verify:
        assert_eq!(23, result);
    }

    #[test]
    fn test_get_co2_scrubber_rating() {
        // setup:
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        // execute:
        let result = get_co2_scrubber_rating(input, 5);

        // verify:
        assert_eq!(10, result);
    }
}
