use std::fmt::Write;

const HEX: &[u8] = b"0123456789abcdef";

#[aoc(day5, part1)]
fn part_1(input: &[u8]) -> String {
    let mut res = String::new();

    let mut md5 = md5::Context::new();
    md5.consume(input);

    let mut buf = String::new();
    for x in 1.. {
        buf.clear();
        write!(&mut buf, "{x}").unwrap();

        let mut md5_step = md5.clone();
        md5_step.consume(buf.as_bytes());
        let hash = md5_step.finalize().0;

        if let [0, 0, b, ..] = hash
            && b >> 4 == 0
        {
            res.push(HEX[(b & 0xF) as usize] as char);
            if res.len() >= 8 {
                return res;
            }
        }
    }
    unreachable!()
}

#[aoc(day5, part2)]
fn part_2(input: &[u8]) -> String {
    let mut res = [0u8; 8];

    let mut root = md5::Context::new();
    root.consume(input);

    let mut mask = 0_u8;
    let mut buf = String::new();
    for x in 1.. {
        buf.clear();
        write!(&mut buf, "{x}").unwrap();

        let mut child = root.clone();
        child.consume(buf.as_bytes());
        let hash = child.finalize().0;

        if let [0, 0, pos, ch, ..] = hash
            && pos < 8
            && mask & (1 << pos) == 0
        {
            res[pos as usize] = HEX[(ch >> 4) as usize];
            mask |= 1 << pos;
            if mask == 0xFF {
                return unsafe { String::from_utf8_unchecked(res.into()) };
            }
        }
    }
    unreachable!()
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