use itertools::Itertools;

use std::collections::VecDeque;

use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    let (_instructions, output, _is_terminated, _index) = execute(get_instructions(&mut input.clone()), &mut VecDeque::new(), 1, None);
    writeln!(
        io::stdout(),
        "Part 1: OUTPUT = {:?}",
        output)?;

    // Part 2

    return Ok(());
}

fn get_instructions(input: &mut String) -> Vec<i64> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(",")
        .flat_map(|e| e.parse::<i64>())
        .collect::<Vec<i64>>();
}

fn execute(
    data: Vec<i64>,
    user_inputs: &mut VecDeque<i64>,
    default_input: i64,
    code_pointer: Option<usize>,
) -> (Vec<i64>, Vec<i64>, bool, usize) {
    let mut instructions: Vec<i64> = data.clone();
    let instructions_len: usize = instructions.len();
    let mut output: Vec<i64> = Vec::new();
    let mut is_terminated = false;
    let mut index: usize = code_pointer.unwrap_or_else(|| 0);
    let mut relative_base: i64 = 0;

    // make memory way bigger more than program (x10)
    instructions.append(&mut vec![0; instructions_len * 10]);

    while index < instructions_len {
        let (operator, inputs_count, mut inputs_modes): (Operation, usize, Vec<ParameterMode>) =
            get_operation(*instructions.get(index).unwrap() as i64).unwrap();
        let raw_instruction: Vec<i64> = instructions[index..(index + inputs_count)].to_vec();
        let mut instruction: Instruction = Instruction {
            operator: operator,
            input_1_index: *raw_instruction.get(1).unwrap_or_else(|| &0),
            input_1_parameter_mode: inputs_modes
                .pop()
                .unwrap_or_else(|| ParameterMode::Position),
            input_2_index: *raw_instruction.get(2).unwrap_or_else(|| &0),
            input_2_parameter_mode: inputs_modes
                .pop()
                .unwrap_or_else(|| ParameterMode::Position),
            user_input: user_inputs.pop_front().unwrap_or_else(|| default_input),
            result_index: *raw_instruction.get(3).unwrap_or_else(|| &0) as usize,
            output: None,
        };
        let (should_stop_processing, next_index) =
            execute_instruction(&mut instruction, &mut relative_base, &mut instructions);
        if next_index.is_none() {
            index += inputs_count;
        } else {
            index = next_index.unwrap() as usize;
        }
        if let Some(out) = instruction.output {
            output.push(out);
        }
        if should_stop_processing {
            is_terminated = true;
            break;
        }
    }

    return (instructions, output, is_terminated, index);
}

fn execute_instruction(instruction: &mut Instruction, relative_base: &mut i64, data: &mut Vec<i64>) -> (bool, Option<i64>) {
    let input_1: i64;
    let input_2: i64;

    match instruction.input_1_parameter_mode {
        ParameterMode::Position => {
            input_1 = data[instruction.input_1_index as usize];
        }
        ParameterMode::Immediate => {
            input_1 = instruction.input_1_index;
        }
        ParameterMode::Relative => {
            input_1 = data[(*relative_base + instruction.input_1_index) as usize];
        }
    }

    match instruction.input_2_parameter_mode {
        ParameterMode::Position => {
            input_2 = data[instruction.input_2_index as usize];
        }
        ParameterMode::Immediate => {
            input_2 = instruction.input_2_index;
        }
        ParameterMode::Relative => {
            input_2 = data[(*relative_base + instruction.input_2_index) as usize];
        }
    }

    return match instruction.operator {
        Operation::Add => {
            data[instruction.result_index as usize] = input_1 + input_2;
            (false, None)
        }
        Operation::Multiply => {
            data[instruction.result_index as usize] = input_1 * input_2;
            (false, None)
        }
        Operation::Input => {
            data[input_1 as usize] = instruction.user_input;
            (false, None)
        }
        Operation::Output => {
            instruction.output = Some(input_1);
            (false, None)
        }
        Operation::JumpIfTrue => {
            if input_1 != 0 {
                (false, Some(input_2))
            } else {
                (false, None)
            }
        }
        Operation::JumpIfFalse => {
            if input_1 == 0 {
                (false, Some(input_2))
            } else {
                (false, None)
            }
        }
        Operation::LessThan => {
            data[instruction.result_index as usize] = (input_1 < input_2) as i64;
            (false, None)
        }
        Operation::Equals => {
            data[instruction.result_index as usize] = (input_1 == input_2) as i64;
            (false, None)
        }
        Operation::AdjustRelativeBase => {
            let new_relative_base: i64 = input_1;
            *relative_base += new_relative_base;
            (false, None)
        }
        Operation::Stop => (true, None),
    };
}

fn get_operation(input: i64) -> Option<(Operation, usize, Vec<ParameterMode>)> {
    let picks: usize;
    let operation: Operation;
    let mut computed_input: i64 = input;

    // instruction
    match computed_input % 100 {
        1 => {
            operation = Operation::Add;
            picks = 4;
        }
        2 => {
            operation = Operation::Multiply;
            picks = 4;
        }
        3 => {
            operation = Operation::Input;
            picks = 2;
        }
        4 => {
            operation = Operation::Output;
            picks = 2;
        }
        5 => {
            operation = Operation::JumpIfTrue;
            picks = 3;
        }
        6 => {
            operation = Operation::JumpIfFalse;
            picks = 3;
        }
        7 => {
            operation = Operation::LessThan;
            picks = 4;
        }
        8 => {
            operation = Operation::Equals;
            picks = 4;
        }
        9 => {
            operation = Operation::AdjustRelativeBase;
            picks = 2;
        }
        99 => {
            operation = Operation::Stop;
            picks = 1;
        }
        _ => {
            return None;
        }
    }

    // params
    computed_input /= 100;
    let inputs_modes = vec![
        get_parameter_mode((computed_input / 10) % 10).unwrap(),
        get_parameter_mode(computed_input % 10).unwrap(),
    ];

    return Some((operation, picks, inputs_modes));
}

fn get_parameter_mode(input: i64) -> Option<ParameterMode> {
    match input {
        0 => Some(ParameterMode::Position),
        1 => Some(ParameterMode::Immediate),
        2 => Some(ParameterMode::Relative),
        _ => None,
    }
}

#[derive(Debug)]
enum Operation {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelativeBase = 9,
    Stop = 99,
}

#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Debug)]
struct Instruction {
    operator: Operation,
    input_1_index: i64,
    input_1_parameter_mode: ParameterMode,
    input_2_index: i64,
    input_2_parameter_mode: ParameterMode,
    user_input: i64,
    result_index: usize,
    output: Option<i64>,
}
