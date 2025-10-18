#[derive(Debug, Clone, Copy)]
enum State {
    Literal,
    Len(usize),
    Repeat(usize, usize),
}

#[aoc(day9, part1)]
fn part_1(input: &[u8]) -> usize {
    measure_decoded_length(input, false)
}

#[aoc(day9, part2)]
fn part_2(input: &[u8]) -> usize {
    measure_decoded_length(input, true)
}

fn measure_decoded_length(input: &[u8], version_2: bool) -> usize {
    let mut stream = input.iter().copied().enumerate();
    let mut state = State::Literal;
    let mut total_decode_len = 0;
    while let Some((position, symbol)) = stream.next() {
        state = match (state, symbol) {
            (State::Literal, b'(') => State::Len(0),
            (State::Len(inner_len), b'0'..=b'9') => {
                State::Len(inner_len * 10 + usize::from(symbol - b'0'))
            }
            (State::Len(inner_len), b'x') => State::Repeat(inner_len, 0),
            (State::Repeat(inner_len, inner_reps), b'0'..=b'9') => {
                State::Repeat(inner_len, inner_reps * 10 + usize::from(symbol - b'0'))
            }
            (State::Repeat(inner_len, inner_reps), b')') => {
                let decoded_len = if version_2 {
                    measure_decoded_length(
                        &input[position + 1..position + 1 + inner_len],
                        version_2,
                    )
                } else {
                    inner_len
                };
                total_decode_len += decoded_len * inner_reps;
                let _ = stream.nth(inner_len - 1); // advance_by(n) is experimental
                State::Literal
            }
            _ => {
                total_decode_len += 1;
                State::Literal
            }
        };
    }
    total_decode_len
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
    fn test_part_1(input: &[u8]) -> usize {
        part_1(input)
    }

    #[test_case(b"(3x3)XYZ" => b"XYZXYZXYZ".len())]
    #[test_case(b"X(8x2)(3x3)ABCY" => b"XABCABCABCABCABCABCY".len())]
    #[test_case(b"(27x12)(20x12)(13x14)(7x10)(1x12)A" => 241_920)]
    #[test_case(b"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN" => 445)]
    fn test_part_2(input: &[u8]) -> usize {
        part_2(input)
    }
}
