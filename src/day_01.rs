use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Mul, MulAssign};
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
    Left,
    Right,
}

impl TryFrom<u8> for Turn {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => return Err(ParseError::SyntaxError),
        })
    }
}

impl Mul<u16> for Turn {
    type Output = Step;

    fn mul(self, rhs: u16) -> Self::Output {
        Step::new(self, rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Step {
    turn: Turn,
    dist: u16,
}

impl Step {
    const fn new(turn: Turn, dist: u16) -> Self {
        Self { turn, dist }
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = s.as_bytes()[0].try_into()?;
        let dist = s[1..].parse()?;
        Ok(Self { turn, dist })
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Result<Vec<Step>, ParseError> {
    input.split(", ").map(str::parse).collect()
}

#[derive(Debug, Clone, Copy, Default)]
enum Direction {
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

impl Mul<Turn> for Direction {
    type Output = Self;

    fn mul(self, rhs: Turn) -> Self::Output {
        match (self, rhs) {
            (Self::East, Turn::Left) | (Self::West, Turn::Right) => Self::North,
            (Self::North, Turn::Right) | (Self::South, Turn::Left) => Self::East,
            (Self::East, Turn::Right) | (Self::West, Turn::Left) => Self::South,
            (Self::North, Turn::Left) | (Self::South, Turn::Right) => Self::West,
        }
    }
}

impl MulAssign<Turn> for Direction {
    fn mul_assign(&mut self, rhs: Turn) {
        *self = *self * rhs;
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Position {
    const fn dist(self) -> u32 {
        self.x.unsigned_abs() + self.y.unsigned_abs()
    }
}

impl Add<Step> for Position {
    type Output = Self;

    fn add(mut self, step: Step) -> Self::Output {
        self.dir *= step.turn;
        match self.dir {
            Direction::North => self.y -= i32::from(step.dist),
            Direction::East => self.x += i32::from(step.dist),
            Direction::South => self.y += i32::from(step.dist),
            Direction::West => self.x -= i32::from(step.dist),
        }
        self
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(mut self, dir: Direction) -> Self::Output {
        self += dir;
        self
    }
}

#[aoc(day1, part1)]
fn part_1(input: &[Step]) -> u32 {
    input
        .iter()
        .copied()
        .fold(Position::default(), Position::add)
        .dist()
}

#[aoc(day1, part2)]
fn part_2(input: &[Step]) -> u32 {
    let mut seen = HashSet::new();
    let mut state = Position::default();
    seen.insert((0, 0));
    for &step in input {
        state.dir *= step.turn;
        for _ in 1..=step.dist {
            state += state.dir;
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

    #[test_case("R2, L3" => &[Turn::Right * 2, Turn::Left * 3][..])]
    #[test_case("R2, R2, R2" => &[Turn::Right * 2, Turn::Right * 2, Turn::Right * 2][..])]
    #[test_case("R5, L5, R5, R3" => &[Turn::Right * 5, Turn::Left * 5, Turn::Right * 5, Turn::Right * 3][..])]
    #[test_case("R8, R4, R4, R8" => &[Turn::Right * 8, Turn::Right * 4, Turn::Right * 4, Turn::Right * 8][..])]
    fn test_parse(input: &str) -> Vec<Step> {
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
