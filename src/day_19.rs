use std::collections::VecDeque;
use std::num::ParseIntError;

#[aoc_generator(day19)]
fn parse(s: &str) -> Result<u64, ParseIntError> {
    s.parse()
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
#[aoc(day19, part1)]
fn part_1(&count: &u64) -> u64 {
    let mut queue = (1..=count).collect::<VecDeque<_>>();
    while let Some(first) = queue.pop_front() {
        if let Some(_second) = queue.pop_front() {
            queue.push_back(first);
        } else {
            return first;
        }
    }
    unreachable!("Empty ring?")
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
#[aoc(day19, part2)]
fn part_2(&count: &u64) -> u64 {
    let mut ahead = (1..=count).collect::<VecDeque<_>>();
    let mut behind = ahead.split_off(ahead.len() / 2);
    while let Some(first) = ahead.pop_front() {
        if let Some(_second) = behind.pop_front() {
            behind.push_back(first);
        } else {
            return first;
        }
        if ahead.len() + 1 < behind.len()
            && let Some(rotate) = behind.pop_front()
        {
            ahead.push_back(rotate);
        }
    }
    behind.pop_front().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(&5);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(&5);

        assert_eq!(result, 2);
    }
}