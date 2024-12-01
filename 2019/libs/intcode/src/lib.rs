use std::collections::VecDeque;

#[derive(Debug)]
pub struct Vm {
    data: Vec<i64>,
    code_pointer: usize,
    relative_base: i64,
    inputs: VecDeque<i64>,
    default_input: i64,
    outputs: Vec<i64>,
    state: State,
}

impl Vm {
    // Vm constructor
    pub fn new(data: Vec<i64>, inputs: VecDeque<i64>) -> Vm {
        let mut cloned_data = data.clone();
        // make memory way bigger more than program (x10)
        // TODO: dynamically allocate it if needs to be accessed
        cloned_data.append(&mut vec![0; data.len() * 10]);

        Vm {
            state: State::NotStarted,
            data: cloned_data,
            code_pointer: 0_usize,
            relative_base: 0,
            inputs,
            default_input: 0,
            outputs: Vec::new(),
        }
    }

    // Inputs setter
    // Allow to add an input to inputs queue.
    pub fn add_input(&mut self, input: i64) {
        self.inputs.push_front(input);
    }

    // Outputs getter
    // Allow to get outputs queue.
    pub fn outputs(&self) -> &[i64] {
        self.outputs.as_slice()
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    // Data getter
    // Allow to get VM data.
    pub fn data(self) -> Vec<i64> {
        self.data
    }

    // Reset Vm allows to reset the VM to its initial state but data memory.
    pub fn reset(&mut self) {
        self.code_pointer = 0_usize;
        self.state = State::NotStarted;
        self.inputs = VecDeque::from(vec![]);
        self.outputs = Vec::new();
    }

    // Get operation from raw input
    // TODO: create a struct `Operation` holding OperationType, number of inputs and
    // parameter modes
    fn get_operation(input: i64) -> Option<(Operation, usize, Vec<ParameterMode>)> {
        let picks: usize;
        let operation: Operation;
        let computed_input: i64 = input;

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
        let inputs_modes = vec![
            Vm::get_parameter_mode((computed_input / 10000) % 10).unwrap(),
            Vm::get_parameter_mode((computed_input / 1000) % 10).unwrap(),
            Vm::get_parameter_mode((computed_input / 100) % 10).unwrap(),
        ];

        Some((operation, picks, inputs_modes))
    }

    fn get_parameter_mode(input: i64) -> Option<ParameterMode> {
        match input {
            0 => Some(ParameterMode::Position),
            1 => Some(ParameterMode::Immediate),
            2 => Some(ParameterMode::Relative),
            _ => None,
        }
    }

    pub fn run(&mut self, stop_on_output: bool) {
        let instructions_len: usize = self.data.len();

        self.state = State::Running;

        while self.code_pointer < instructions_len {
            let (operator, inputs_count, mut inputs_modes): (Operation, usize, Vec<ParameterMode>) =
                Vm::get_operation(*self.data.get(self.code_pointer).unwrap()).unwrap();
            let raw_instruction: Vec<i64> =
                self.data[self.code_pointer..(self.code_pointer + inputs_count)].to_vec();
            let instruction: Instruction = Instruction {
                operator,
                input_1_index: *raw_instruction.get(1).unwrap_or(&0),
                input_1_parameter_mode: inputs_modes.pop().unwrap_or(ParameterMode::Position),
                input_2_index: *raw_instruction.get(2).unwrap_or(&0),
                input_2_parameter_mode: inputs_modes.pop().unwrap_or(ParameterMode::Position),
                input_3_index: *raw_instruction.get(3).unwrap_or(&0),
                input_3_parameter_mode: inputs_modes.pop().unwrap_or(ParameterMode::Position),
                code_pointer_increment: inputs_count,
            };

            let pre_instruction_outputs_len: usize = self.outputs.len();

            Vm::execute_instruction(self, instruction);

            if stop_on_output && self.outputs.len() > pre_instruction_outputs_len {
                break;
            }

            if self.state == State::Stopped {
                break;
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        // TODO: extract in a function
        let input_1_read: i64;
        let input_1_write: i64;
        match instruction.input_1_parameter_mode {
            ParameterMode::Position => {
                input_1_read = self.data[instruction.input_1_index as usize];
                input_1_write = instruction.input_1_index;
            }
            ParameterMode::Immediate => {
                input_1_read = instruction.input_1_index;
                input_1_write = instruction.input_1_index;
            }
            ParameterMode::Relative => {
                input_1_read = self.data[(self.relative_base + instruction.input_1_index) as usize];
                input_1_write = self.relative_base + instruction.input_1_index;
            }
        }

        let input_2_read: i64;
        let input_2_write: i64;
        match instruction.input_2_parameter_mode {
            ParameterMode::Position => {
                input_2_read = self.data[instruction.input_2_index as usize];
                input_2_write = instruction.input_2_index;
            }
            ParameterMode::Immediate => {
                input_2_read = instruction.input_2_index;
                input_2_write = instruction.input_2_index;
            }
            ParameterMode::Relative => {
                input_2_read = self.data[(self.relative_base + instruction.input_2_index) as usize];
                input_2_write = self.relative_base + instruction.input_2_index;
            }
        }

        let input_3_read: i64;
        let input_3_write: i64;
        match instruction.input_3_parameter_mode {
            ParameterMode::Relative => {
                input_3_read = self.data[(self.relative_base + instruction.input_3_index) as usize];
                input_3_write = self.relative_base + instruction.input_3_index;
            }
            _ => {
                input_3_read = self.data[instruction.input_3_index as usize];
                input_3_write = instruction.input_3_index;
            }
        }

        match instruction.operator {
            Operation::Add => {
                self.data[input_3_write as usize] = input_1_read + input_2_read;
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::Multiply => {
                self.data[input_3_write as usize] = input_1_read * input_2_read;
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::Input => {
                self.data[input_1_write as usize] = self.inputs.pop_back().unwrap_or(0);
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::Output => {
                self.outputs.push(input_1_read);
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::JumpIfTrue => {
                if input_1_read != 0 {
                    self.code_pointer = input_2_read as usize;
                } else {
                    self.code_pointer += instruction.code_pointer_increment;
                }
            }
            Operation::JumpIfFalse => {
                if input_1_read == 0 {
                    self.code_pointer = input_2_read as usize;
                } else {
                    self.code_pointer += instruction.code_pointer_increment;
                }
            }
            Operation::LessThan => {
                self.data[input_3_write as usize] = (input_1_read < input_2_read) as i64;
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::Equals => {
                self.data[input_3_write as usize] = (input_1_read == input_2_read) as i64;
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::AdjustRelativeBase => {
                let new_relative_base: i64 = input_1_read;
                self.relative_base += new_relative_base;
                self.code_pointer += instruction.code_pointer_increment;
            }
            Operation::Stop => {
                self.state = State::Stopped;
                self.code_pointer += instruction.code_pointer_increment;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    NotStarted = 0,
    Running = 1,
    Stopped = 2,
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
    input_3_index: i64,
    input_3_parameter_mode: ParameterMode,
    code_pointer_increment: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_position_mode() {
        // some jump tests that take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        // https://adventofcode.com/2019/day/5

        // Setup:
        let mut input_1: VecDeque<i64> = VecDeque::new();
        input_1.push_front(1);

        let mut vm: Vm = Vm::new(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            input_1,
        );

        // Execute:
        vm.run(true);

        // Verify:
        assert_eq!(vec![1], vm.outputs());
    }

    #[test]
    fn test_jump_immediate_mode() {
        // some jump tests that take an input,
        // then output 0 if the input was zero or 1 if the input was non-zero
        // https://adventofcode.com/2019/day/5

        // Setup:
        let mut input_1: VecDeque<i64> = VecDeque::new();
        input_1.push_front(1);

        let mut vm: Vm = Vm::new(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            input_1,
        );

        // Execute:
        vm.run(true);

        // Verify:
        assert_eq!(vec![1], vm.outputs());
    }

    #[test]
    fn test_copy_of_itself() {
        // Takes no input and produces a copy of itself as output
        // https://advddentofcode.com/2019/day/9

        // Setup:
        let mut vm: Vm = Vm::new(
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            VecDeque::new(),
        );

        // Execute:
        vm.run(false);

        // Verify:
        assert_eq!(
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
            vm.outputs()
        );
    }

    #[test]
    fn test_16_digits_number() {
        // Takes no input and produces a 16 digits number
        // https://advddentofcode.com/2019/day/9

        // Setup:
        let mut vm: Vm = Vm::new(
            vec![1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0],
            VecDeque::new(),
        );

        // Execute:
        vm.run(true);

        // Verify:
        assert_eq!(vec![1_219_070_632_396_864], vm.outputs());
    }

    #[test]
    fn test_large_middle_number() {
        // Takes no input and produce the large number in the middle
        // https://advddentofcode.com/2019/day/9

        // Setup:
        let mut vm: Vm = Vm::new(vec![104, 1_125_899_906_842_624, 99], VecDeque::new());

        // Execute:
        vm.run(true);

        // Verify:
        assert_eq!(vec![1_125_899_906_842_624], vm.outputs());
    }

    #[test]
    fn test_complete_via_test_mode() {
        // single input; run it in test mode by providing it the value 1. It will perform a
        // series of checks on each opcode, output any opcodes (and the associated parameter
        // modes) that seem to be functioning incorrectly, and finally output a BOOST keycode.
        // https://advddentofcode.com/2019/day/9

        // Setup:
        let mut input_1: VecDeque<i64> = VecDeque::new();
        input_1.push_front(1);

        let mut vm: Vm = Vm::new(
            vec![
                1102, 34_463_338, 34_463_338, 63, 1007, 63, 34_463_338, 63, 1005, 63, 53, 1101, 3,
                0, 1000, 109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63,
                1005, 63, 65, 1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4,
                25, 104, 0, 99, 4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1102, 1, 344, 1023,
                1101, 0, 0, 1020, 1101, 0, 481, 1024, 1102, 1, 1, 1021, 1101, 0, 24, 1005, 1101, 0,
                29, 1018, 1102, 39, 1, 1019, 1102, 313, 1, 1028, 1102, 1, 35, 1009, 1101, 28, 0,
                1001, 1101, 26, 0, 1013, 1101, 0, 351, 1022, 1101, 564, 0, 1027, 1102, 1, 32, 1011,
                1101, 23, 0, 1006, 1102, 1, 25, 1015, 1101, 21, 0, 1003, 1101, 0, 31, 1014, 1101,
                33, 0, 1004, 1102, 37, 1, 1000, 1102, 476, 1, 1025, 1101, 22, 0, 1007, 1102, 30, 1,
                1012, 1102, 1, 27, 1017, 1102, 1, 34, 1002, 1101, 38, 0, 1008, 1102, 1, 36, 1010,
                1102, 1, 20, 1016, 1102, 567, 1, 1026, 1102, 1, 304, 1029, 109, -6, 2108, 35, 8,
                63, 1005, 63, 201, 1001, 64, 1, 64, 1106, 0, 203, 4, 187, 1002, 64, 2, 64, 109, 28,
                21101, 40, 0, -9, 1008, 1013, 38, 63, 1005, 63, 227, 1001, 64, 1, 64, 1105, 1, 229,
                4, 209, 1002, 64, 2, 64, 109, -2, 1205, 1, 243, 4, 235, 1105, 1, 247, 1001, 64, 1,
                64, 1002, 64, 2, 64, 109, -12, 2102, 1, -5, 63, 1008, 63, 24, 63, 1005, 63, 271,
                1001, 64, 1, 64, 1105, 1, 273, 4, 253, 1002, 64, 2, 64, 109, 8, 2108, 22, -9, 63,
                1005, 63, 295, 4, 279, 1001, 64, 1, 64, 1106, 0, 295, 1002, 64, 2, 64, 109, 17,
                2106, 0, -5, 4, 301, 1001, 64, 1, 64, 1106, 0, 313, 1002, 64, 2, 64, 109, -21,
                21107, 41, 40, 7, 1005, 1019, 333, 1001, 64, 1, 64, 1105, 1, 335, 4, 319, 1002, 64,
                2, 64, 109, 1, 2105, 1, 10, 1001, 64, 1, 64, 1105, 1, 353, 4, 341, 1002, 64, 2, 64,
                109, 10, 1206, -3, 371, 4, 359, 1001, 64, 1, 64, 1105, 1, 371, 1002, 64, 2, 64,
                109, -5, 21108, 42, 42, -7, 1005, 1011, 393, 4, 377, 1001, 64, 1, 64, 1105, 1, 393,
                1002, 64, 2, 64, 109, -8, 2101, 0, -4, 63, 1008, 63, 23, 63, 1005, 63, 415, 4, 399,
                1105, 1, 419, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 21102, 43, 1, -6, 1008,
                1017, 43, 63, 1005, 63, 441, 4, 425, 1106, 0, 445, 1001, 64, 1, 64, 1002, 64, 2,
                64, 109, -21, 1207, 0, 33, 63, 1005, 63, 465, 1001, 64, 1, 64, 1106, 0, 467, 4,
                451, 1002, 64, 2, 64, 109, 19, 2105, 1, 3, 4, 473, 1106, 0, 485, 1001, 64, 1, 64,
                1002, 64, 2, 64, 109, 1, 21101, 44, 0, -7, 1008, 1015, 44, 63, 1005, 63, 511, 4,
                491, 1001, 64, 1, 64, 1106, 0, 511, 1002, 64, 2, 64, 109, 2, 1206, -3, 527, 1001,
                64, 1, 64, 1105, 1, 529, 4, 517, 1002, 64, 2, 64, 109, -8, 1201, -7, 0, 63, 1008,
                63, 35, 63, 1005, 63, 555, 4, 535, 1001, 64, 1, 64, 1105, 1, 555, 1002, 64, 2, 64,
                109, 1, 2106, 0, 10, 1105, 1, 573, 4, 561, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
                4, 21107, 45, 46, -7, 1005, 1014, 591, 4, 579, 1106, 0, 595, 1001, 64, 1, 64, 1002,
                64, 2, 64, 109, -12, 1208, -6, 21, 63, 1005, 63, 617, 4, 601, 1001, 64, 1, 64,
                1105, 1, 617, 1002, 64, 2, 64, 109, -11, 1208, 6, 31, 63, 1005, 63, 637, 1001, 64,
                1, 64, 1106, 0, 639, 4, 623, 1002, 64, 2, 64, 109, 16, 2101, 0, -7, 63, 1008, 63,
                20, 63, 1005, 63, 659, 1105, 1, 665, 4, 645, 1001, 64, 1, 64, 1002, 64, 2, 64, 109,
                3, 2102, 1, -9, 63, 1008, 63, 38, 63, 1005, 63, 691, 4, 671, 1001, 64, 1, 64, 1106,
                0, 691, 1002, 64, 2, 64, 109, 4, 1205, -1, 703, 1105, 1, 709, 4, 697, 1001, 64, 1,
                64, 1002, 64, 2, 64, 109, -14, 21108, 46, 45, 7, 1005, 1014, 729, 1001, 64, 1, 64,
                1105, 1, 731, 4, 715, 1002, 64, 2, 64, 109, 7, 21102, 47, 1, 0, 1008, 1014, 45, 63,
                1005, 63, 755, 1001, 64, 1, 64, 1106, 0, 757, 4, 737, 1002, 64, 2, 64, 109, -12,
                2107, 34, 7, 63, 1005, 63, 775, 4, 763, 1105, 1, 779, 1001, 64, 1, 64, 1002, 64, 2,
                64, 109, -5, 1207, 6, 22, 63, 1005, 63, 797, 4, 785, 1106, 0, 801, 1001, 64, 1, 64,
                1002, 64, 2, 64, 109, 12, 1202, 0, 1, 63, 1008, 63, 35, 63, 1005, 63, 827, 4, 807,
                1001, 64, 1, 64, 1105, 1, 827, 1002, 64, 2, 64, 109, -5, 1202, 0, 1, 63, 1008, 63,
                36, 63, 1005, 63, 851, 1001, 64, 1, 64, 1105, 1, 853, 4, 833, 1002, 64, 2, 64, 109,
                -2, 1201, 4, 0, 63, 1008, 63, 20, 63, 1005, 63, 873, 1105, 1, 879, 4, 859, 1001,
                64, 1, 64, 1002, 64, 2, 64, 109, 2, 2107, 22, -1, 63, 1005, 63, 899, 1001, 64, 1,
                64, 1106, 0, 901, 4, 885, 4, 64, 99, 21102, 1, 27, 1, 21101, 0, 915, 0, 1105, 1,
                922, 21201, 1, 53897, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201,
                -2, -1, 1, 21101, 0, 942, 0, 1106, 0, 922, 21202, 1, 1, -1, 21201, -2, -3, 1,
                21101, 0, 957, 0, 1105, 1, 922, 22201, 1, -1, -2, 1105, 1, 968, 22102, 1, -2, -2,
                109, -3, 2105, 1, 0,
            ],
            input_1,
        );

        // Execute:
        vm.run(false);

        // Verify:
        assert_eq!(vec![4_080_871_669], vm.outputs());
    }
}
