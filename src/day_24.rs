use thiserror::Error;

use crate::utils::{Grid, GridParseError, TilePath, permute};

#[derive(Debug, Error)]
enum TileParseError {
    #[error("Invalid tile: {0:?}")]
    InvalidTile(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Tile {
    Wall = b'#',
    Open = b'.',
    Target(u8) = b'0',
}

impl TryFrom<u8> for Tile {
    type Error = TileParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            b'#' => Self::Wall,
            b'.' => Self::Open,
            b'0'..=b'9' => Self::Target(value - b'0'),
            _ => return Err(TileParseError::InvalidTile(value as char)),
        })
    }
}

impl TilePath for Tile {
    fn is_passable(&self) -> bool {
        !matches!(self, Self::Wall)
    }
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Result<Grid<Tile>, GridParseError<TileParseError>> {
    input.parse()
}

#[aoc(day24, part1)]
fn part_1(grid: &Grid<Tile>) -> usize {
    find_shortest_distance(grid, false)
}

#[aoc(day24, part2)]
fn part_2(grid: &Grid<Tile>) -> usize {
    find_shortest_distance(grid, true)
}

fn find_shortest_distance(grid: &Grid<Tile>, close_path: bool) -> usize {
    let locations = (b'0'..=b'9')
        .map(|ch| Tile::Target(ch - b'0'))
        .filter_map(|t| grid.find_pos(|&t1| t1 == t))
        .collect::<Vec<_>>();

    let distances: Vec<Vec<_>> = locations
        .iter()
        .map(|&source| {
            let mut dists = vec![None; locations.len()];
            let is_target = |tile: &Tile| matches!(tile, Tile::Target(..));
            for (dist, &target) in grid.all_shortest_paths(source, is_target) {
                if let Tile::Target(target) = target
                    && dist > 0
                {
                    dists[target as usize] = Some(dist);
                }
            }
            dists
        })
        .collect();

    let mut remaining = (1..locations.len()).collect::<Vec<_>>();
    let mut min_distance = usize::MAX;
    permute(&mut remaining, &mut |sequence: &[usize]| {
        let mut dist = 0;
        let mut prev = 0;
        for &next in sequence {
            let Some(step) = distances[prev][next] else {
                return;
            };
            dist += step;
            prev = next;
        }
        if close_path {
            let Some(close_dist) = distances[prev][0] else {
                return;
            };
            dist += close_dist;
        }
        min_distance = min_distance.min(dist);
    });
    min_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ###########\n\
        #0.1.....2#\n\
        #.#######.#\n\
        #4.......3#\n\
        ###########\
    ";

    #[test]
    fn test_parse() {
        const WW: Tile = Tile::Wall;
        const OO: Tile = Tile::Open;
        const T0: Tile = Tile::Target(0);
        const T1: Tile = Tile::Target(1);
        const T2: Tile = Tile::Target(2);
        const T3: Tile = Tile::Target(3);
        const T4: Tile = Tile::Target(4);
        let grid = parse(EXAMPLE).unwrap();
        let exptected = [
            [WW; 11],
            [WW, T0, OO, T1, OO, OO, OO, OO, OO, T2, WW],
            [WW, OO, WW, WW, WW, WW, WW, WW, WW, OO, WW],
            [WW, T4, OO, OO, OO, OO, OO, OO, OO, T3, WW],
            [WW; 11],
        ];
        for (r, row) in exptected.into_iter().enumerate() {
            for (c, cell) in row.into_iter().enumerate() {
                assert_eq!(grid[(r, c)], cell);
            }
        }
    }

    #[test]
    fn test_part_1() {
        let grid = parse(EXAMPLE).unwrap();
        let result = part_1(&grid);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let grid = parse(EXAMPLE).unwrap();
        let result = part_2(&grid);
        assert_eq!(result, 20);
    }
}
