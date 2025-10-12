use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Seed(Seed),
    Bot(Bot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seed {
    value: u32,
    destination: Destination,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Bot {
    id: usize,
    low: Destination,
    high: Destination,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Destination {
    #[default]
    None,
    Bot(usize),
    Output(usize),
}

impl FromStr for Destination {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(num) = s.strip_prefix("bot ") {
            Self::Bot(num.parse()?)
        } else if let Some(num) = s.strip_prefix("output ") {
            Self::Output(num.parse()?)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("bot ") {
            let (bot, rest) = rest
                .split_once(" gives low to ")
                .ok_or(ParseError::SyntaxError)?;
            let (low, high) = rest
                .split_once(" and high to ")
                .ok_or(ParseError::SyntaxError)?;
            Self::Bot(Bot {
                id: bot.parse()?,
                low: low.parse()?,
                high: high.parse()?,
            })
        } else if let Some(rest) = s.strip_prefix("value ") {
            let (value, dest) = rest
                .split_once(" goes to ")
                .ok_or(ParseError::SyntaxError)?;
            Self::Seed(Seed {
                value: value.parse()?,
                destination: dest.parse()?,
            })
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Lot<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

impl<T: PartialEq> PartialEq for Lot<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(l0), Self::Single(r0)) => l0 == r0,
            (Self::Pair(l0, l1), Self::Pair(r0, r1)) => {
                (l0, l1) == (r0, r1) || (l0, r1) == (r1, r0)
            }
            (Self::Empty, Self::Empty) => true,
            _ => false,
        }
    }
}

impl<T: Eq> Eq for Lot<T> {}

impl<T: Copy> Lot<T> {
    fn push(&mut self, value: T) {
        *self = match *self {
            Self::Empty => Self::Single(value),
            Self::Single(x) => Self::Pair(x, value),
            Self::Pair(_, _) => panic!("Push into pull Lot"),
        };
    }

    const fn is_full(&self) -> bool {
        matches!(self, Self::Pair(_, _))
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day10, part1)]
fn part_1(instructions: &[Instruction]) -> usize {
    let mut factory: Factory = instructions.into();
    factory.simulate();
    for (ix, &inv) in factory.bot_inventory.iter().enumerate() {
        if let Lot::Pair(x, y) = inv
            && x.min(y) == 17
            && x.max(y) == 61
        {
            return ix;
        }
    }
    usize::MAX
}

#[aoc(day10, part2)]
fn part_2(instructions: &[Instruction]) -> u32 {
    let mut factory: Factory = instructions.into();
    factory.simulate();
    if let &[Lot::Single(x), Lot::Single(y), Lot::Single(z), ..] = &factory.outputs[..] {
        x * y * z
    } else {
        0
    }
}

struct Factory {
    bot_inventory: Vec<Lot<u32>>,
    bots: Vec<Bot>,
    outputs: Vec<Lot<u32>>,
    queue: VecDeque<usize>,
}

impl From<&[Instruction]> for Factory {
    fn from(instructions: &[Instruction]) -> Self {
        let (num_bots, num_outputs) = count_bots_and_outputs(instructions);
        let mut bot_inventory = vec![Lot::Empty; num_bots];
        let mut bots = vec![Bot::default(); num_bots];
        let mut outputs = vec![Lot::Empty; num_outputs];
        for &instr in instructions {
            match instr {
                Instruction::Seed(seed) => match seed.destination {
                    Destination::Bot(n) => bot_inventory[n].push(seed.value),
                    Destination::Output(n) => outputs[n].push(seed.value),
                    Destination::None => (),
                },
                Instruction::Bot(bot) => bots[bot.id] = bot,
            }
        }
        Self {
            bot_inventory,
            bots,
            outputs,
            queue: VecDeque::new(),
        }
    }
}

impl Factory {
    fn simulate(&mut self) {
        self.queue.clear();
        for (ix, &inv) in self.bot_inventory.iter().enumerate() {
            if let Lot::Pair(_, _) = inv {
                self.queue.push_back(ix);
            }
        }
        while let Some(ix) = self.queue.pop_front() {
            let Lot::Pair(x, y) = self.bot_inventory[ix] else {
                panic!("Trying to process bot without both values")
            };
            self.send_value(self.bots[ix].low, x.min(y));
            self.send_value(self.bots[ix].high, x.max(y));
        }
    }

    fn send_value(&mut self, dest: Destination, value: u32) {
        match dest {
            Destination::Bot(n) => {
                self.bot_inventory[n].push(value);
                if self.bot_inventory[n].is_full() {
                    self.queue.push_back(n);
                }
            }
            Destination::Output(n) => self.outputs[n].push(value),
            Destination::None => (),
        }
    }
}

fn count_bots_and_outputs(instructions: &[Instruction]) -> (usize, usize) {
    let mut num_bots = 0;
    let mut num_outputs = 0;
    for &instr in instructions {
        match instr {
            Instruction::Seed(seed) => match seed.destination {
                Destination::Bot(n) => num_bots = num_bots.max(n + 1),
                Destination::Output(n) => num_outputs = num_outputs.max(n + 1),
                Destination::None => (),
            },
            Instruction::Bot(bot) => {
                num_bots = num_bots.max(bot.id + 1);
                match bot.low {
                    Destination::Bot(n) => num_bots = num_bots.max(n + 1),
                    Destination::Output(n) => num_outputs = num_outputs.max(n + 1),
                    Destination::None => (),
                }
                match bot.high {
                    Destination::Bot(n) => num_bots = num_bots.max(n + 1),
                    Destination::Output(n) => num_outputs = num_outputs.max(n + 1),
                    Destination::None => (),
                }
            }
        }
    }
    (num_bots, num_outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        value 5 goes to bot 2\n\
        bot 2 gives low to bot 1 and high to bot 0\n\
        value 3 goes to bot 1\n\
        bot 1 gives low to output 1 and high to bot 0\n\
        bot 0 gives low to output 2 and high to output 0\n\
        value 2 goes to bot 2\
    ";

    #[test]
    fn test_parse() {
        let instructions = parse(EXAMPLE).unwrap();
        let expected = &[
            Instruction::Seed(Seed {
                value: 5,
                destination: Destination::Bot(2),
            }),
            Instruction::Bot(Bot {
                id: 2,
                low: Destination::Bot(1),
                high: Destination::Bot(0),
            }),
            Instruction::Seed(Seed {
                value: 3,
                destination: Destination::Bot(1),
            }),
            Instruction::Bot(Bot {
                id: 1,
                low: Destination::Output(1),
                high: Destination::Bot(0),
            }),
            Instruction::Bot(Bot {
                id: 0,
                low: Destination::Output(2),
                high: Destination::Output(0),
            }),
            Instruction::Seed(Seed {
                value: 2,
                destination: Destination::Bot(2),
            }),
        ][..];

        assert_eq!(instructions, expected);
    }

    #[test]
    fn test_factory() {
        let instructions = parse(EXAMPLE).unwrap();
        let mut factory: Factory = instructions.as_slice().into();
        factory.simulate();

        assert_eq!(
            factory.outputs,
            &[Lot::Single(5), Lot::Single(2), Lot::Single(3)][..]
        );
        assert_eq!(factory.bot_inventory[2], Lot::Pair(5, 2));
    }
}
