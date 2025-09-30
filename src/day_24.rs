use std::fmt::{Display, Write};

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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Wall => f.write_char('#'),
            Self::Open => f.write_char('.'),
            Self::Target(n) => f.write_char((b'0' + n) as char),
        }
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
    let locations = (b'0'..=b'9')
        .map(|ch| Tile::Target(ch - b'0'))
        .filter_map(|t| grid.find_pos(|&t1| t1 == t))
        .collect::<Vec<_>>();
    let distances: Vec<Vec<_>> = locations
        .iter()
        .map(|l1| {
            locations
                .iter()
                .map(|l2| (l1 != l2).then(|| grid.shortest_path(*l1, *l2)).flatten())
                .collect()
        })
        .collect();
    let mut remaining = (1..locations.len()).collect::<Vec<_>>();
    let mut min_distance = usize::MAX;
    permute(&mut remaining, &mut |perm: &[usize]| {
        let total = perm
            .iter()
            .fold((0, Some(0)), |(source, dist), &target| {
                (
                    target,
                    dist.and_then(|d1| distances[source][target].map(|d2| d1 + d2)),
                )
            })
            .1;
        if let Some(dist) = total {
            min_distance = min_distance.min(dist);
        }
    });
    min_distance
}

#[aoc(day24, part2)]
fn part_2(grid: &Grid<Tile>) -> usize {
    let locations = (b'0'..=b'9')
        .map(|ch| Tile::Target(ch - b'0'))
        .filter_map(|t| grid.find_pos(|&t1| t1 == t))
        .collect::<Vec<_>>();
    let distances: Vec<Vec<_>> = locations
        .iter()
        .map(|l1| {
            locations
                .iter()
                .map(|l2| (l1 != l2).then(|| grid.shortest_path(*l1, *l2)).flatten())
                .collect()
        })
        .collect();
    let mut remaining = (1..locations.len()).collect::<Vec<_>>();
    let mut min_distance = usize::MAX;
    permute(&mut remaining, &mut |perm: &[usize]| {
        let total = perm.iter().fold((0, Some(0)), |(source, dist), &target| {
            (
                target,
                dist.and_then(|d1| distances[source][target].map(|d2| d1 + d2)),
            )
        });
        if let (last, Some(dist)) = total
            && let Some(closing) = distances[last][0]
        {
            min_distance = min_distance.min(dist + closing);
        }
    });
    min_distance
}
