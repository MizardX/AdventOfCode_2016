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
struct Disc {
    id: u64,
    num_positions: u64,
    initial_position: u64,
}

impl FromStr for Disc {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Disc #").ok_or(ParseError::SyntaxError)?;
        let (id, rest) = rest.split_once(" has ").ok_or(ParseError::SyntaxError)?;
        let (num_positions, rest) = rest
            .split_once(" positions; at time=0, it is at position ")
            .ok_or(ParseError::SyntaxError)?;
        let initial_position = rest.strip_suffix(".").ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            id: id.parse()?,
            num_positions: num_positions.parse()?,
            initial_position: initial_position.parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Sculpture {
    discs: Vec<Disc>,
}

impl Sculpture {
    fn new(discs: &[Disc]) -> Self {
        Self {
            discs: discs.to_vec(),
        }
    }

    fn find_alignment_time(&self) -> u64 {
        let mut time = 0;
        let mut time_step = 1;
        for disc in &self.discs {
            while (time + disc.id + disc.initial_position) % disc.num_positions != 0 {
                time += time_step;
            }
            time_step *= disc.num_positions;
        }
        time
    }
}

#[aoc_generator(day15)]
fn parse(s: &str) -> Result<Vec<Disc>, ParseError> {
    s.lines().map(str::parse).collect()
}

#[aoc(day15, part1)]
fn part_1(discs: &[Disc]) -> u64 {
    let sculpture = Sculpture::new(discs);
    sculpture.find_alignment_time()
}

#[aoc(day15, part2)]
fn part_2(discs: &[Disc]) -> u64 {
    let mut sculpture = Sculpture::new(discs);
    sculpture.discs.push(Disc {
        id: sculpture.discs.len() as u64 + 1,
        initial_position: 0,
        num_positions: 11,
    });
    sculpture.find_alignment_time()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    Disc #1 has 5 positions; at time=0, it is at position 4.\n\
    Disc #2 has 2 positions; at time=0, it is at position 1.\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            &[
                Disc {
                    id: 1,
                    num_positions: 5,
                    initial_position: 4
                },
                Disc {
                    id: 2,
                    num_positions: 2,
                    initial_position: 1
                },
            ][..]
        );
    }

    #[test]
    fn test_part_1() {
        let sculpture = parse(EXAMPLE).unwrap();
        let result = part_1(&sculpture);
        assert_eq!(result, 5);
    }
}
