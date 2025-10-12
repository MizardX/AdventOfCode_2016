use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(rest) = s.strip_prefix("rect ") {
            let (width, height) = rest.split_once('x').ok_or(ParseError::SyntaxError)?;
            let width = width.parse()?;
            let height = height.parse()?;
            Self::Rect(width, height)
        } else if let Some(rest) = s.strip_prefix("rotate row y=") {
            let (row, steps) = rest.split_once(" by ").ok_or(ParseError::SyntaxError)?;
            let row = row.parse()?;
            let steps = steps.parse()?;
            Self::RotateRow(row, steps)
        } else if let Some(rest) = s.strip_prefix("rotate column x=") {
            let (col, steps) = rest.split_once(" by ").ok_or(ParseError::SyntaxError)?;
            let col = col.parse()?;
            let steps = steps.parse()?;
            Self::RotateColumn(col, steps)
        } else {
            return Err(ParseError::SyntaxError);
        })
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    input.lines().map(str::parse).collect()
}

#[aoc(day8, part1)]
fn part_1(instructions: &[Instruction]) -> usize {
    let grid: [[bool; 50]; 6] = execute(instructions);
    grid.into_iter()
        .map(|row| row.into_iter().map(usize::from).sum::<usize>())
        .sum()
}

#[aoc(day8, part2)]
fn part_2(instructions: &[Instruction]) -> String {
    let grid: [[bool; 50]; 6] = execute(instructions);
    let mut display = String::new();
    for (row1, row2) in grid.iter().step_by(2).zip(grid.iter().skip(1).step_by(2)) {
        display.push('\n');
        for (&val1, &val2) in row1.iter().zip(row2) {
            display.push(match (val1, val2) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            });
        }
    }
    display
}

fn execute<const R: usize, const C: usize>(instructions: &[Instruction]) -> [[bool; C]; R] {
    let mut grid = [[false; C]; R];
    for &instruction in instructions {
        match instruction {
            Instruction::Rect(width, height) => {
                for row in &mut grid[..height] {
                    row[..width].fill(true);
                }
            }
            Instruction::RotateRow(row, steps) => {
                grid[row].rotate_right(steps);
            }
            Instruction::RotateColumn(col, steps) => {
                let mut column = grid.map(|row| row[col]);
                column.rotate_right(steps);
                for (row, value) in grid.iter_mut().zip(column) {
                    row[col] = value;
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        rect 3x2\n\
        rotate column x=1 by 1\n\
        rotate row y=0 by 4\n\
        rotate column x=1 by 1\
    ";

    #[test]
    fn test_parse() {
        let result = parse(EXAMPLE).unwrap();
        let expected = [
            Instruction::Rect(3, 2),
            Instruction::RotateColumn(1, 1),
            Instruction::RotateRow(0, 4),
            Instruction::RotateColumn(1, 1),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute() {
        const X: bool = true;
        const O: bool = false;
        let instructions = parse(EXAMPLE).unwrap();
        let grid = execute(&instructions);
        let expected = [
            [O, X, O, O, X, O, X],
            [X, O, X, O, O, O, O],
            [O, X, O, O, O, O, O],
        ];
        assert_eq!(grid, expected);
    }
}
