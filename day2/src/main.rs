use std::env;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Add(u32, u32, u32),
    Mul(u32, u32, u32),
    End,
}

#[derive(Debug)]
struct IntcodeComputer {
    pc: usize,
    program: Vec<u32>,
}

impl IntcodeComputer {
    fn new(program: Vec<u32>) -> IntcodeComputer {
        IntcodeComputer {
            pc: 0,
            program,
        }
    }

    fn current_instruction(&self) -> Instruction {
        match self.get(self.pc) {
            1 => Instruction::Add(self.parameter(0), self.parameter(1), self.parameter(2)),
            2 => Instruction::Mul(self.parameter(0), self.parameter(1), self.parameter(2)),
            99 => Instruction::End,
            _ => panic!("Unexpected instruction"),
        }
    }

    fn get(&self, addr: usize) -> u32 {
        self.program[addr]
    }

    fn parameter(&self, parameter: usize) -> u32 {
        self.get(self.pc + 1 + parameter)
    }

    fn set(&mut self, index: usize, value: u32) {
        self.program[index] = value;
    }

    fn step(&mut self, amount: usize) {
        self.pc += amount;
    }

    fn run(&mut self) -> u32 {
        loop {
            match self.current_instruction() {
                Instruction::Add(a, b, c) => {
                    self.set(c as usize, self.get(a as usize) + self.get(b as usize));
                    self.step(4);
                },
                Instruction::Mul(a, b, c) => {
                    self.set(c as usize, self.get(a as usize) * self.get(b as usize));
                    self.step(4);
                },
                Instruction::End => break self.get(0),
            }
        }
    }
}

fn solve_inputs(program: Vec<u32>, target: u32) -> Option<(u32, u32)> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut computer = IntcodeComputer::new(program.clone());
            computer.set(1, noun);
            computer.set(2, verb);

            if computer.run() == target {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file required");
    let target: u32 = args.get(2).expect("Target value required")
        .parse().expect("Expected a number for target value");

    let program: Vec<u32> = fs::read_to_string(input_file).expect("Unable to read input file")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    match solve_inputs(program, target) {
        Some((noun, verb)) => println!("Noun: {}, Verb: {}, Result: {}", noun, verb, 100 * noun + verb),
        None => println!("No noun/verb combination available for: {}", target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_computer {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (program, expected_program, expected_output) = $value;
                    let mut computer = IntcodeComputer::new(program.to_vec());

                    let output = computer.run();

                    assert_eq!(output, expected_output);
                    assert_eq!(computer.program, expected_program.to_vec());
                }
            )*
        }
    }

    // Testing examples from challenge docs
    test_computer! {
        test_computer_example_1: ([1,0,0,0,99], [2,0,0,0,99], 2),
        test_computer_example_2: ([2,3,0,3,99], [2,3,0,6,99], 2),
        test_computer_example_3: ([2,4,4,5,99,0], [2,4,4,5,99,9801], 2),
        test_computer_example_4: ([1,1,1,4,99,5,6,0,99], [30,1,1,4,2,5,6,0,99], 30),
    }
}