use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Add;
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
enum Turn {
    Left(u16),
    Right(u16),
}

impl FromStr for Turn {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("R") {
            Self::Right(rest.parse()?)
        } else if let Some(rest) = s.strip_prefix("L") {
            Self::Left(rest.parse()?)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<Turn>, ParseError> {
    input.split(", ").map(str::parse).collect()
}

#[derive(Debug, Clone, Copy, Default)]
enum Dir {
    #[default]
    /// -y
    North,
    /// +x
    East,
    /// +y
    South,
    /// -x
    West,
}

#[derive(Debug, Clone, Copy, Default)]
struct State {
    x: i32,
    y: i32,
    dir: Dir,
}

impl State {
    const fn dist(self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl Add<Turn> for State {
    type Output = Self;

    fn add(self, turn: Turn) -> Self::Output {
        match (self.dir, turn) {
            (Dir::North, Turn::Left(d)) | (Dir::South, Turn::Right(d)) => Self {
                x: self.x - i32::from(d),
                dir: Dir::West,
                ..self
            },
            (Dir::North, Turn::Right(d)) | (Dir::South, Turn::Left(d)) => Self {
                x: self.x + i32::from(d),
                dir: Dir::East,
                ..self
            },
            (Dir::East, Turn::Left(d)) | (Dir::West, Turn::Right(d)) => Self {
                y: self.y - i32::from(d),
                dir: Dir::North,
                ..self
            },
            (Dir::East, Turn::Right(d)) | (Dir::West, Turn::Left(d)) => Self {
                y: self.y + i32::from(d),
                dir: Dir::South,
                ..self
            },
        }
    }
}

impl Add<Dir> for State {
    type Output = Self;

    fn add(mut self, dir: Dir) -> Self::Output {
        match dir {
            Dir::North => self.y -= 1,
            Dir::East => self.x += 1,
            Dir::South => self.y += 1,
            Dir::West => self.x -= 1,
        }
        self
    }
}

#[aoc(day1, part1)]
fn part_1(input: &[Turn]) -> u32 {
    input
        .iter()
        .copied()
        .fold(State::default(), State::add)
        .dist()
}

#[aoc(day1, part2)]
fn part_2(input: &[Turn]) -> u32 {
    let mut seen = HashSet::new();
    let mut state = State::default();
    seen.insert((0, 0));
    for &turn in input {
        let d;
        (d, state) = match turn {
            Turn::Left(d) => (d, state + Turn::Left(0)),
            Turn::Right(d) => (d, state + Turn::Right(0)),
        };
        for _ in 1..=d {
            state = state + state.dir;
            if !seen.insert((state.x, state.y)) {
                return state.dist();
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("R2, L3" => &[Turn::Right(2), Turn::Left(3)][..])]
    #[test_case("R2, R2, R2" => &[Turn::Right(2), Turn::Right(2), Turn::Right(2)][..])]
    #[test_case("R5, L5, R5, R3" => &[Turn::Right(5), Turn::Left(5), Turn::Right(5), Turn::Right(3)][..])]
    #[test_case("R8, R4, R4, R8" => &[Turn::Right(8), Turn::Right(4), Turn::Right(4), Turn::Right(8)][..])]
    fn test_parse(input: &str) -> Vec<Turn> {
        parse(input).unwrap()
    }

    #[test_case("R2, L3" => 5)]
    #[test_case("R2, R2, R2" => 2)]
    #[test_case("R5, L5, R5, R3" => 12)]
    fn test_part_1(input: &str) -> u32 {
        let turns = parse(input).unwrap();
        part_1(&turns)
    }

    #[test_case("R8, R4, R4, R8" => 4)]
    fn test_part_2(input: &str) -> u32 {
        let turns = parse(input).unwrap();
        part_2(&turns)
    }
}
