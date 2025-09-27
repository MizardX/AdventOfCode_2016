use std::collections::{HashSet, VecDeque};
use std::num::ParseIntError;

struct Maze {
    seed: u64,
}

impl Maze {
    const fn new(seed: u64) -> Self {
        Self { seed }
    }

    const fn is_open(&self, x: u64, y: u64) -> bool {
        ((3 + x + 2 * y) * x + (1 + y) * y + self.seed).count_ones() & 1 == 0
    }

    fn neighbors(&self, (x, y): (u64, u64), dist: u64, queue: &mut VecDeque<(u64, u64, u64)>) {
        for (x1, y1) in [
            x.checked_sub(1).map(|x1| (x1, y)),
            y.checked_sub(1).map(|y1| (x, y1)),
            Some((x + 1, y)),
            Some((x, y + 1)),
        ]
        .iter()
        .filter_map(|pt| pt.filter(|&(x, y)| self.is_open(x, y)))
        {
            queue.push_back((x1, y1, dist));
        }
    }

    fn find_path(&self, source: (u64, u64), dest: (u64, u64)) -> u64 {
        let mut visited = HashSet::new();
        let mut pending = VecDeque::new();
        pending.push_back((source.0, source.1, 0));
        while let Some((x, y, dist)) = pending.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }
            if (x, y) == dest {
                return dist;
            }
            self.neighbors((x, y), dist + 1, &mut pending);
        }
        0
    }

    fn find_in_range(&self, source: (u64, u64), max_dist: u64) -> usize {
        let mut visited = HashSet::new();
        let mut pending = VecDeque::new();
        pending.push_back((source.0, source.1, 0));
        while let Some((x, y, dist)) = pending.pop_front() {
            if !visited.insert((x, y)) {
                continue;
            }
            if dist < max_dist {
                self.neighbors((x, y), dist + 1, &mut pending);
            }
        }
        visited.len()
    }
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Result<Maze, ParseIntError> {
    Ok(Maze::new(input.parse()?))
}

#[aoc(day13, part1)]
fn part_1(maze: &Maze) -> u64 {
    maze.find_path((1, 1), (31, 39))
}

#[aoc(day13, part2)]
fn part_2(maze: &Maze) -> usize {
    maze.find_in_range((1, 1), 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walls() {
        let maze = Maze::new(10);

        let mut display = String::with_capacity(77);
        for y in 0..=6 {
            for x in 0..=9 {
                display.push(if maze.is_open(x, y) { '.' } else { '#' });
            }
            display.push('\n');
        }

        assert_eq!(
            display,
            "\
            .#.####.##\n\
            ..#..#...#\n\
            #....##...\n\
            ###.#.###.\n\
            .##..#..#.\n\
            ..##....#.\n\
            #...##.###\n\
            "
        );
    }

    #[test]
    fn test_find_path() {
        let maze = Maze::new(10);
        let result = maze.find_path((1, 1), (7, 4));
        assert_eq!(result, 11);
    }
}
