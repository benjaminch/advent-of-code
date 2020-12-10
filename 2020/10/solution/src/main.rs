use std::collections::HashMap;
use std::io::{self, Error, Read, Write};

#[derive(Debug)]
struct AdaptorsChain {
    adaptors: Vec<i64>,
    differences_count: HashMap<i64, i64>,
}

impl AdaptorsChain {
    fn new(adaptors: Vec<i64>) -> AdaptorsChain {
        let mut chain: AdaptorsChain = AdaptorsChain {
            adaptors: adaptors.to_vec(),
            differences_count: HashMap::new(),
        };

        for i in 0..=adaptors.len() - 2 {
            let differences = adaptors[i + 1_usize] - adaptors[i as usize];
            *chain.differences_count.entry(differences).or_insert(1) += 1;
        }

        chain
    }
}

fn get_adaptors_list_from_str(input: &str) -> Vec<i64> {
    input
        .lines()
        .flat_map(|e| e.trim().parse::<i64>())
        .collect::<Vec<i64>>()
}

fn get_adaptors_chain(adaptors: Vec<i64>) -> Vec<i64> {
    let mut unique_sorted_adaptors: Vec<i64> = adaptors;
    unique_sorted_adaptors.sort_unstable();
    unique_sorted_adaptors.dedup();
    unique_sorted_adaptors[..].to_vec()
}

fn get_permutations_count(adaptors: &[i64], lower_bound: i64, higher_bound: i64) -> u64 {
    let mut permutations = HashMap::new();

    // starts at 0, 1 available permutation
    permutations.insert(0, 1);

    for &adaptor in adaptors {
        permutations.insert(adaptor, {
            let mut acc: u64 = 0;

            for i in lower_bound..higher_bound + 1 {
                acc += permutations.get(&(adaptor - i)).unwrap_or(&0);
            }

            acc
        });
    }

    permutations[adaptors.last().unwrap()]
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let part_1_chain = AdaptorsChain::new(get_adaptors_chain(get_adaptors_list_from_str(&input)));
    writeln!(io::stdout(), "Part - 1 / Chain = {:?}", part_1_chain)?;

    // Part 2
    const MIN_VOLTAGE_DIFF: i64 = 1;
    const MAX_VOLTAGE_DIFF: i64 = 3;
    let part_2_available_permutations = get_permutations_count(
        &part_1_chain.adaptors[..],
        MIN_VOLTAGE_DIFF,
        MAX_VOLTAGE_DIFF,
    );
    writeln!(
        io::stdout(),
        "Part - 2 / Available permutations = {}",
        part_2_available_permutations
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_difference_distances_case_1() {
        // Setup:
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let chain: AdaptorsChain = AdaptorsChain::new(get_adaptors_chain(input));

        // Execute:
        let result = chain.differences_count;

        // Verify:
        assert_eq!([(1, 7), (3, 5)].iter().cloned().collect::<HashMap<i64, i64>>(), result);
    }

    #[test]
    fn test_count_difference_distances_case_2() {
        // Setup:
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let chain: AdaptorsChain = AdaptorsChain::new(get_adaptors_chain(input));

        // Execute:
        let result = chain.differences_count;

        // Verify:
        assert_eq!([(1, 22), (3, 10)].iter().cloned().collect::<HashMap<i64, i64>>(), result);
    }

    #[test]
    fn test_get_permutations_count_case_1() {
        // Setup:
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        const MIN_VOLTAGE_DIFF: i64 = 1;
        const MAX_VOLTAGE_DIFF: i64 = 3;
        let chain: AdaptorsChain = AdaptorsChain::new(get_adaptors_chain(input));

        // Execute:
        let result =
            get_permutations_count(&chain.adaptors[..], MIN_VOLTAGE_DIFF, MAX_VOLTAGE_DIFF);

        // Verify:
        assert_eq!(8, result);
    }

    #[test]
    fn test_get_permutations_count_case_2() {
        // Setup:
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        const MIN_VOLTAGE_DIFF: i64 = 1;
        const MAX_VOLTAGE_DIFF: i64 = 3;
        let chain: AdaptorsChain = AdaptorsChain::new(get_adaptors_chain(input));

        // Execute:
        let result =
            get_permutations_count(&chain.adaptors[..], MIN_VOLTAGE_DIFF, MAX_VOLTAGE_DIFF);

        // Verify:
        assert_eq!(19208, result);
    }
}
