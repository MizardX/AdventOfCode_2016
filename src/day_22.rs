use std::collections::VecDeque;
use std::num::ParseIntError;
use std::ops::{Index, IndexMut};
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
    let target_pos = (0, grid.cols - 1);
    let front_of_target = (target_pos.0, target_pos.1 - 1);
    let goal_pos = (0, 0);
    let move_empty_to_front_of_target = grid.shortest_path(empty_pos, front_of_target);
    let move_target_to_goal = grid.shortest_path(target_pos, goal_pos);
    move_empty_to_front_of_target + 5 * (move_target_to_goal - 1) + 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Blocker,
    Empty,
    Slidable,
}

#[derive(Debug, Clone)]
struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl TryFrom<&[NetworkNode]> for Grid<Tile> {
    type Error = ();

    fn try_from(value: &[NetworkNode]) -> Result<Self, Self::Error> {
        let rows = value.iter().map(|n| n.row).max().ok_or(())? + 1;
        let cols = value.iter().map(|n| n.col).max().ok_or(())? + 1;
        let mut data = vec![Tile::Slidable; rows * cols];
        let min_size = value.iter().map(|n| n.size).min().ok_or(())?;
        for node in value {
            data[cols * node.row + node.col] = match node {
                NetworkNode { used: 0, .. } => Tile::Empty,
                NetworkNode { used, .. } if *used > min_size => Tile::Blocker,
                _ => Tile::Slidable,
            }
        }
        Ok(Self { data, rows, cols })
    }
}

impl<T> Grid<T> {
    fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Copy,
    {
        let data = vec![T::default(); rows * cols];
        Self { data, rows, cols }
    }

    fn find_pos<P>(&self, predicate: P) -> Option<(usize, usize)>
    where
        P: FnMut(&T) -> bool,
    {
        #[allow(clippy::cast_possible_truncation)]
        self.data
            .iter()
            .position(predicate)
            .map(|index| (index / self.cols, index % self.cols))
    }
}

impl Grid<Tile> {
    fn shortest_path(&self, source: (usize, usize), target: (usize, usize)) -> usize {
        let mut visited = Grid::<bool>::new(self.rows, self.cols);
        let mut pending = VecDeque::new();
        pending.push_back(source);
        let mut dist = 0;
        while !pending.is_empty() {
            for _ in 0..pending.len() {
                let pos = pending.pop_front().unwrap();
                if visited[pos] {
                    continue;
                }
                visited[pos] = true;
                if pos == target {
                    return dist;
                }
                self.enqueue_neighbors(pos, &mut pending);
            }
            dist += 1;
        }
        0
    }

    fn enqueue_neighbors(&self, pos: (usize, usize), queue: &mut VecDeque<(usize, usize)>) {
        queue.extend(
            [
                pos.0.checked_sub(1).map(|r1| (r1, pos.1)),
                pos.1.checked_sub(1).map(|c1| (pos.0, c1)),
                (pos.0 + 1 < self.rows).then_some((pos.0 + 1, pos.1)),
                (pos.1 + 1 < self.cols).then_some((pos.0, pos.1 + 1)),
            ]
            .into_iter()
            .flatten()
            .filter(|&pos1| self[pos1] != Tile::Blocker),
        );
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
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
        assert_eq!(grid.rows, 3);
        assert_eq!(grid.cols, 3);
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
