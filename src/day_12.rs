use std::num::ParseIntError;
use std::str::FromStr;

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

#[aoc_generator(day12)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[derive(Debug, Clone)]
struct Machine<'a> {
    instructions: &'a [Instruction],
    ip: usize,
    registers: [i64; 4],
    stopped: bool,
}

impl<'a> Machine<'a> {
    const fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            ip: 0,
            registers: [0; 4],
            stopped: false,
        }
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

#[aoc(day12, part1)]
fn part_1(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine::new(instructions);
    machine.run();
    machine.get_register(Reg::A)
}

#[aoc(day12, part2)]
fn part_2(instructions: &[Instruction]) -> i64 {
    let mut machine = Machine::new(instructions);
    machine.set_register(Reg::C, 1);
    machine.run();
    machine.get_register(Reg::A)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        cpy 41 a\n\
        inc a\n\
        inc a\n\
        dec a\n\
        jnz a 2\n\
        dec a\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            &[
                Instruction::Cpy(RegOrValue::Value(41), Reg::A),
                Instruction::Inc(Reg::A),
                Instruction::Inc(Reg::A),
                Instruction::Dec(Reg::A),
                Instruction::Jnz(RegOrValue::Reg(Reg::A), 2),
                Instruction::Dec(Reg::A),
            ][..]
        );
    }

    #[test]
    fn test_part_1() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = part_1(&instructions);
        assert_eq!(result, 42);
    }
}
