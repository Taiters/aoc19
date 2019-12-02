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
            let instruction = self.current_instruction();
            println!("Executing: {:?}", instruction);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file required");
    let noun: u32 = match args.get(2) {
        Some(n) => {
            let number = n.parse().expect("Expected a number for noun");
            if number < 100 {
                number
            } else {
                panic!("verb is out of range (Expected: 0 <= noun < 100)")
            }
        },
        None => 0,
    };
    let verb: u32 = match args.get(3) {
        Some(n) => {
            let number = n.parse().expect("Expected a number for verb");
            if number < 100 {
                number
            } else {
                panic!("verb is out of range (Expected: 0 <= verb < 100)")
            }
        },
        None => 0,
    };

    let program = fs::read_to_string(input_file).expect("Unable to read input file")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut computer = IntcodeComputer::new(program);
    computer.set(1, noun);
    computer.set(2, verb);

    println!("{}", computer.run());
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