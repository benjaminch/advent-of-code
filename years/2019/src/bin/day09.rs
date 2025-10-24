use intcode::Vm;
use std::collections::VecDeque;

use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let mut vm_part_1 = Vm::new(
        get_instructions(&mut input.clone()),
        VecDeque::from(vec![1]),
    );
    vm_part_1.run(false);
    writeln!(io::stdout(), "Part 1: output {:?}", vm_part_1.outputs())?;

    // Part 2
    let mut vm_part_2 = Vm::new(get_instructions(&mut input), VecDeque::from(vec![2]));
    vm_part_2.run(false);
    writeln!(io::stdout(), "Part 2: output {:?}", vm_part_2.outputs())?;

    Ok(())
}

fn get_instructions(input: &mut String) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    input
        .split(',')
        .flat_map(|e| e.parse::<i64>())
        .collect::<Vec<i64>>()
}
