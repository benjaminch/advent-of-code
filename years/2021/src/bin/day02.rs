use std::io::{self, Error, Read, Write};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let instructions: Vec<Instruction> = get_instructions_from_str(&input);

    // Part 1
    let mut position = Position::new();
    for &instruction in instructions.iter() {
        position.apply_instruction(instruction);
    }

    writeln!(
        io::stdout(),
        "Part 1 / Position: {:?}, Answer (depth * horizontal): {} ",
        position,
        position.depth * position.horizontal
    )?;

    // Part 2
    let mut position = Position::new();
    for &instruction in instructions.iter() {
        position.apply_instruction_v2(instruction);
    }

    writeln!(
        io::stdout(),
        "Part 2 / Position: {:?}, Answer (depth * horizontal): {} ",
        position,
        position.depth * position.horizontal
    )?;

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Down(d) => {
                self.depth += d;
            }
            Instruction::Up(d) => {
                self.depth -= d;
            }
            Instruction::Forward(d) => {
                self.horizontal += d;
            }
        };
    }

    fn apply_instruction_v2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Down(d) => {
                self.aim += d;
            }
            Instruction::Up(d) => {
                self.aim -= d;
            }
            Instruction::Forward(d) => {
                self.horizontal += d;
                self.depth += self.aim * d;
            }
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Down(i32),
    Up(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let split: Vec<&str> = s.trim().split(" ").collect();

        if split.len() != 2 {
            return Err(format!("bad input string: `{}`", s));
        }

        let distance_raw = split
            .last()
            .expect("cannot get last element from split string");
        let distance = match distance_raw.parse::<i32>() {
            Ok(d) => d,
            Err(_) => {
                return Err(format!("unknown distance: `{}`", distance_raw));
            }
        };

        let move_raw = split
            .first()
            .expect("cannot get first element from split string");
        match *move_raw {
            "down" => Ok(Instruction::Down(distance)),
            "up" => Ok(Instruction::Up(distance)),
            "forward" => Ok(Instruction::Forward(distance)),
            _ => Err(format!("unknown move: `{}`", move_raw)),
        }
    }
}

fn get_instructions_from_str(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .flat_map(|e| e.trim().parse::<Instruction>())
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_instruction_parse_fromm_str() {
        // setup:
        let inputs = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];

        // execute:
        let result = inputs
            .iter()
            .map(|e| e.parse::<Instruction>().unwrap())
            .collect::<Vec<Instruction>>();

        // verify:
        assert_eq!(
            vec![
                Instruction::Forward(5),
                Instruction::Down(5),
                Instruction::Forward(8),
                Instruction::Up(3),
                Instruction::Down(8),
                Instruction::Forward(2)
            ],
            result
        );
    }

    #[test]
    fn test_apply_instruction() {
        // setup:
        let instructions = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];
        let mut result = Position::new();

        // execute:
        for instruction in instructions {
            result.apply_instruction(instruction);
        }

        // verify:
        assert_eq!(
            Position {
                horizontal: 15,
                depth: 10,
                aim: 0
            },
            result
        );
    }

    #[test]
    fn test_apply_instruction_v2() {
        // setup:
        let instructions = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];
        let mut result = Position::new();

        // execute:
        for instruction in instructions {
            result.apply_instruction_v2(instruction);
        }

        // verify:
        assert_eq!(
            Position {
                horizontal: 15,
                depth: 60,
                aim: 10
            },
            result
        );
    }
}
