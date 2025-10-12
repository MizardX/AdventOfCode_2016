use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Missing delimiter")]
    MissingDelimiter,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Room {
    name: String,
    sector: u64,
    checksum: String,
}

use std::cmp::Reverse;

impl Room {
    fn valid_checksum(&self) -> bool {
        let mut counts = [0; 26];
        for ch in self.name.bytes() {
            if ch != b'-' {
                counts[(ch - b'a') as usize] += 1;
            }
        }
        let mut prev_count = u8::MAX;
        let mut prev_ch = b'~';
        for ch in self.checksum.bytes() {
            let cnt = &mut counts[(ch - b'a') as usize];
            if *cnt == 0 || (*cnt, Reverse(ch)) >= (prev_count, Reverse(prev_ch)) {
                return false;
            }
            (prev_count, prev_ch) = (*cnt, ch);
            *cnt = 0;
        }
        #[expect(clippy::cast_possible_truncation, reason = "index < 26")]
        counts.iter().enumerate().all(|(index, &cnt)| {
            (cnt, Reverse(b'a' + index as u8)) < (prev_count, Reverse(prev_ch))
        })
    }

    fn decrypt_name(&self) -> String {
        let mut res = self.name.clone();
        let bytes = unsafe { res.as_bytes_mut() };
        for ch in bytes {
            *ch = match *ch {
                b'-' => b' ',
                x @ b'a'..=b'z' => ((u64::from(x - b'a') + self.sector) % 26) as u8 + b'a',
                x => x,
            };
        }
        res
    }
}

impl FromStr for Room {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (rest, checksum) = if let Some(rest) = value.strip_suffix(']') {
            rest.rsplit_once('[').ok_or(ParseError::MissingDelimiter)?
        } else {
            (value, "")
        };
        let (name, sector) = rest.rsplit_once('-').ok_or(ParseError::MissingDelimiter)?;
        let sector = sector.parse()?;
        Ok(Self {
            name: name.to_string(),
            sector,
            checksum: checksum.to_string(),
        })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Result<Vec<Room>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day4, part1)]
fn part_1(input: &[Room]) -> u64 {
    input
        .iter()
        .filter_map(|r| r.valid_checksum().then_some(r.sector))
        .sum()
}

#[aoc(day4, part2)]
fn part_2(input: &[Room]) -> u64 {
    input
        .iter()
        .find_map(|r| {
            (r.valid_checksum() && r.decrypt_name().contains("northpole")).then_some(r.sector)
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "\
        aaaaa-bbb-z-y-x-123[abxyz]\n\
        a-b-c-d-e-f-g-h-987[abcde]\n\
        not-a-real-room-404[oarel]\n\
        totally-real-room-200[decoy]\
    ";

    const EXAMPLE2: &str = "\
        qzmt-zixmtkozy-ivhz-343\
    ";

    macro_rules! room {
        ($name:literal, $sector:literal, $checksum:literal) => {
            Room {
                name: String::from($name),
                sector: $sector,
                checksum: String::from($checksum),
            }
        };
    }

    #[test]
    fn test_parse_1() {
        let result = parse(EXAMPLE1).unwrap();
        assert_eq!(
            result,
            [
                room!("aaaaa-bbb-z-y-x", 123, "abxyz"),
                room!("a-b-c-d-e-f-g-h", 987, "abcde"),
                room!("not-a-real-room", 404, "oarel"),
                room!("totally-real-room", 200, "decoy"),
            ]
        );
    }

    #[test]
    fn test_parse_2() {
        let result = parse(EXAMPLE2).unwrap();

        assert_eq!(result, [room!("qzmt-zixmtkozy-ivhz", 343, "")]);
    }

    #[test]
    fn test_valid_checksum() {
        let rooms = parse(EXAMPLE1).unwrap();
        let valid: Vec<_> = rooms.into_iter().map(|r| r.valid_checksum()).collect();
        let expected = &[true, true, true, false][..];
        assert_eq!(valid, expected);
    }

    #[test]
    fn test_part_1() {
        let rooms = parse(EXAMPLE1).unwrap();
        let result = part_1(&rooms);
        assert_eq!(result, 1514);
    }

    #[test]
    fn test_decrypt() {
        let rooms = parse(EXAMPLE2).unwrap();
        let translated: Vec<_> = rooms.into_iter().map(|r| r.decrypt_name()).collect();
        let expected = &["very encrypted name"][..];

        assert_eq!(translated, expected);
    }
}
