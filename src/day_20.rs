use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Invalid range")]
    InvalidRange,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Range(u32, u32);

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(ParseError::InvalidRange)?;
        Ok(Self(start.parse()?, end.parse()?))
    }
}

#[aoc_generator(day20)]
fn parse(s: &str) -> Result<Vec<Range>, ParseError> {
    let mut blocked = s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?;
    blocked.sort_unstable();
    Ok(blocked)
}

#[aoc(day20, part1)]
fn part_1(blocked: &[Range]) -> u32 {
    let mut first_free = 0;
    for range in blocked {
        if first_free < range.0 {
            return first_free;
        }
        first_free = first_free.max(range.1 + 1);
    }
    0
}

#[aoc(day20, part2)]
fn part_2(blocked: &[Range]) -> u64 {
    count_nonblocked(blocked, 1 << u32::BITS)
}

fn count_nonblocked(blocked: &[Range], max: u64) -> u64 {
    let mut count_nonblocked = 0;

    let mut first_free = 0;
    for range in blocked {
        count_nonblocked += u64::from(range.0).saturating_sub(first_free);
        first_free = first_free.max(u64::from(range.1) + 1);
    }
    count_nonblocked += max.saturating_sub(first_free);
    count_nonblocked
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        5-8\n\
        0-2\n\
        4-7\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(result, [Range(5, 8), Range(0, 2), Range(4, 7)]);
    }

    #[test]
    fn test_part_1() {
        let blocked = parse(EXAMPLE).unwrap();
        let result = part_1(&blocked);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let blocked = parse(EXAMPLE).unwrap();
        let result = count_nonblocked(&blocked, 10);
        assert_eq!(result, 2);
    }
}
