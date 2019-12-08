use std::fs;
use std::io;
use std::env;
use std::process;

const ADD: i32 = 1;
const MULTIPLY: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMPIFTRUE: i32 = 5;
const JUMPIFFALSE: i32 = 6;
const LESSTHAN: i32 = 7;
const EQUALS: i32 = 8;
const HALT: i32 = 99;

#[derive(Debug, PartialEq)]
enum Parameter {
    Position(usize),
    Immediate(i32),
}

impl Parameter {
    fn new(value: i32, mode: i32) -> Result<Parameter, &'static str> {
        match mode {
            0 => Ok(Parameter::Position(value as usize)),
            1 => Ok(Parameter::Immediate(value)),
            _ => Err("Unexpected parameter mode"),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add(Parameter, Parameter, usize),
    Multiply(Parameter, Parameter, usize),
    Input(usize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equals(Parameter, Parameter, usize),
    Halt,
}

impl Instruction {
    fn parse(program: &[i32]) -> Result<Instruction, &'static str> {
        let opcode = program[0] % 100;
        let c_mode = (program[0] / 100) % 10;
        let b_mode = (program[0] / 1000) % 10;

        match opcode {
            ADD => Ok(Instruction::Add(
                Parameter::new(program[1], c_mode)?, 
                Parameter::new(program[2], b_mode)?, 
                program[3] as usize,
            )),
            MULTIPLY => Ok(Instruction::Multiply(
                Parameter::new(program[1], c_mode)?, 
                Parameter::new(program[2], b_mode)?, 
                program[3] as usize,
            )),
            INPUT => Ok(Instruction::Input(program[1] as usize)),
            OUTPUT => Ok(Instruction::Output(
                Parameter::new(program[1], c_mode)?
            )),
            JUMPIFTRUE => Ok(Instruction::JumpIfTrue(
                Parameter::new(program[1], c_mode)?,
                Parameter::new(program[2], b_mode)?
            )),
            JUMPIFFALSE => Ok(Instruction::JumpIfFalse(
                Parameter::new(program[1], c_mode)?,
                Parameter::new(program[2], b_mode)?
            )),
            LESSTHAN => Ok(Instruction::LessThan(
                Parameter::new(program[1], c_mode)?,
                Parameter::new(program[2], b_mode)?,
                program[3] as usize
            )),
            EQUALS => Ok(Instruction::Equals(
                Parameter::new(program[1], c_mode)?,
                Parameter::new(program[2], b_mode)?,
                program[3] as usize
            )),
            HALT => Ok(Instruction::Halt),
            _ => Err("Unexpected opcode"),
        }
    }
}

#[derive(Debug)]
struct IntcodeComputer {
    pc: usize,
    program: Vec<i32>,
}

impl IntcodeComputer {
    fn new(program: Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            pc: 0,
            program,
        }
    }

    fn get_parameter_value(&self, parameter: &Parameter) -> i32 {
        match parameter {
            Parameter::Position(value) => self.program[*value],
            Parameter::Immediate(value) => *value,
        }
    }

