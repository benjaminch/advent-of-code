use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    writeln!(io::stdout(), "Part 1:",)?;
    execute(get_instructions(&mut input.clone()), 1);

    // Part 2
    writeln!(io::stdout(), "Part 2:",)?;
    execute(get_instructions(&mut input.clone()), 5);

    return Ok(());
}

fn get_instructions(input: &mut String) -> Vec<i32> {
    input.retain(|c| !c.is_whitespace());
    return input
        .split(",")
        .flat_map(|e| e.parse::<i32>())
        .collect::<Vec<i32>>();
}

fn execute(data: Vec<i32>, user_input: i32) -> Vec<i32> {
    let mut instructions: Vec<i32> = data.clone();
    let instructions_len: usize = instructions.len();
    let mut index: usize = 0;

    while index < instructions_len {
        let (operator, inputs_count, mut inputs_modes): (Operation, usize, Vec<ParameterMode>) =
            get_operation(*instructions.get(index).unwrap() as i32).unwrap();
        let raw_instruction: Vec<i32> = instructions[index..(index + inputs_count)].to_vec();
        let instruction: Instruction = Instruction {
            operator: operator,
            input_1_index: *raw_instruction.get(1).unwrap_or_else(|| &0),
            input_1_parameter_mode: inputs_modes
                .pop()
                .unwrap_or_else(|| ParameterMode::Position),
            input_2_index: *raw_instruction.get(2).unwrap_or_else(|| &0),
            input_2_parameter_mode: inputs_modes
                .pop()
                .unwrap_or_else(|| ParameterMode::Position),
            user_input: user_input,
            result_index: *raw_instruction.get(3).unwrap_or_else(|| &0) as usize,
        };
        let (should_stop_processing, next_index) =
            execute_instruction(instruction, &mut instructions);
        if should_stop_processing {
            break;
        }
        if next_index.is_none() {
            index += inputs_count;
        } else {
            index = next_index.unwrap() as usize;
        }
    }

    return instructions;
}

fn execute_instruction(instruction: Instruction, data: &mut Vec<i32>) -> (bool, Option<i32>) {
    let input_1: i32;
    let input_2: i32;

    match instruction.input_1_parameter_mode {
        ParameterMode::Position => {
            input_1 = data[instruction.input_1_index as usize];
        }
        ParameterMode::Immediate => {
            input_1 = instruction.input_1_index;
        }
    }

    match instruction.input_2_parameter_mode {
        ParameterMode::Position => {
            input_2 = data[instruction.input_2_index as usize];
        }
        ParameterMode::Immediate => {
            input_2 = instruction.input_2_index;
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
            data[instruction.input_1_index as usize] = instruction.user_input;
            (false, None)
        }
        Operation::Output => {
            writeln!(io::stdout(), "{}", data[instruction.input_1_index as usize]);
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
            data[instruction.result_index as usize] = (input_1 < input_2) as i32;
            (false, None)
        }
        Operation::Equals => {
            data[instruction.result_index as usize] = (input_1 == input_2) as i32;
            (false, None)
        }
        Operation::Stop => (true, None),
    };
}

fn get_operation(input: i32) -> Option<(Operation, usize, Vec<ParameterMode>)> {
    let picks: usize;
    let operation: Operation;
    let mut computed_input: i32 = input;

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
        99 => {
            operation = Operation::Stop;
            picks = 0;
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

fn get_parameter_mode(input: i32) -> Option<ParameterMode> {
    match input {
        0 => Some(ParameterMode::Position),
        1 => Some(ParameterMode::Immediate),
        _ => None,
    }
}

#[derive(Debug)]
enum Operation {
    Add = 0,
    Multiply = 1,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Stop = 99,
}

#[derive(Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

#[derive(Debug)]
struct Instruction {
    operator: Operation,
    input_1_index: i32,
    input_1_parameter_mode: ParameterMode,
    input_2_index: i32,
    input_2_parameter_mode: ParameterMode,
    user_input: i32,
    result_index: usize,
}
