use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid instruction name or syntax")]
    SyntaxError,
    #[error("Invalid register name")]
    InvalidRegister,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    /// Copy
    Cpy(RegOrValue, Reg),
    /// Increase
    Inc(Reg),
    /// Decrease
    Dec(Reg),
    /// Jump if not zero
    Jnz(RegOrValue, isize),
    /// Output
    Out(RegOrValue),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.split_once(' ').ok_or(ParseError::SyntaxError)? {
            ("cpy", rest) => {
                let (a, b) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
                Self::Cpy(a.parse()?, b.parse()?)
            }
            ("inc", rest) => Self::Inc(rest.parse()?),
            ("dec", rest) => Self::Dec(rest.parse()?),
            ("jnz", rest) => {
                let (a, b) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
                Self::Jnz(a.parse()?, b.parse()?)
            }
            ("out", rest) => Self::Out(rest.parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegOrValue {
    Reg(Reg),
    Value(i64),
}

impl FromStr for RegOrValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.as_bytes() {
            [b'0'..=b'9' | b'-', ..] => Self::Value(s.parse()?),
            _ => Self::Reg(s.parse()?),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reg {
    A,
    B,
    C,
    D,
}

impl FromStr for Reg {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "d" => Self::D,
            _ => return Err(ParseError::InvalidRegister),
        })
    }
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug)]
struct Machine<'a> {
    instructions: &'a [Instruction],
    ip: usize,
    registers: [i64; 4],
    stopped: bool,
    output: Vec<i64>,
}

impl<'a> Machine<'a> {
    const fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            ip: 0,
            registers: [0; 4],
            stopped: false,
            output: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.registers = [0; 4];
        self.stopped = false;
        self.output.clear();
    }

    fn output(&self) -> &[i64] {
        &self.output
    }

    const fn get_register(&self, reg: Reg) -> i64 {
        self.registers[reg as usize]
    }

    const fn set_register(&mut self, reg: Reg, value: i64) {
        self.registers[reg as usize] = value;
    }

    const fn get_value(&self, source: RegOrValue) -> i64 {
        match source {
            RegOrValue::Reg(reg) => self.get_register(reg),
            RegOrValue::Value(v) => v,
        }
    }

    fn step(&mut self) {
        if self.stopped {
            return;
        }
        match self.instructions[self.ip] {
            Instruction::Cpy(reg_or_value, reg) => {
                self.set_register(reg, self.get_value(reg_or_value));
            }
            Instruction::Inc(reg) => self.set_register(reg, self.get_register(reg) + 1),
            Instruction::Dec(reg) => self.set_register(reg, self.get_register(reg) - 1),
            Instruction::Jnz(reg_or_value, delta) => {
                if self.get_value(reg_or_value) != 0 {
                    if let Some(new_ip) = self.ip.checked_add_signed(delta)
                        && new_ip < self.instructions.len()
                    {
                        self.ip = new_ip;
                    } else {
                        self.stopped = true;
                    }
                    return;
                }
            }
            Instruction::Out(reg_or_value) => {
                self.output.push(self.get_value(reg_or_value));
                if self.output.len() == 10 {
                    self.stopped = true;
                    return;
                }
            }
        }
        self.ip += 1;
        self.stopped = self.ip >= self.instructions.len();
    }

    fn run(&mut self) {
        while !self.stopped {
            self.step();
        }
    }
}

// #[aoc(day25, part1)]
#[allow(unused, reason = "Alternative solution")]
fn part_1(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine::new(instructions);
    for initial_value in 0.. {
        machine.reset();
        machine.set_register(Reg::A, initial_value);
        machine.run();

        let output = machine.output();
        if output.iter().zip(&output[1..]).all(|(&a, &b)| b == 1 - a) {
            return initial_value;
        }
    }
    0
}

#[aoc(day25, part1)]
fn part_1_faster(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine::new(instructions);
    machine.run(); // A = 0
    // Output will be the lowest 10 bits of A + (secret number)
    let output = machine.output();
    // We need to match an alternating sequence of ones and zeros, which is just:
    let target = 0b10_1010_1010;
    let output_num = output.iter().rev().fold(0, |s, &b| (s << 1) | b);
    // How much we need to add to get to target
    target - output_num
}
