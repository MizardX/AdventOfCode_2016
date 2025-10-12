use std::fmt::Display;
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
    Cpy(RegOrValue, RegOrValue),
    /// Increase
    Inc(RegOrValue),
    /// Decrease
    Dec(RegOrValue),
    /// Jump if not zero
    Jnz(RegOrValue, RegOrValue),
    /// Toggle
    Tgl(RegOrValue),
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
            ("tgl", rest) => Self::Tgl(rest.parse()?),
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Cpy(a, b) => write!(f, "cpy {a} {b}"),
            Self::Inc(a) => write!(f, "inc {a}"),
            Self::Dec(a) => write!(f, "dec {a}"),
            Self::Jnz(a, b) => write!(f, "jnz {a} {b}"),
            Self::Tgl(a) => write!(f, "tgl {a}"),
        }
    }
}

impl Instruction {
    const fn toggle(self) -> Self {
        match self {
            Self::Cpy(a, b) => Self::Jnz(a, b),
            Self::Inc(a) => Self::Dec(a),
            Self::Dec(a) | Self::Tgl(a) => Self::Inc(a),
            Self::Jnz(a, b) => Self::Cpy(a, b),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegOrValue {
    Reg(Reg),
    Value(isize),
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

impl Display for RegOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => write!(f, "{reg}"),
            Self::Value(val) => write!(f, "{val}"),
        }
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

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day23, part1)]
fn part_1(instructions: &[Instruction]) -> isize {
    let mut machine = Machine::new(instructions);
    // machine.set_register(Reg::A, 7);
    // machine.run();
    // machine.get_register(Reg::A)
    machine.set_register(Reg::A, 6);
    machine.run();
    let delta = machine.get_register(Reg::A) - (1..=6).product::<isize>();
    (1..=7).product::<isize>() + delta
}

#[aoc(day23, part2)]
fn part_2(instructions: &[Instruction]) -> isize {
    let mut machine = Machine::new(instructions);
    // machine.set_register(Reg::A, 12);
    // machine.run();
    // machine.get_register(Reg::A)
    machine.set_register(Reg::A, 6);
    machine.run();
    let delta = machine.get_register(Reg::A) - (1..=6).product::<isize>();
    (1..=12).product::<isize>() + delta
}

#[derive(Debug, Clone)]
struct Machine {
    instructions: Vec<Instruction>,
    ip: usize,
    registers: [isize; 4],
    stopped: bool,
}

impl Machine {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            instructions: instructions.to_vec(),
            ip: 0,
            registers: [0; 4],
            stopped: false,
        }
    }

    const fn get_register(&self, reg: Reg) -> isize {
        self.registers[reg as usize]
    }

    const fn set_register(&mut self, reg: Reg, value: isize) {
        self.registers[reg as usize] = value;
    }

    const fn get_value(&self, source: RegOrValue) -> isize {
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
            Instruction::Cpy(value, reg) => {
                if let RegOrValue::Reg(reg) = reg {
                    self.set_register(reg, self.get_value(value));
                }
            }
            Instruction::Inc(reg) => {
                if let RegOrValue::Reg(reg) = reg {
                    self.set_register(reg, self.get_register(reg) + 1);
                }
            }
            Instruction::Dec(reg) => {
                if let RegOrValue::Reg(reg) = reg {
                    self.set_register(reg, self.get_register(reg) - 1);
                }
            }
            Instruction::Jnz(condition, distance) => {
                if self.get_value(condition) != 0 {
                    if let Some(new_ip) = self.ip.checked_add_signed(self.get_value(distance))
                        && new_ip < self.instructions.len()
                    {
                        self.ip = new_ip;
                    } else {
                        self.stopped = true;
                    }
                    return;
                }
            }
            Instruction::Tgl(distance) => {
                if let Some(new_ip) = self.ip.checked_add_signed(self.get_value(distance))
                    && new_ip < self.instructions.len()
                {
                    self.instructions[new_ip] = self.instructions[new_ip].toggle();
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

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [reg_a, reg_b, reg_c, reg_d] = self.registers;
        writeln!(f, "A: {reg_a}, B: {reg_b}, C: {reg_c}, D: {reg_d}")?;
        for (i, instr) in self.instructions.iter().enumerate() {
            let active = if self.ip == i { '>' } else { ' ' };
            writeln!(f, "{i:2}) {active} {instr}")?;
        }
        writeln!(f, "---")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        cpy 2 a\n\
        tgl a\n\
        tgl a\n\
        tgl a\n\
        cpy 1 a\n\
        dec a\n\
        dec a\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                Instruction::Cpy(RegOrValue::Value(2), RegOrValue::Reg(Reg::A)),
                Instruction::Tgl(RegOrValue::Reg(Reg::A)),
                Instruction::Tgl(RegOrValue::Reg(Reg::A)),
                Instruction::Tgl(RegOrValue::Reg(Reg::A)),
                Instruction::Cpy(RegOrValue::Value(1), RegOrValue::Reg(Reg::A)),
                Instruction::Dec(RegOrValue::Reg(Reg::A)),
                Instruction::Dec(RegOrValue::Reg(Reg::A)),
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let instructions = parse(EXAMPLE).unwrap();
        let result = part_1(&instructions);
        assert_eq!(result, 3);
    }
}
