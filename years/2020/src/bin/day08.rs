use parse_display::{Display, FromStr};
use std::collections::HashSet;
use std::io::{self, Error, Read, Write};

#[derive(Display, FromStr, PartialEq, Debug, Copy, Clone)]
enum Operation {
    #[display("acc {0}")]
    Accumulate(i32),

    #[display("jmp {0}")]
    Jump(i32),

    #[display("nop {0}")]
    NoOperation(i32),
}

struct Executor<'a> {
    accumulator: i32,
    code_pointer: usize,
    operations: &'a Vec<Operation>,
    executed_operations: HashSet<usize>,
    instruction_to_be_changed_pointer: Option<usize>,
}

impl<'a> Executor<'a> {
    const EMTPY_OPERATIONS: &'a Vec<Operation> = &Vec::new();

    fn new() -> Executor<'a> {
        Executor {
            accumulator: 0,
            code_pointer: 0,
            operations: Executor::EMTPY_OPERATIONS,
            executed_operations: HashSet::new(),
            instruction_to_be_changed_pointer: None,
        }
    }

    fn set_operations(&mut self, operations: &'a Vec<Operation>) {
        self.operations = operations;
    }

    fn reset(&mut self) {
        self.code_pointer = 0;
        self.accumulator = 0;
        self.executed_operations = HashSet::new();
    }

    fn execute(&mut self) -> Result<(), Error> {
        if self.instruction_to_be_changed_pointer.is_none() {
            self.instruction_to_be_changed_pointer = Some(0)
        } else {
            self.instruction_to_be_changed_pointer = Some(
                (self.instruction_to_be_changed_pointer.unwrap() + 1)
                    .rem_euclid(self.operations.len()),
            );
        }

        let mut operation_changed = false;

        while self.code_pointer < self.operations.len() {
            let mut operation: Operation = self.operations[self.code_pointer];

            if !operation_changed
                && self.instruction_to_be_changed_pointer.is_some()
                && self.instruction_to_be_changed_pointer.unwrap() == self.code_pointer
            {
                operation = match operation {
                    Operation::Jump(value) => Operation::NoOperation(value),
                    Operation::NoOperation(value) => Operation::Jump(value),
                    _ => operation,
                };

                operation_changed = true;
            }

            match operation {
                Operation::Accumulate(value) => {
                    self.accumulator += value;
                    self.code_pointer += 1;
                }
                Operation::Jump(value) => {
                    self.code_pointer = (self.code_pointer as i32 + value) as usize;
                }
                Operation::NoOperation(_) => {
                    self.code_pointer += 1;
                }
            }

            if self.executed_operations.contains(&self.code_pointer) {
                return Err(Error::other("LOOP!"));
            } else {
                self.executed_operations.insert(self.code_pointer);
            }
        }

        Ok(())
    }
}

fn get_operations_from_input(input: &str) -> Result<Vec<Operation>, Error> {
    Ok(input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<Operation>>())
}

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let operations: Vec<Operation> = get_operations_from_input(&input)?;
    let mut executor = Executor::new();
    executor.set_operations(&operations);

    // Part 1
    match executor.execute() {
        Ok(_) => writeln!(io::stdout(), "Part 1/ Error, should have crashed :("),
        Err(_) => writeln!(io::stdout(), "Part 1/ ACC: {}", executor.accumulator),
    }?;

    // Part 2
    loop {
        executor.reset();
        if executor.execute().is_ok() {
            writeln!(io::stdout(), "Part - 2 / ACC: {}", executor.accumulator,)?;
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
