use intcode::Vm;
use std::collections::VecDeque;
use std::io::{self, Error, Read, Write};
use std::vec::Vec;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let mut vm_part_1 = Vm::new(get_instructions(&mut input.clone(), 12, 2), VecDeque::new());
    vm_part_1.run(true);
    writeln!(
        io::stdout(),
        "Part 1, index 0 value: {:?}",
        vm_part_1.data()[0]
    )?;

    // Part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut vm_part_2 = Vm::new(
                get_instructions(&mut input.clone(), noun, verb),
                VecDeque::new(),
            );
            vm_part_2.run(true);
            if vm_part_2.data()[0] == 19_690_720 {
                writeln!(
                    io::stdout(),
                    "Part 2 (100 * noun + verb): {:?}",
                    ((100 * noun) + verb)
                )?;
                break;
            }
        }
    }

    Ok(())
}

fn get_instructions(input: &mut String, noun: i64, verb: i64) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    let mut instructions: Vec<i64> = input
        .split(',')
        .flat_map(|e| e.parse::<i64>())
        .collect::<Vec<i64>>();

    instructions[1] = noun;
    instructions[2] = verb;

    instructions
}
