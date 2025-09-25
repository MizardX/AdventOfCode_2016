use std::cmp::Reverse;
use std::convert::identity;

#[aoc(day6, part1)]
fn part_1(input: &str) -> String {
    decode_message(input, identity)
}

#[aoc(day6, part2)]
fn part_2(input: &str) -> String {
    decode_message(input, Reverse)
}

fn decode_message<K: Ord>(input: &str, cmp: fn(usize) -> K) -> String {
    let len = input.lines().next().unwrap().len();
    let mut counts = vec![[0; 26]; len];
    for line in input.lines() {
        for (ch, ch_counts) in line.bytes().zip(&mut counts) {
            ch_counts[(ch - b'a') as usize] += 1;
        }
    }
    let mut res = vec![b'_'; len];
    #[allow(clippy::cast_possible_truncation)]
    for (ch_counts, ch) in counts.iter().zip(&mut res) {
        *ch = ch_counts
            .iter()
            .enumerate()
            .filter(|&(_, &cnt)| cnt > 0)
            .max_by_key(|&(_, &cnt)| cmp(cnt))
            .unwrap()
            .0 as u8
            + b'a';
    }
    unsafe { String::from_utf8_unchecked(res) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
"
    .trim_ascii();

    #[test]
    fn test_part_1() {
        let result = part_1(EXAMPLE);

        assert_eq!(result, "easter");
    }
    
    #[test]
    fn test_part_2() {
        let result = part_2(EXAMPLE);

        assert_eq!(result, "advent");
    }
}
