use itertools::Itertools;

use std::collections::VecDeque;

use std::io::{self, Error, Read, Write};

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Part 1
    writeln!(io::stdout(), "Max signal {}", max_signal(input.clone()))?;

    // Part 2
    writeln!(
        io::stdout(),
        "Max signal v2 {}",
        max_signal_v2(input.clone())
    )?;

    return Ok(());
}

fn max_signal(code: String) -> i64 {
    return (0..5)
        .permutations(5)
        .map(|amplifiers_phases| {
            return compute_signal(amplifiers_phases.clone(), code.clone());
        })
        .max()
        .unwrap();
}

fn compute_signal(amplifiers_phase: Vec<i64>, code: String) -> i64 {
    let mut input: i64 = 0;
    let data = get_instructions(&mut code.clone());
    for phase in amplifiers_phase {
        let (_d, output, _is_terminated, _code_pointer) = execute(
            data.clone(),
            &mut VecDeque::from(vec![phase, input]),
            input,
            None,
        );
        input = output.unwrap();
    }
    return input;
}

fn max_signal_v2(code: String) -> i64 {
    return (5..10)
        .permutations(5)
        .map(|amplifiers_phases| {
            return compute_signal_v2(amplifiers_phases.clone(), code.clone());
        })
        .max()
        .unwrap();
}

fn compute_signal_v2(amplifiers_phase: Vec<i64>, code: String) -> i64 {
    let _input: i64 = 0;
    let data = get_instructions(&mut code.clone());
    let mut is_last_loop: bool = false;

    let mut amplifiers_data: Vec<Vec<i64>> = Vec::new();
    let mut amplifiers_outputs: Vec<i64> = vec![0; amplifiers_phase.len()];
    let mut amplifiers_code_pointers: Vec<Option<usize>> = vec![None; amplifiers_phase.len()];
    let mut amplifiers_inputs: Vec<VecDeque<i64>> = vec![VecDeque::new(); amplifiers_phase.len()];

    for i in 0..amplifiers_phase.len() {
        amplifiers_data.push(data.clone());
        amplifiers_inputs[i].push_back(amplifiers_phase[i as usize]);
    }

    amplifiers_inputs[0].push_back(0);

    while !is_last_loop {
        let mut amplifier_number: usize = 0;
        for _phase in amplifiers_phase.clone() {
            let default_input: i64 = *amplifiers_inputs[amplifier_number]
                .back()
                .unwrap_or_else(|| &0);
            let _inputs_debug = amplifiers_inputs[amplifier_number].clone();
            let (d, output, is_terminated, code_pointer) = execute(
                amplifiers_data[amplifier_number].clone(),
                &mut amplifiers_inputs[amplifier_number],
                default_input,
                amplifiers_code_pointers[amplifier_number],
            );
            let next_amplifier_number = if amplifier_number < amplifiers_inputs.len() - 1 {
                amplifier_number + 1
            } else {
                0
            };
            // TODO: shame clean this mess
            // introduce a proper / clean amplifier context
            amplifiers_inputs[next_amplifier_number].push_back(output.unwrap_or_else(|| 0));
            amplifiers_outputs[amplifier_number] = output.unwrap_or_else(|| 0);
            amplifiers_code_pointers[amplifier_number] = Some(code_pointer);
            amplifiers_data[amplifier_number] = d;
            if is_terminated {
                is_last_loop = true;
                break;
            }
            amplifier_number += 1;
        }
    }
    return *amplifiers_outputs.last().unwrap();
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
) -> (Vec<i64>, Option<i64>, bool, usize) {
    let mut instructions: Vec<i64> = data.clone();
    let instructions_len: usize = instructions.len();
    let mut output: Option<i64> = None;
    let mut is_terminated = false;
    let mut index: usize = code_pointer.unwrap_or_else(|| 0);

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
            execute_instruction(&mut instruction, &mut instructions);
        if next_index.is_none() {
            index += inputs_count;
        } else {
            index = next_index.unwrap() as usize;
        }
        if instruction.output.is_some() {
            output = instruction.output;
            break;
        }
        if should_stop_processing {
            is_terminated = true;
            break;
        }
    }

    return (instructions, output, is_terminated, index);
}

fn execute_instruction(instruction: &mut Instruction, data: &mut Vec<i64>) -> (bool, Option<i64>) {
    let input_1: i64;
    let input_2: i64;

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
    input_1_index: i64,
    input_1_parameter_mode: ParameterMode,
    input_2_index: i64,
    input_2_parameter_mode: ParameterMode,
    user_input: i64,
    result_index: usize,
    output: Option<i64>,
}
