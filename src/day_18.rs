use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid char: {0:?}")]
    InvalidChar(char),
}

#[derive(Debug, Clone, Copy)]
struct Traps {
    mask: u128,
    traps: u128,
}

impl FromStr for Traps {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut traps = 0_u128;
        let mut mask = 0_u128;
        for (i, ch) in s.bytes().enumerate() {
            traps |= match ch {
                b'^' => 1_u128,
                b'.' => 0_u128,
                _ => return Err(ParseError::InvalidChar(ch as char)),
            } << (s.len() - 1 - i);
            mask |= 1_u128 << (s.len() - 1 - i);
        }
        Ok(Self { mask, traps })
    }
}

impl Traps {
    const fn step(self) -> Self {
        let traps = ((self.traps << 1) ^ (self.traps >> 1)) & self.mask;
        Self { traps, ..self }
    }

    const fn count_safe(self) -> u32 {
        (self.traps ^ self.mask).count_ones()
    }
}

#[aoc_generator(day18)]
fn parse(s: &str) -> Result<Traps, ParseError> {
    s.parse()
}

#[aoc(day18, part1)]
fn part_1(traps: &Traps) -> u32 {
    count_safe(*traps, 40)
}

#[aoc(day18, part2)]
fn part_2(traps: &Traps) -> u32 {
    count_safe(*traps, 400_000)
}

fn count_safe(mut traps: Traps, rows: usize) -> u32 {
    let mut total = 0;
    for _ in 0..rows {
        total += traps.count_safe();
        traps = traps.step();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "..^^.";
    const EXAMPLE2: &str = ".^^.^.^^^^";

    #[test]
    fn test_example_1() {
        let mut traps: Traps = EXAMPLE1.parse().unwrap();
        let mut counts = vec![traps.count_safe()];
        for _ in 1..3 {
            traps = traps.step();
            counts.push(traps.count_safe());
        }
        assert_eq!(counts, [3, 1, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut traps: Traps = EXAMPLE2.parse().unwrap();
        let mut counts = vec![traps.count_safe()];
        for _ in 1..10 {
            traps = traps.step();
            counts.push(traps.count_safe());
        }

        assert_eq!(counts, [3, 5, 4, 5, 3, 5, 3, 3, 4, 3]);
    }
}
