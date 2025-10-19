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
    /// value .. goes to ..
    Seed(Seed),
    /// bot .. gives low to .. and high to ..
    Bot(Bot),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("bot ") {
            let (id, rest) = rest
                .split_once(" gives low to ")
                .ok_or(ParseError::SyntaxError)?;
            let (low_to, high_to) = rest
                .split_once(" and high to ")
                .ok_or(ParseError::SyntaxError)?;
            Self::Bot(Bot {
                id: id.parse()?,
                low_to: low_to.parse()?,
                high_to: high_to.parse()?,
            })
        } else if let Some(rest) = s.strip_prefix("value ") {
            let (value, value_to) = rest
                .split_once(" goes to ")
                .ok_or(ParseError::SyntaxError)?;
            Self::Seed(Seed {
                value: value.parse()?,
                value_to: value_to.parse()?,
            })
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

/// value `value` goes to `value_to`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Seed {
    value: u32,
    value_to: Destination,
}

/// bot `id` gives low to `low_to` and high to `high_to`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bot {
    id: usize,
    low_to: Destination,
    high_to: Destination,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Destination {
    /// bot `0`
    Bot(usize),
    /// output `0`
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

#[derive(Debug, Error)]
enum HeapError {
    #[error("Tried to push onto a full TwoHeap")]
    HeapFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TwoHeap<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

impl<T: Copy + Ord> TwoHeap<T> {
    fn push(&mut self, value: T) -> Result<(), HeapError> {
        *self = match *self {
            Self::Empty => Self::Single(value),
            Self::Single(x) if x <= value => Self::Pair(x, value),
            Self::Single(x) => Self::Pair(value, x),
            Self::Pair(_, _) => return Err(HeapError::HeapFull),
        };
        Ok(())
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
    let mut factory = Factory::new(instructions).unwrap();
    factory.simulate().unwrap();
    for (ix, &inv) in factory.bot_inventory.iter().enumerate() {
        if inv == TwoHeap::Pair(17, 61) {
            return ix;
        }
    }
    usize::MAX
}

#[aoc(day10, part2)]
fn part_2(instructions: &[Instruction]) -> u32 {
    let mut factory = Factory::new(instructions).unwrap();
    factory.simulate().unwrap();
    if let &[
        TwoHeap::Single(x),
        TwoHeap::Single(y),
        TwoHeap::Single(z),
        ..,
    ] = &factory.outputs[..]
    {
        x * y * z
    } else {
        0
    }
}

#[derive(Debug, Error)]
enum FactoryError {
    #[error("Trying to process bot without both inputs")]
    UnpreparedBot,
    #[error(transparent)]
    HeapError(#[from] HeapError),
}

#[derive(Debug, Clone)]
struct Factory {
    bot_inventory: Vec<TwoHeap<u32>>,
    bots: Vec<Option<Bot>>,
    outputs: Vec<TwoHeap<u32>>,
    events: VecDeque<usize>,
}

impl Factory {
    fn new(instructions: &[Instruction]) -> Result<Self, FactoryError> {
        let (num_bots, num_outputs) = count_bots_and_outputs(instructions);
        let mut bot_inventory = vec![TwoHeap::Empty; num_bots];
        let mut bots = vec![None; num_bots];
        let mut outputs = vec![TwoHeap::Empty; num_outputs];
        for &instr in instructions {
            match instr {
                Instruction::Seed(seed) => match seed.value_to {
                    Destination::Bot(bot_ix) => bot_inventory[bot_ix].push(seed.value)?,
                    Destination::Output(output_ix) => outputs[output_ix].push(seed.value)?,
                },
                Instruction::Bot(bot) => bots[bot.id] = Some(bot),
            }
        }
        Ok(Self {
            bot_inventory,
            bots,
            outputs,
            events: VecDeque::new(),
        })
    }

    fn simulate(&mut self) -> Result<(), FactoryError> {
        self.events.clear();
        for (bot_ix, &inv) in self.bot_inventory.iter().enumerate() {
            if let TwoHeap::Pair(..) = inv {
                self.events.push_back(bot_ix);
            }
        }
        while let Some(bot_ix) = self.events.pop_front() {
            let TwoHeap::Pair(low, high) = self.bot_inventory[bot_ix] else {
                return Err(FactoryError::UnpreparedBot);
            };
            let Bot {
                low_to, high_to, ..
            } = self.bots[bot_ix].unwrap();
            self.send_value(low_to, low)?;
            self.send_value(high_to, high)?;
        }
        Ok(())
    }

    fn send_value(&mut self, dest: Destination, value: u32) -> Result<(), FactoryError> {
        match dest {
            Destination::Bot(bot_ix) => {
                self.bot_inventory[bot_ix].push(value)?;
                if self.bot_inventory[bot_ix].is_full() {
                    self.events.push_back(bot_ix);
                }
            }
            Destination::Output(output_ix) => self.outputs[output_ix].push(value)?,
        }
        Ok(())
    }
}

fn count_bots_and_outputs(instructions: &[Instruction]) -> (usize, usize) {
    let mut num_bots = 0;
    let mut num_outputs = 0;
    macro_rules! count_dest {
        ($dest:expr) => {
            match $dest {
                Destination::Bot(bot_ix) => num_bots = num_bots.max(bot_ix + 1),
                Destination::Output(output_ix) => num_outputs = num_outputs.max(output_ix + 1),
            }
        };
    }
    for &instr in instructions {
        match instr {
            Instruction::Seed(seed) => count_dest!(seed.value_to),
            Instruction::Bot(bot) => {
                num_bots = num_bots.max(bot.id + 1);
                count_dest!(bot.low_to);
                count_dest!(bot.high_to);
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
                value_to: Destination::Bot(2),
            }),
            Instruction::Bot(Bot {
                id: 2,
                low_to: Destination::Bot(1),
                high_to: Destination::Bot(0),
            }),
            Instruction::Seed(Seed {
                value: 3,
                value_to: Destination::Bot(1),
            }),
            Instruction::Bot(Bot {
                id: 1,
                low_to: Destination::Output(1),
                high_to: Destination::Bot(0),
            }),
            Instruction::Bot(Bot {
                id: 0,
                low_to: Destination::Output(2),
                high_to: Destination::Output(0),
            }),
            Instruction::Seed(Seed {
                value: 2,
                value_to: Destination::Bot(2),
            }),
        ][..];

        assert_eq!(instructions, expected);
    }

    #[test]
    fn test_factory() {
        let instructions = parse(EXAMPLE).unwrap();
        let mut factory: Factory = Factory::new(&instructions).unwrap();
        factory.simulate().unwrap();

        assert_eq!(
            factory.outputs,
            &[TwoHeap::Single(5), TwoHeap::Single(2), TwoHeap::Single(3)][..]
        );
        assert_eq!(factory.bot_inventory[2], TwoHeap::Pair(2, 5));
    }
}
