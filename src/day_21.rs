use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
    #[error("Invalid letter: {0}")]
    InvalidLetter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl FromStr for Letter {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Self::A,
            "b" => Self::B,
            "c" => Self::C,
            "d" => Self::D,
            "e" => Self::E,
            "f" => Self::F,
            "g" => Self::G,
            "h" => Self::H,
            _ => {
                return Err(ParseError::InvalidLetter(
                    s.bytes().next().unwrap_or(b'?') as char
                ));
            }
        })
    }
}

impl Letter {
    const fn to_u8(self) -> u8 {
        self as u8 + b'a'
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    SwapPosition(usize, usize),
    SwapLetters(Letter, Letter),
    RotateLeft(usize),
    RotateRight(usize),
    RotateByLetter(Letter),
    ReverseRange(usize, usize),
    MovePosition(usize, usize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("swap ") {
            if let Some(rest) = rest.strip_prefix("position ") {
                let (x, y) = rest
                    .split_once(" with position ")
                    .ok_or(ParseError::SyntaxError)?;
                Self::SwapPosition(x.parse()?, y.parse()?)
            } else if let Some(rest) = rest.strip_prefix("letter ") {
                let (x, y) = rest
                    .split_once(" with letter ")
                    .ok_or(ParseError::SyntaxError)?;
                Self::SwapLetters(x.parse()?, y.parse()?)
            } else {
                return Err(ParseError::SyntaxError);
            }
        } else if let Some(rest) = s.strip_prefix("rotate ") {
            if let Some(rest) = rest.strip_prefix("left ") {
                let x = rest
                    .trim_end_matches('s')
                    .strip_suffix(" step")
                    .ok_or(ParseError::SyntaxError)?;
                Self::RotateLeft(x.parse()?)
            } else if let Some(rest) = rest.strip_prefix("right ") {
                let x = rest
                    .trim_end_matches('s')
                    .strip_suffix(" step")
                    .ok_or(ParseError::SyntaxError)?;
                Self::RotateRight(x.parse()?)
            } else {
                let letter = rest
                    .strip_prefix("based on position of letter ")
                    .ok_or(ParseError::SyntaxError)?;
                Self::RotateByLetter(letter.parse()?)
            }
        } else if let Some(rest) = s.strip_prefix("reverse positions ") {
            let (x, y) = rest
                .split_once(" through ")
                .ok_or(ParseError::SyntaxError)?;
            Self::ReverseRange(x.parse()?, y.parse()?)
        } else if let Some(rest) = s.strip_prefix("move position ") {
            let (x, y) = rest
                .split_once(" to position ")
                .ok_or(ParseError::SyntaxError)?;
            Self::MovePosition(x.parse()?, y.parse()?)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[aoc_generator(day21)]
fn parse(s: &str) -> Result<Vec<Instruction>, ParseError> {
    s.lines().map(str::parse).collect()
}

#[aoc(day21, part1)]
fn part_1(instructions: &[Instruction]) -> String {
    let mut password = *b"abcdefgh";
    scamble(&mut password, instructions);
    unsafe { str::from_utf8_unchecked(&password) }.to_string()
}

fn scamble(password: &mut [u8], instructions: &[Instruction]) {
    for &instr in instructions {
        match instr {
            Instruction::SwapPosition(pos1, pos2) => password.swap(pos1, pos2),
            Instruction::SwapLetters(let1, let2) => {
                let pos1 = password.iter().position(|&ch| ch == let1.to_u8()).unwrap();
                let pos2 = password.iter().position(|&ch| ch == let2.to_u8()).unwrap();
                password.swap(pos1, pos2);
            }
            Instruction::RotateLeft(n) => password.rotate_left(n),
            Instruction::RotateRight(n) => password.rotate_right(n),
            Instruction::RotateByLetter(letter) => {
                let pos = password
                    .iter()
                    .position(|&ch| ch == letter.to_u8())
                    .unwrap();
                password.rotate_right((pos + 1 + usize::from(pos >= 4)) % password.len());
            }
            Instruction::ReverseRange(pos1, pos2) => {
                password[pos1.min(pos2)..=pos1.max(pos2)].reverse();
            }
            Instruction::MovePosition(pos1, pos2) => {
                if pos1 < pos2 {
                    password[pos1..=pos2].rotate_left(1);
                } else if pos1 > pos2 {
                    password[pos2..=pos1].rotate_right(1);
                }
            }
        }
    }
}

#[aoc(day21, part2)]
fn part_2(instructions: &[Instruction]) -> String {
    let mut scrambled = *b"fbgdceah";
    unscamble(&mut scrambled, instructions);
    unsafe { str::from_utf8_unchecked(&scrambled) }.to_string()
}

fn unscamble(scrambled: &mut [u8], instructions: &[Instruction]) {
    for &instr in instructions.iter().rev() {
        match instr {
            Instruction::SwapPosition(pos1, pos2) => scrambled.swap(pos1, pos2),
            Instruction::SwapLetters(let1, let2) => {
                let pos1 = scrambled.iter().position(|&ch| ch == let1.to_u8()).unwrap();
                let pos2 = scrambled.iter().position(|&ch| ch == let2.to_u8()).unwrap();
                scrambled.swap(pos1, pos2);
            }
            Instruction::RotateLeft(n) => scrambled.rotate_right(n),
            Instruction::RotateRight(n) => scrambled.rotate_left(n),
            Instruction::RotateByLetter(letter) => {
                let n = scrambled.len();
                let mut matched_index = None;
                for candidate_index in 0..n {
                    let rot = (candidate_index + 1 + usize::from(candidate_index >= 4)) % n;
                    scrambled.rotate_left(rot);
                    if scrambled[candidate_index] == letter.to_u8() {
                        if matched_index.is_some() {
                            // Except for only lengths 1, 3, and 8, there is a possibility of multiple valid unscramblings.
                            println!(
                                "Warning: RotateByLetter has multiple unscramblings. Picking the first one."
                            );
                        }
                        matched_index = Some(rot);
                    }
                    scrambled.rotate_right(rot);
                }
                if let Some(rot) = matched_index {
                    scrambled.rotate_left(rot);
                } else {
                    println!("Warning: RotateByLetter no match found");
                }
            }
            Instruction::ReverseRange(pos1, pos2) => {
                scrambled[pos1.min(pos2)..=pos1.max(pos2)].reverse();
            }
            Instruction::MovePosition(pos1, pos2) => {
                if pos1 < pos2 {
                    scrambled[pos1..=pos2].rotate_right(1);
                } else if pos1 > pos2 {
                    scrambled[pos2..=pos1].rotate_left(1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "\
        swap position 4 with position 0\n\
        swap letter d with letter b\n\
        reverse positions 0 through 4\n\
        rotate left 1 step\n\
        move position 1 to position 4\n\
        move position 3 to position 0\n\
        rotate based on position of letter b\n\
        rotate based on position of letter d\n\
    "
    .trim_ascii();

    #[test_case("swap position 4 with position 0" => Instruction::SwapPosition(4, 0))]
    #[test_case("swap letter d with letter b" => Instruction::SwapLetters(Letter::D, Letter::B))]
    #[test_case("reverse positions 0 through 4" => Instruction::ReverseRange(0, 4))]
    #[test_case("rotate left 1 step" => Instruction::RotateLeft(1))]
    #[test_case("move position 1 to position 4" => Instruction::MovePosition(1, 4))]
    #[test_case("move position 3 to position 0" => Instruction::MovePosition(3, 0))]
    #[test_case("rotate based on position of letter b" => Instruction::RotateByLetter(Letter::B))]
    #[test_case("rotate based on position of letter d" => Instruction::RotateByLetter(Letter::D))]
    fn test_parse(line: &str) -> Instruction {
        line.parse().unwrap()
    }

    #[test]
    fn test_scamble() {
        let instructions = parse(EXAMPLE).unwrap();
        let mut password = *b"abcde";
        scamble(&mut password, &instructions);
        assert_eq!(&password, b"decab");
    }

    #[test]
    fn test_unscamble() {
        let instructions = parse(EXAMPLE).unwrap();
        let mut scrambled = *b"decab";
        unscamble(&mut scrambled, &instructions);
        assert_eq!(&scrambled, b"abcde");
    }
}
