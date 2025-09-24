use std::fmt::{Display,  Write};
use std::ops::Add;

use thiserror::Error;

#[derive(Debug,  Error)]
enum ParseError {
    #[error("Invalid char: {0:?}")]
    InvalidChar(char), 
}

#[derive(Debug,  Clone,  Copy,  PartialEq,  Eq)]
enum Dir {
    Up, 
    Right, 
    Down, 
    Left, 
}

impl TryFrom<u8> for Dir {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self,  Self::Error> {
        Ok(match value {
            b'U' => Self::Up, 
            b'R' => Self::Right, 
            b'D' => Self::Down, 
            b'L' => Self::Left, 
            _ => return Err(ParseError::InvalidChar(value as char)), 
        })
    }
}

#[derive(Debug,  Default,  Clone,  Copy,  PartialEq,  Eq)]
enum Keypad1 {
    One, 
    Two, 
    Three, 
    Four, 
    #[default]
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
}

impl Display for Keypad1 {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::One => '1', 
            Self::Two => '2', 
            Self::Three => '3', 
            Self::Four => '4', 
            Self::Five => '5', 
            Self::Six => '6', 
            Self::Seven => '7', 
            Self::Eight => '8', 
            Self::Nine => '9', 
        })
    }
}

trait Keypad: Sized + Copy {
    fn pos(self) -> (u8,  u8);
    fn from_pos(pos: (u8,  u8)) -> Option<Self>;
    fn add_impl(self,  dir: Dir) -> Self {
        let (mut x,  mut y) = self.pos();
        match dir {
            Dir::Up => y -= 1, 
            Dir::Right => x += 1, 
            Dir::Down => y += 1, 
            Dir::Left => x -= 1, 
        }
        Self::from_pos((x,  y)).unwrap_or(self)
    }
}

impl Add<Dir> for Keypad1 {
    type Output = Self;

    fn add(self,  dir: Dir) -> Self::Output {
        self.add_impl(dir)
    }
}

impl Keypad for Keypad1 {
    fn pos(self) -> (u8,  u8) {
        match self {
            Self::One => (1,  1), 
            Self::Two => (2,  1), 
            Self::Three => (3,  1), 
            Self::Four => (1,  2), 
            Self::Five => (2,  2), 
            Self::Six => (3,  2), 
            Self::Seven => (1,  3), 
            Self::Eight => (2,  3), 
            Self::Nine => (3,  3), 
        }
    }

    fn from_pos(pos: (u8,  u8)) -> Option<Self> {
        Some(match pos {
            (1,  1) => Self::One, 
            (2,  1) => Self::Two, 
            (3,  1) => Self::Three, 
            (1,  2) => Self::Four, 
            (2,  2) => Self::Five, 
            (3,  2) => Self::Six, 
            (1,  3) => Self::Seven, 
            (2,  3) => Self::Eight, 
            (3,  3) => Self::Nine, 
            _ => return None, 
        })
    }
}

#[aoc_generator(day2)]
fn parse(input: &[u8]) -> Result<Vec<Vec<Dir>>,  ParseError> {
    input
        .split(|&ch| ch == b'\n')
        .map(|line| {
            line.iter()
                .copied()
                .map(Dir::try_from)
                .collect::<Result<Vec<_>,  _>>()
        })
        .collect()
}

#[aoc(day2,  part1)]
fn part_1(input: &[Vec<Dir>]) -> String {
    let mut result = String::new();
    let mut state = Keypad1::default();
    for line in input {
        for &step in line {
            state = state + step;
        }
        write!(&mut result,  "{state}").unwrap();
    }
    result
}

#[derive(Debug,  Default,  Clone,  Copy)]
enum Keypad2 {
    One, 
    Two, 
    Three, 
    Four, 
    #[default]
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    A, 
    B, 
    C, 
    D, 
}

impl Display for Keypad2 {
    fn fmt(&self,  f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::One => '1', 
            Self::Two => '2', 
            Self::Three => '3', 
            Self::Four => '4', 
            Self::Five => '5', 
            Self::Six => '6', 
            Self::Seven => '7', 
            Self::Eight => '8', 
            Self::Nine => '9', 
            Self::A => 'A', 
            Self::B => 'B', 
            Self::C => 'C', 
            Self::D => 'D', 
        })
    }
}

impl Keypad for Keypad2 {
    fn pos(self) -> (u8,  u8) {
        match self {
            Self::One => (3,  1), 
            Self::Two => (2,  2), 
            Self::Three => (3,  2), 
            Self::Four => (4,  2), 
            Self::Five => (1,  3), 
            Self::Six => (2,  3), 
            Self::Seven => (3,  3), 
            Self::Eight => (4,  3), 
            Self::Nine => (5,  3), 
            Self::A => (2,  4), 
            Self::B => (3,  4), 
            Self::C => (4,  4), 
            Self::D => (3,  5), 
        }
    }

    fn from_pos(pos: (u8,  u8)) -> Option<Self> {
        Some(match pos {
            (3,  1) => Self::One, 
            (2,  2) => Self::Two, 
            (3,  2) => Self::Three, 
            (4,  2) => Self::Four, 
            (1,  3) => Self::Five, 
            (2,  3) => Self::Six, 
            (3,  3) => Self::Seven, 
            (4,  3) => Self::Eight, 
            (5,  3) => Self::Nine, 
            (2,  4) => Self::A, 
            (3,  4) => Self::B, 
            (4,  4) => Self::C, 
            (3,  5) => Self::D, 
            _ => return None, 
        })
    }
}

impl Add<Dir> for Keypad2 {
    type Output = Self;

    fn add(self,  dir: Dir) -> Self::Output {
        self.add_impl(dir)
    }
}

#[aoc(day2,  part2)]
fn part_2(input: &[Vec<Dir>]) -> String {
    let mut result = String::new();
    let mut state = Keypad2::default();
    for line in input {
        for &step in line {
            state = state + step;
        }
        write!(&mut result,  "{state}").unwrap();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"ULL\nRRDDD\nLURDL\nUUUUD" => &[
        &[Dir::Up, Dir::Left, Dir::Left][..],
        &[Dir::Right, Dir::Right, Dir::Down, Dir::Down, Dir::Down][..],
        &[Dir::Left, Dir::Up, Dir::Right, Dir::Down, Dir::Left][..],
        &[Dir::Up,  Dir::Up, Dir::Up, Dir::Up, Dir::Down][..],
    ][..]; "example")]
    fn test_parse(input: &[u8]) -> Vec<Vec<Dir>> {
        parse(input).unwrap()
    }

    #[test_case(b"ULL\nRRDDD\nLURDL\nUUUUD" => "1985"; "example 1")]
    fn test_part_1(input: &[u8]) -> String {
        let instruction = parse(input).unwrap();
        part_1(&instruction)
    }

    #[test_case(b"ULL\nRRDDD\nLURDL\nUUUUD" => "5DB3"; "example 2")]
    fn test_part_2(input: &[u8]) -> String {
        let instruction = parse(input).unwrap();
        part_2(&instruction)
    }
}
