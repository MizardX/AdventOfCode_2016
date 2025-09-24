use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid syntax")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {
    const fn new(a: u32, b: u32, c: u32) -> Self {
        Self { a, b, c }
    }
    const fn is_possible(&self) -> bool {
        let (mut a, mut b, mut c) = (self.a, self.b, self.c);
        if a > b {
            (a, b) = (b, a);
        }
        if b > c {
            (b, c) = (c, b);
        }
        if a > b {
            (a, b) = (b, a);
        }
        a + b > c
    }

    const fn transpose(first: Self, second: Self, third: Self) -> [Self; 3] {
        [
            Self::new(first.a, second.a, third.a),
            Self::new(first.b, second.b, third.b),
            Self::new(first.c, second.c, third.c),
        ]
    }
}

impl FromStr for Triangle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "  123  456  789"
        //  012345678901234
        let mut words = s.trim_ascii().split_ascii_whitespace();
        let first = words.next().ok_or(ParseError::SyntaxError)?;
        let second = words.next().ok_or(ParseError::SyntaxError)?;
        let third = words.next().ok_or(ParseError::SyntaxError)?;
        if words.next().is_some() {
            return Err(ParseError::SyntaxError);
        }
        Ok(Self {
            a: first.parse()?,
            b: second.parse()?,
            c: third.parse()?,
        })
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Result<Vec<Triangle>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day3, part1)]
fn part_1(input: &[Triangle]) -> usize {
    input.iter().copied().filter(Triangle::is_possible).count()
}

#[aoc(day3, part2)]
fn part_2(input: &[Triangle]) -> usize {
    input
        .chunks_exact(3)
        .flat_map(|arr| Triangle::transpose(arr[0], arr[1], arr[2]))
        .filter(Triangle::is_possible)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE1: &str = "
5 10 25
"
    .trim_ascii();

    const EXAMPLE2: &str = "
101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603
"
    .trim_ascii();

    #[test_case(EXAMPLE1 => &[Triangle::new(5,10,25)][..]; "Parse example 1")]
    #[test_case(EXAMPLE2 => &[
        Triangle::new(101,301,501),
        Triangle::new(102,302,502),
        Triangle::new(103,303,503),
        Triangle::new(201,401,601),
        Triangle::new(202,402,602),
        Triangle::new(203,403,603),
    ][..]; "Parse example 2")]
    fn test_parse(input: &str) -> Vec<Triangle> {
        parse(input).unwrap()
    }

    #[test_case(EXAMPLE1 => 0)]
    #[test_case(EXAMPLE2 => 3)]
    fn test_part_1(input: &str) -> usize {
        let triangles = parse(input).unwrap();
        part_1(&triangles)
    }

    #[test_case(EXAMPLE2 => 6)]
    fn test_part_2(input: &str) -> usize {
        let triangles = parse(input).unwrap();
        part_2(&triangles)
    }
}
