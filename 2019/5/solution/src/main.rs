use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let user_input: i32 = 1;

    // Part 1 
    writeln!(
        io::stdout(),
        "Part 1:",
    )?;
    execute(get_instructions(&mut input.clone()), user_input);

    // Part 2

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
    let mut should_stop_processing: bool = false;

    while should_stop_processing == false && index < instructions_len {
        let (operator, inputs_count, mut inputs_modes): (Operation, usize, Vec<ParameterMode>)
            = get_operation(*instructions.get(index).unwrap() as i32).unwrap();
        let raw_instruction: Vec<i32> = instructions[index..(index + inputs_count)].to_vec();
        let instruction: Instruction = Instruction {
            operator: operator,
            input_1_index: *raw_instruction.get(index + 1).unwrap_or_else(|| &0) as usize,
            input_1_parameter_mode: inputs_modes.pop().unwrap_or_else(|| ParameterMode::Position),
            input_2_index: *raw_instruction.get(index + 2).unwrap_or_else(|| &0) as usize,
            input_2_parameter_mode: inputs_modes.pop().unwrap_or_else(|| ParameterMode::Position),
            user_input: user_input,
            result_index: *raw_instruction.get(index + 3).unwrap_or_else(|| &0) as usize,
        };
        index += inputs_count;
        should_stop_processing = execute_instruction(instruction, &mut instructions);
    }

    return instructions;
}

fn execute_instruction(instruction: Instruction, data: &mut Vec<i32>) -> bool {
    let input_1: i32;
    let input_2: i32;

    match instruction.input_1_parameter_mode {
        ParameterMode::Position => {
            input_1 = data[instruction.input_1_index]; 
        }
        ParameterMode::Immediate => {
            input_1 = instruction.input_1_index as i32; 
        }
    }

    match instruction.input_1_parameter_mode {
        ParameterMode::Position => {
            input_2 = data[instruction.input_2_index]; 
        }
        ParameterMode::Immediate => {
            input_2 = instruction.input_2_index as i32; 
        }
    }

    return match instruction.operator {
        Operation::Add => {
            data[instruction.result_index] = input_1 + input_2;
            false
        }
        Operation::Multiply => {
            data[instruction.result_index] = input_1 * input_2;
            false
        }
        Operation::Input => {
            data[instruction.input_1_index] = instruction.user_input;
            false
        }
        Operation::Output => {
            writeln!(io::stdout(), "{}", data[instruction.input_1_index]);
            false
        }
        Operation::Stop => true,
    };
}

fn get_operation(input: i32) -> Option<(Operation, usize, Vec<ParameterMode>)> {
    let mut inputs_modes: Vec<ParameterMode> = Vec::new();
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
    while computed_input as f32 > 0.0 {
        inputs_modes.push(get_parameter_mode(computed_input % 10).unwrap());
        computed_input /= 10;
    }

    println!("Input: {} Instruction: {:?} Modes {:?}", input, operation, inputs_modes);

    return Some((operation, picks, inputs_modes));
}

fn get_parameter_mode(input: i32) -> Option<ParameterMode> {
    match input {
        0 => Some(ParameterMode::Position),
        1 => Some(ParameterMode::Immediate),
        _ => None,
    }
}

fn to_digits(number: i32) -> Vec<u8> {
    format!("{:0>5}", number.to_string())
        .chars()
        .map(|e| e.to_string().parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
}

fn to_number(digits: Vec<u8>) -> u32 {
    digits
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u32>()
        .unwrap()
}

#[derive(Debug)]
enum Operation {
    Add = 0,
    Multiply = 1,
    Input = 3,
    Output = 4,
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
    input_1_index: usize,
    input_1_parameter_mode: ParameterMode,
    input_2_index: usize,
    input_2_parameter_mode: ParameterMode,
    user_input: i32,
    result_index: usize,
}
