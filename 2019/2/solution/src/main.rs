use std::io::{self, Read, Write};
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut instructions: Vec<i32> = get_instructions(&mut input.clone());

    // Part 1
    instructions[1] = 12;
    instructions[2] = 2;
    let data: Vec<i32> = execute(instructions);
    writeln!(io::stdout(), "Part 1, index 0 value: {:?}", data[0]);

    // Part 2
    for noun in 0..100 {
        for verb in 0..100 {
            let mut instructions2: Vec<i32> = get_instructions(&mut input.clone());
            instructions2[1] = noun;
            instructions2[2] = verb;
            let data2: Vec<i32> = execute(instructions2);
            // println!("{} {} {}", noun, verb, data2[0]);
            if data2[0] == 19690720 {
                writeln!(
                    io::stdout(),
                    "Part 2 (100 * noun + verb): {:?}",
                    ((100 * noun) + verb)
                );
                break;
            }
        }
    }
}

fn get_instructions(input: &mut String) -> Vec<i32> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(",")
        .flat_map(|e| e.parse::<i32>())
        .collect::<Vec<i32>>();
}

fn execute(data: Vec<i32>) -> Vec<i32> {
    let mut instructions: Vec<i32> = data.clone();
    let step: usize = 4;
    let instructions_len: usize = instructions.len();
    let mut index: usize = 0;
    let mut should_stop_processing: bool = false;

    while should_stop_processing == false && index < instructions_len {
        let instruction: Instruction = Instruction {
            operator: get_operation(*instructions.get(index).unwrap() as i32).unwrap(),
            input_1_index: *instructions.get(index + 1).or_else(|| Some(&0)).unwrap() as usize,
            input_2_index: *instructions.get(index + 2).or_else(|| Some(&0)).unwrap() as usize,
            result_index: *instructions.get(index + 3).or_else(|| Some(&0)).unwrap() as usize,
        };
        index += step;
        should_stop_processing = execute_instruction(instruction, &mut instructions);
    }

    return instructions;
}

fn execute_instruction(instruction: Instruction, data: &mut Vec<i32>) -> bool {
    return match instruction.operator {
        Operation::Add => {
            data[instruction.result_index] =
                data[instruction.input_1_index] + data[instruction.input_2_index];
            false
        }
        Operation::Multiply => {
            data[instruction.result_index] =
                data[instruction.input_1_index] * data[instruction.input_2_index];
            false
        }
        Operation::Stop => true,
    };
}

fn get_operation(input: i32) -> Option<Operation> {
    match input {
        1 => Some(Operation::Add),
        2 => Some(Operation::Multiply),
        99 => Some(Operation::Stop),
        _ => None,
    }
}

#[derive(Debug)]
enum Operation {
    Add = 0,
    Multiply = 1,
    Stop = 99,
}

#[derive(Debug)]
struct Instruction {
    operator: Operation,
    input_1_index: usize,
    input_2_index: usize,
    result_index: usize,
}
