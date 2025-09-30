use std::collections::VecDeque;
use std::mem::MaybeUninit;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self
    where
        T: Default + Copy,
    {
        let data = vec![T::default(); rows * cols];
        Self { data, rows, cols }
    }

    pub fn find_pos<P>(&self, predicate: P) -> Option<(usize, usize)>
    where
        P: FnMut(&T) -> bool,
    {
        self.data
            .iter()
            .position(predicate)
            .map(|index| (index / self.cols, index % self.cols))
    }
}

impl<T: TilePath> Grid<T> {
    pub fn shortest_path(&self, source: (usize, usize), target: (usize, usize)) -> Option<usize> {
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
                    return Some(dist);
                }
                self.enqueue_neighbors(pos, &mut pending);
            }
            dist += 1;
        }
        None
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
            .filter(|&pos1| self[pos1].is_passable()),
        );
    }
}

#[derive(Debug, Error)]
pub enum GridParseError<E> {
    #[error("Invalid tile")]
    InvalidTile(#[from] E),
    #[error("Not all lines where the same length")]
    ShapeError,
}

impl<T, E> FromStr for Grid<T>
where
    T: TryFrom<u8, Error = E>,
{
    type Err = GridParseError<E>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.lines().next().unwrap().len();
        let rows = s.lines().count();
        let data = (0..rows * cols)
            .map(|_| MaybeUninit::uninit())
            .collect::<Vec<_>>();
        let mut grid = Grid { data, rows, cols };
        for (r, line) in s.lines().enumerate() {
            if line.len() != cols {
                return Err(GridParseError::ShapeError);
            }
            for (c, cell) in line.bytes().enumerate() {
                let tile: T = cell.try_into()?;
                grid[(r, c)] = MaybeUninit::new(tile);
            }
        }
        Ok(unsafe { std::mem::transmute(grid) })
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

pub trait TilePath {
    fn is_passable(&self) -> bool;
}

pub fn permute<T, F: FnMut(&[T])>(items: &mut [T], callback: &mut F) {
    fn inner<T, F: FnMut(&[T])>(items: &mut [T], index: usize, callback: &mut F) {
        if index == items.len() {
            callback(items);
            return;
        }
        for swap_with in index..items.len() {
            items.swap(index, swap_with);
            inner(items, index + 1, callback);
            items.swap(index, swap_with);
        }
    }
    inner(items, 0, callback);
}
