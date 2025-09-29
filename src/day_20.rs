use std::cmp::Reverse;
use std::collections::BinaryHeap;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    s.lines().map(str::parse).collect()
}

#[aoc(day20, part1)]
fn part_1(blocked: &[Range]) -> u32 {
    let mut pending = blocked.to_vec();
    pending.sort_unstable_by_key(|r| (r.0, Reverse(r.1)));

    let mut open = BinaryHeap::<u32>::new();
    let mut last_close = 0;
    for range in pending {
        while let Some(&end) = open.peek() {
            if end <= range.0 {
                last_close = last_close.max(end);
                open.pop();
            } else {
                break;
            }
        }
        if open.is_empty() && last_close < range.0 {
            return last_close;
        }
        open.push(range.1 + 1);
    }
    0
}

#[aoc(day20, part2)]
fn part_2(blocked: &[Range]) -> u64 {
    count_nonblocked(blocked, u64::from(u32::MAX) + 1)
}

fn count_nonblocked(blocked: &[Range], max: u64) -> u64 {
    let mut count_nonblocked = 0;
    let mut pending = blocked.to_vec();
    pending.sort_unstable_by_key(|r| (r.0, Reverse(r.1)));

    let mut open = BinaryHeap::<u64>::new();
    let mut last_close = 0;
    for range in pending {
        while let Some(&end) = open.peek() {
            if end <= u64::from(range.0) {
                last_close = last_close.max(end);
                open.pop();
            } else {
                break;
            }
        }
        if open.is_empty() && last_close < u64::from(range.0) {
            count_nonblocked += u64::from(range.0) - last_close;
        }
        open.push(u64::from(range.1) + 1);
    }
    while let Some(&end) = open.peek() {
        if end <= max {
            last_close = last_close.max(end);
            open.pop();
        } else {
            break;
        }
    }
    if open.is_empty() && last_close < max {
        count_nonblocked += max - last_close;
    }
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
