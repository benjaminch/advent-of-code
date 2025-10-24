use std::io::{self, Error, Read, Write};

fn get_groups_anyone_yes_answers(input: &str) -> Vec<u32> {
    let mut answers: Vec<u32> = Vec::new();
    let a_index: u32 = 'a' as u32;
    let z_index: u32 = 'z' as u32;

    let mut group_answers: u32 = 0;

    for line in input.lines() {
        for c in line.chars() {
            let c_index: u32 = c as u32;
            if c_index >= a_index && c_index <= z_index {
                group_answers |= 1 << (c_index % (z_index - a_index + 1));
            }
        }

        if line.is_empty() {
            answers.push(group_answers);
            group_answers = 0;
        }
    }
    answers.push(group_answers);

    answers
}

// TODO: To be refactored using the part 1 method
fn get_groups_everyone_yes_answers(input: &str) -> Vec<u32> {
    let mut answers: Vec<u32> = Vec::new();
    let a_index: u32 = 'a' as u32;
    let z_index: u32 = 'z' as u32;

    let mut group_answers: u32 = u32::MAX;

    for line in input.lines() {
        if !line.is_empty() {
            let mut individual_answer: u32 = 0;
            for c in line.chars() {
                let c_index: u32 = c as u32;
                if c_index >= a_index && c_index <= z_index {
                    individual_answer |= 1 << (c_index % (z_index - a_index + 1));
                }
            }

            group_answers &= individual_answer;
        } else {
            answers.push(group_answers);
            group_answers = u32::MAX;
        }
    }
    answers.push(group_answers);

    answers
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let part_1_sum_of_yes: u32 = get_groups_anyone_yes_answers(&input)
        .iter()
        .map(|g| g.count_ones())
        .sum();

    writeln!(io::stdout(), "Part - 1 / Sum of yes: {}", part_1_sum_of_yes)?;

    // Part 2
    let part_2_sum_of_yes: u32 = get_groups_everyone_yes_answers(&input)
        .iter()
        .map(|g| g.count_ones())
        .sum();

    writeln!(io::stdout(), "Part - 2 / Sum of yes: {}", part_2_sum_of_yes)?;

    Ok(())
}

#[cfg(test)]
mod tests {}
