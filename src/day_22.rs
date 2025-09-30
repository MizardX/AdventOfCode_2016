use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::utils::{Grid, TilePath};

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NetworkNode {
    row: usize,
    col: usize,
    size: usize,
    used: usize,
}

impl FromStr for NetworkNode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s
            .strip_prefix("/dev/grid/node-x")
            .ok_or(ParseError::SyntaxError)?;
        let (col, rest) = rest.split_once("-y").ok_or(ParseError::SyntaxError)?;
        let (row, rest) = rest.split_once(' ').ok_or(ParseError::SyntaxError)?;
        let (size, rest) = rest
            .trim_ascii_start()
            .split_once("T ")
            .ok_or(ParseError::SyntaxError)?;
        let (used, _) = rest
            .trim_ascii_start()
            .split_once("T ")
            .ok_or(ParseError::SyntaxError)?;
        Ok(Self {
            col: col.parse()?,
            row: row.parse()?,
            size: size.parse()?,
            used: used.parse()?,
        })
    }
}

impl NetworkNode {
    const fn avail(self) -> usize {
        self.size - self.used
    }
}

#[aoc_generator(day22)]
fn parse(s: &str) -> Result<Vec<NetworkNode>, ParseError> {
    s.lines().skip(2).map(str::parse).collect()
}

#[aoc(day22, part1)]
fn part_1(nodes: &[NetworkNode]) -> u32 {
    let mut count = 0;
    for (i, node1) in nodes.iter().enumerate() {
        for node2 in &nodes[i + 1..] {
            if node1.used > 0 && node1.used <= node2.avail() {
                count += 1;
            }
            if node2.used > 0 && node2.used <= node1.avail() {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day22, part2)]
fn part_2(nodes: &[NetworkNode]) -> usize {
    let grid: Grid<Tile> = nodes.try_into().unwrap();
    let empty_pos = grid.find_pos(|&tile| tile == Tile::Empty).unwrap();
    let target_pos = (0, grid.cols() - 1);
    let front_of_target = (target_pos.0, target_pos.1 - 1);
    let goal_pos = (0, 0);
    let move_empty_to_front_of_target = grid.shortest_path(empty_pos, front_of_target).unwrap();
    let move_target_to_goal = grid.shortest_path(target_pos, goal_pos).unwrap();
    move_empty_to_front_of_target + 5 * (move_target_to_goal - 1) + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    Blocker,
    Empty,
    #[default]
    Slidable,
}

impl TilePath for Tile {
    fn is_passable(&self) -> bool {
        !matches!(self, Self::Blocker)
    }
}

impl TryFrom<&[NetworkNode]> for Grid<Tile> {
    type Error = ();

    fn try_from(value: &[NetworkNode]) -> Result<Self, Self::Error> {
        let rows = value.iter().map(|n| n.row).max().ok_or(())? + 1;
        let cols = value.iter().map(|n| n.col).max().ok_or(())? + 1;
        let mut grid = Self::new(rows, cols);
        let min_size = value.iter().map(|n| n.size).min().ok_or(())?;
        for node in value {
            grid[(node.row, node.col)] = match node {
                NetworkNode { used: 0, .. } => Tile::Empty,
                NetworkNode { used, .. } if *used > min_size => Tile::Blocker,
                _ => Tile::Slidable,
            };
        }
        Ok(grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        root@ebhq-gridcenter# df -h\n\
        Filesystem            Size  Used  Avail  Use%\n\
        /dev/grid/node-x0-y0   10T    8T     2T   80%\n\
        /dev/grid/node-x0-y1   11T    6T     5T   54%\n\
        /dev/grid/node-x0-y2   32T   28T     4T   87%\n\
        /dev/grid/node-x1-y0    9T    7T     2T   77%\n\
        /dev/grid/node-x1-y1    8T    0T     8T    0%\n\
        /dev/grid/node-x1-y2   11T    7T     4T   63%\n\
        /dev/grid/node-x2-y0   10T    6T     4T   60%\n\
        /dev/grid/node-x2-y1    9T    8T     1T   88%\n\
        /dev/grid/node-x2-y2    9T    6T     3T   66%\n\
    "
    .trim_ascii();

    #[test]
    fn test_parse() {
        const fn new_node(row: usize, col: usize, size: usize, used: usize) -> NetworkNode {
            NetworkNode {
                row,
                col,
                size,
                used,
            }
        }
        let result = parse(EXAMPLE).unwrap();
        assert_eq!(
            result,
            [
                new_node(0, 0, 10, 8),
                new_node(1, 0, 11, 6),
                new_node(2, 0, 32, 28),
                new_node(0, 1, 9, 7),
                new_node(1, 1, 8, 0),
                new_node(2, 1, 11, 7),
                new_node(0, 2, 10, 6),
                new_node(1, 2, 9, 8),
                new_node(2, 2, 9, 6)
            ]
        );
    }

    #[test]
    fn test_grid() {
        let nodes = parse(EXAMPLE).unwrap();
        let grid: Grid<Tile> = nodes.as_slice().try_into().unwrap();
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
        let expected = [
            [Tile::Slidable, Tile::Slidable, Tile::Slidable],
            [Tile::Slidable, Tile::Empty, Tile::Slidable],
            [Tile::Blocker, Tile::Slidable, Tile::Slidable],
        ];
        for (r, row) in expected.into_iter().enumerate() {
            for (c, tile) in row.into_iter().enumerate() {
                assert_eq!(grid[(r, c)], tile);
            }
        }
    }

    #[test]
    fn test_part_2() {
        let nodes = parse(EXAMPLE).unwrap();
        let result = part_2(&nodes);

        assert_eq!(result, 7);
    }
}
