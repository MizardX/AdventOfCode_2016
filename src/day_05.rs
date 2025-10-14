use std::fmt::Write;

use rayon::iter::{
    IndexedParallelIterator, IntoParallelIterator, ParallelExtend, ParallelIterator,
};

const HEX: &[u8] = b"0123456789abcdef";

#[aoc(day5, part1)]
fn part_1(input: &[u8]) -> String {
    let mut res = Vec::new();

    let root = &{
        let mut root = md5::Context::new();
        root.consume(input);
        root
    };

    let mut start = 1_usize;
    let mut window_size = 256;
    while res.len() < 8 {
        res.par_extend(
            (start..start + window_size)
                .into_par_iter()
                .map_init(String::new, |buf, x| {
                    let mut child = root.clone();

                    buf.clear();
                    write!(buf, "{x}").unwrap();
                    child.consume(buf.as_bytes());

                    let hash = child.finalize().0;

                    if let [0, 0, b, ..] = hash
                        && b >> 4 == 0
                    {
                        Some(HEX[(b & 0xF) as usize])
                    } else {
                        None
                    }
                })
                .flatten_iter(),
        );
        start += window_size;
        window_size *= 2;
    }
    res.truncate(8);
    String::from_utf8(res).unwrap()
}

#[aoc(day5, part2)]
fn part_2(input: &[u8]) -> String {
    let mut res = [0u8; 8];

    let root = &{
        let mut root = md5::Context::new();
        root.consume(input);
        root
    };

    let mut mask = 0_u8;
    let mut start = 1_usize;
    let mut window_size = 0x10000;
    let mut events = Vec::new();
    while mask != 0xFF {
        events.par_extend(
            (start..start + window_size)
                .into_par_iter()
                .by_exponential_blocks()
                .map_init(String::new, |buf, x| {
                    let mut child = root.clone();

                    buf.clear();
                    write!(buf, "{x}").unwrap();
                    child.consume(buf.as_bytes());

                    let hash = child.finalize().0;

                    if let [0, 0, pos, ch, ..] = hash
                        && pos < 8
                        && mask & (1 << pos) == 0
                    {
                        Some((pos as usize, HEX[(ch >> 4) as usize]))
                    } else {
                        None
                    }
                })
                .flatten_iter(),
        );
        for &(pos, ch) in &events {
            let bit = 1 << pos;
            if mask & bit == 0 {
                mask |= bit;
                res[pos] = ch;
            }
        }
        events.clear();
        start += window_size;
        window_size *= 2;
    }
    str::from_utf8(&res).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Expensive"]
    fn test_part_1() {
        let result = part_1(b"abc");
        let expected = "18f47a30";

        assert_eq!(result, expected);
    }

    #[test]
    #[ignore = "Expensive"]
    fn test_part_2() {
        let result = part_2(b"abc");
        let expected = "05ace8e3";

        assert_eq!(result, expected);
    }
}
