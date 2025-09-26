#[derive(Debug, Clone, Copy)]
enum State {
    Normal,
    Len(u64),
    Repeat(u64, u64),
}

#[aoc(day9, part1)]
fn part_1(input: &[u8]) -> u64 {
    let mut it = input.iter().copied();
    let mut state = State::Normal;
    let mut decoded = 0;
    while let Some(ch) = it.next() {
        state = match (state, ch) {
            (State::Normal, b'(') => State::Len(0),
            (State::Len(n), b'0'..=b'9') => State::Len(n * 10 + u64::from(ch - b'0')),
            (State::Len(n), b'x') => State::Repeat(n, 0),
            (State::Repeat(n, r), b'0'..=b'9') => State::Repeat(n, r * 10 + u64::from(ch - b'0')),
            (State::Repeat(n, r), b')') => {
                decoded += n * r;
                let n = usize::try_from(n).unwrap();
                let _ = it.nth(n - 1); // advance_by(n) is experimental
                State::Normal
            }
            _ => {
                decoded += 1;
                State::Normal
            }
        };
    }
    decoded
}

#[aoc(day9, part2)]
fn part_2(input: &[u8]) -> u64 {
    let mut it = input.iter().copied().enumerate();
    let mut state = State::Normal;
    let mut decoded = 0;
    while let Some((i, ch)) = it.next() {
        state = match (state, ch) {
            (State::Normal, b'(') => State::Len(0),
            (State::Len(n), b'0'..=b'9') => State::Len(n * 10 + u64::from(ch - b'0')),
            (State::Len(n), b'x') => State::Repeat(n, 0),
            (State::Repeat(n, r), b'0'..=b'9') => State::Repeat(n, r * 10 + u64::from(ch - b'0')),
            (State::Repeat(n, r), b')') => {
                let n = usize::try_from(n).unwrap();
                decoded += part_2(&input[i + 1..i + 1 + n]) * r;
                let _ = it.nth(n - 1); // advance_by(n) is experimental
                State::Normal
            }
            _ => {
                decoded += 1;
                State::Normal
            }
        };
    }
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"ADVENT" => 6)]
    #[test_case(b"A(1x5)BC" => 7)]
    #[test_case(b"(3x3)XYZ" => 9)]
    #[test_case(b"A(2x2)BCD(2x2)EFG" => 11)]
    #[test_case(b"(6x1)(1x3)A" => 6)]
    #[test_case(b"X(8x2)(3x3)ABCY" => 18)]
    fn test_part_1(input: &[u8]) -> u64 {
        part_1(input)
    }

    #[test_case(b"(3x3)XYZ" => b"XYZXYZXYZ".len() as u64)]
    #[test_case(b"X(8x2)(3x3)ABCY" => b"XABCABCABCABCABCABCY".len() as u64)]
    #[test_case(b"(27x12)(20x12)(13x14)(7x10)(1x12)A" => 241_920)]
    #[test_case(b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" => 445)]
    fn test_part_2(input: &[u8]) -> u64 {
        part_2(input)
    }
}