    fn next_instruction(&self) -> Result<Instruction, &'static str> {
        Instruction::parse(&self.program[self.pc..])
    }

    fn set(&mut self, index: usize, value: i32) {
        self.program[index] = value;
    }

    fn jump(&mut self, value: usize) {
        self.pc = value;
    }

    fn run(&mut self) -> Result<(), &'static str> {
        loop {
            let instruction = self.next_instruction()?;
            match instruction {
                Instruction::Add(a, b, result) => {
                    self.set(result, self.get_parameter_value(&a) + self.get_parameter_value(&b));
                    self.jump(self.pc + 4);
                }
                Instruction::Multiply(a, b, result) => {
                    self.set(result, self.get_parameter_value(&a) * self.get_parameter_value(&b));
                    self.jump(self.pc + 4);
                },
                Instruction::Input(location) => {
                    loop {
                        println!("Enter a number:");
                        let mut input = String::new();
                        match io::stdin().read_line(&mut input) {
                            Ok(_) => {
                                match input.trim().parse() {
                                    Ok(value) => {
                                        self.set(location, value);
                                        self.jump(self.pc + 2);
                                        break;
                                    },
                                    Err(_) => println!("Not a valid number"),
                                }
                            },
                            Err(_) => println!("Failed to read input"),
                        }
                    }
                },
                Instruction::Output(i) => {
                    println!("{}", self.get_parameter_value(&i));
                    self.jump(self.pc + 2);
                },
                Instruction::JumpIfTrue(value, target) => {
                    if self.get_parameter_value(&value) != 0 {
                        self.jump(self.get_parameter_value(&target) as usize);
                    } else {
                        self.jump(self.pc + 3);
                    }
                },
                Instruction::JumpIfFalse(value, target) => {
                    if self.get_parameter_value(&value) == 0 {
                        self.jump(self.get_parameter_value(&target) as usize);
                    } else {
                        self.jump(self.pc + 3);
                    }
                },
                Instruction::LessThan(a, b, result) => {
                    let value = if self.get_parameter_value(&a) < self.get_parameter_value(&b) {
                        1
                    } else {
                        0
                    };

                    self.set(result, value);
                    self.jump(self.pc + 4);
                },
                Instruction::Equals(a, b, result) => {
                    let value = if self.get_parameter_value(&a) == self.get_parameter_value(&b) {
                        1
                    } else {
                        0
                    };

                    self.set(result, value);
                    self.jump(self.pc + 4);
                }
                Instruction::Halt => break Ok(()),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file required");
    let program: Vec<i32> = fs::read_to_string(input_file).expect("Unable to read input file")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    
    let mut computer = IntcodeComputer::new(program);

    if let Err(e) = computer.run() {
        eprintln!("Error: {}", e);
        eprintln!("PC: {}", computer.pc);
        eprintln!("Program (Starting at PC): {:?}", &computer.program[computer.pc..]);
        eprintln!("Final computer state: {:?}", computer);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_parse_add() {
        let instruction = Instruction::parse(&[10101, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(instruction, Instruction::Add(
            Parameter::Immediate(3),
            Parameter::Position(4),
            5
        ));
    }

    #[test]
    fn test_instruction_parse_multiply() {
        let instruction = Instruction::parse(&[1002, 3, 4, 5, 6, 7]).unwrap();

        assert_eq!(instruction, Instruction::Multiply(
            Parameter::Position(3),
            Parameter::Immediate(4),
            5
        ));
    }

    #[test]
    fn test_instruction_parse_input() {
        let instruction = Instruction::parse(&[103, 18]).unwrap();

        assert_eq!(instruction, Instruction::Input(18));
    }

    #[test]
    fn test_instruction_parse_output() {
        let instruction = Instruction::parse(&[4, 56]).unwrap();

        assert_eq!(instruction, Instruction::Output(
            Parameter::Position(56)
        ));
    }

    #[test]
    fn test_instruction_parse_halt() {
        let instruction = Instruction::parse(&[99, 100]).unwrap();

        assert_eq!(instruction, Instruction::Halt);
    }

    #[test]
    fn test_instruction_parse_bad_opcode() {
        let instruction = Instruction::parse(&[23, 100]);

        assert_eq!(instruction, Err("Unexpected opcode"));
    }

    #[test]
    fn test_instruction_parse_bad_parameter_type() {
        let instruction = Instruction::parse(&[201, 100]);

        assert_eq!(instruction, Err("Unexpected parameter mode"));
    }

    #[test]
    fn test_get_parameter_value_position() {
        let computer = IntcodeComputer::new(vec!(12, 2, 3, 4));
        let value = computer.get_parameter_value(&Parameter::Position(2));

        assert_eq!(value, 3);
    }

    #[test]
    fn test_get_parameter_value_immediate() {
        let computer = IntcodeComputer::new(vec!(12, 2, 3, 4));
        let value = computer.get_parameter_value(&Parameter::Immediate(2));

        assert_eq!(value, 2);
    }

    #[test]
    fn test_next_instruction() {
        let mut computer = IntcodeComputer::new(vec!(12, 102, 3, 4, 5));
        computer.pc = 1;
        let instruction = computer.next_instruction().unwrap();

        assert_eq!(instruction, Instruction::Multiply(
            Parameter::Immediate(3),
            Parameter::Position(4),
            5
        ));
    }

    #[test]
    fn test_run() {
        let mut computer = IntcodeComputer::new(vec!(10101, 10, 6, 0, 11002, 0, 20, 4, 99));
        computer.run().unwrap();

        assert_eq!(computer.program, vec!(30, 10, 6, 0, 600, 0, 20, 4, 99));
    }
}