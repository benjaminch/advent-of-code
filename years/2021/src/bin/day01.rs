use std::io::{self, Error, Read, Write};


fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<i32> = get_numbers_list_from_str(&input);

    // Part 1
    writeln!(io::stdout(), "Part 1 / Increased count: {}", get_variations(numbers.clone()).iter().filter(|&e| *e == DepthVariation::Increase).count())?;
    
    // Part 2
    writeln!(io::stdout(), "Part 2 / Increased count: {}", get_variations_windows(numbers.clone(), 3).iter().filter(|&e| *e == DepthVariation::Increase).count())?;

    Ok(())
}

#[derive(Debug, PartialEq)]
enum DepthVariation {
    NA,
    Increase,
    Decrease,
    Stable,
}

fn get_numbers_list_from_str(input: &str) -> Vec<i32> {
    input
        .lines()
        .flat_map(|e| e.trim().parse::<i32>())
        .collect::<Vec<i32>>()
}


fn get_variations(numbers: Vec<i32>) -> Vec<DepthVariation> {
    let mut previous_depth: Option<i32> = None;
    let mut results: Vec<DepthVariation> = Vec::with_capacity(numbers.len());

    for &depth in numbers.iter() {
        results.push(match previous_depth {
            None => DepthVariation::NA,
            Some(previous_depth) => if depth - previous_depth < 0 {
                DepthVariation::Decrease
            } else if depth - previous_depth > 0 {
                DepthVariation::Increase
            } else {
                DepthVariation::Stable
            },
        });
        
        previous_depth = Some(depth);
    }

    results
}

fn get_variations_windows(numbers: Vec<i32>, window_size: usize) -> Vec<DepthVariation> {
    get_variations(numbers.as_slice().windows(window_size).map(|g| g.iter().sum()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_variations() {
        // setup:
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        let expected = vec![
            DepthVariation::NA,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Decrease,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Decrease,
            DepthVariation::Increase,
        ];

        // execute:
       let result = get_variations(input.clone());
       
       // verify:
       assert_eq!(expected, result);
    }

    #[test]
    fn test_get_variations_windows() {
        // setup:
        let input = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];
        let expected = vec![
            DepthVariation::NA,
            DepthVariation::Increase,
            DepthVariation::Stable,
            DepthVariation::Decrease,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Increase,
            DepthVariation::Increase,
        ];

        // execute:
       let result = get_variations_windows(input.clone(), 3);
       
       // verify:
       assert_eq!(expected, result);
    }
}
