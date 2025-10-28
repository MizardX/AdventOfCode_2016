use std::collections::VecDeque;
use std::num::ParseIntError;

#[aoc_generator(day19)]
fn parse(s: &str) -> Result<u64, ParseIntError> {
    s.parse()
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
#[aoc(day19, part1)]
const fn part_1_jospehus(&count: &u64) -> u64 {
    let highest_one = 1 << (63 - count.leading_zeros());
    (count - highest_one) * 2 + 1
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
// #[aoc(day19, part1)]
#[allow(unused, reason = "reference, tests")]
fn part_1_deques(&count: &u64) -> u64 {
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
const fn part_2_josephus(&count: &u64) -> u64 {
    let mut highest_power_of_3 = 1_u64;
    while let Some(next) = highest_power_of_3.checked_mul(3)
        && next < count
    {
        highest_power_of_3 = next;
    }
    count.saturating_sub(highest_power_of_3) + count.saturating_sub(2 * highest_power_of_3)
}

#[expect(clippy::trivially_copy_pass_by_ref, reason = "AOC library")]
//#[aoc(day19, part2)]
#[allow(unused, reason = "reference, tests")]
fn part_2_deques(&count: &u64) -> u64 {
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
    fn pattern_1() {
        for n in 1..=10_000 {
            let res = part_1_deques(&n);
            let approx = part_1_jospehus(&n);
            assert_eq!(res, approx);
        }
    }

    #[test]
    fn pattern_2() {
        for n in 2..=10_000 {
            let res = part_2_deques(&n);
            let approx = part_2_josephus(&n);
            assert_eq!(res, approx);
        }
    }

    #[test]
    fn test_part_1() {
        let result = part_1_jospehus(&5);

        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2() {
        let result = part_2_josephus(&5);

        assert_eq!(result, 2);
    }
}
