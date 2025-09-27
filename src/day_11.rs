use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Syntax error")]
    SyntaxError,
    #[error("Invalid floor")]
    InvalidFloor,
    #[error("Invalid item")]
    InvalidItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Floor {
    First,
    Second,
    Third,
    Fourth,
}

impl TryFrom<u32> for Floor {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::First,
            1 => Self::Second,
            2 => Self::Third,
            3 => Self::Fourth,
            v => return Err(v),
        })
    }
}

impl FromStr for Floor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "first" => Self::First,
            "second" => Self::Second,
            "third" => Self::Third,
            "fourth" => Self::Fourth,
            _ => return Err(ParseError::InvalidFloor),
        })
    }
}

impl Floor {
    fn up(self) -> Option<Self> {
        Some(match self {
            Self::First => Self::Second,
            Self::Second => Self::Third,
            Self::Third => Self::Fourth,
            Self::Fourth => None?,
        })
    }

    fn down(self) -> Option<Self> {
        Some(match self {
            Self::First => None?,
            Self::Second => Self::First,
            Self::Third => Self::Second,
            Self::Fourth => Self::Third,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Item {
    Generator(usize),
    Chip(usize),
}

impl Item {
    fn try_parse(s: &str, materials: &mut HashMap<String, usize>) -> Result<Self, ParseError> {
        let rest = s.strip_prefix("a ").ok_or(ParseError::InvalidItem)?;
        Ok(
            if let Some(material) = rest.strip_suffix("-compatible microchip") {
                Self::Chip(if let Some(&index) = materials.get(material) {
                    index
                } else {
                    let index = materials.len();
                    materials.insert(material.to_string(), index);
                    index
                })
            } else if let Some(material) = rest.strip_suffix(" generator") {
                Self::Generator(if let Some(&index) = materials.get(material) {
                    index
                } else {
                    let index = materials.len();
                    materials.insert(material.to_string(), index);
                    index
                })
            } else {
                return Err(ParseError::InvalidItem);
            },
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Facility {
    materials: Vec<String>,
    items: Vec<(Item, Floor)>,
}

impl FromStr for Facility {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut facility = Self::default();
        let mut materials = HashMap::new();
        for line in s.lines() {
            let rest = line.strip_prefix("The ").ok_or(ParseError::SyntaxError)?;
            let (floor, rest) = rest
                .split_once(" floor contains ")
                .ok_or(ParseError::SyntaxError)?;
            let rest = rest.strip_suffix(".").ok_or(ParseError::SyntaxError)?;
            if rest == "nothing relevant" {
                // No items
                continue;
            }
            let floor: Floor = floor.parse()?;
            if let Some((rest, last)) = rest.split_once(" and ") {
                // 2+ items
                let mut rest = rest.trim_end_matches(','); // Oxford comma only if 3+ items
                while let Some((first, rest1)) = rest.split_once(", ") {
                    facility
                        .items
                        .push((Item::try_parse(first, &mut materials)?, floor));
                    rest = rest1;
                }
                facility
                    .items
                    .push((Item::try_parse(rest, &mut materials)?, floor));
                facility
                    .items
                    .push((Item::try_parse(last, &mut materials)?, floor));
            } else {
                // Single item
                facility
                    .items
                    .push((Item::try_parse(rest, &mut materials)?, floor));
            }
        }
        facility.items.sort_unstable();
        facility.materials.resize(materials.len(), String::new());
        for (name, index) in materials {
            facility.materials[index] = name;
        }
        Ok(facility)
    }
}

#[derive(Clone, Copy)]
struct State {
    bits: u32,
    count: usize,
    round: u8,
}

impl State {
    fn from_facility(facility: &Facility) -> Self {
        let mut bits = 0;
        for &(_, floor) in facility.items.iter().rev() {
            bits = (bits << 2) | (floor as u32);
        }
        Self {
            bits,
            count: facility.materials.len(),
            round: 0,
        }
    }

    fn elevator_floor(self) -> Floor {
        (self.bits >> (4 * self.count)).try_into().unwrap()
    }

    fn floor_of(self, item_index: usize) -> Floor {
        ((self.bits >> (2 * item_index)) & 0b11).try_into().unwrap()
    }

    const fn with_elevator(self, floor: Floor) -> Self {
        let n = self.count;
        self.with_item(2 * n, floor)
    }

    const fn with_item(self, item: usize, floor: Floor) -> Self {
        let mask = !(0b11 << (2 * item));
        let floor_mask = floor as u32;
        let bits = self.bits & mask | (floor_mask << (2 * item));
        Self { bits, ..self }
    }

    const fn with_next_round(self) -> Self {
        let round = self.round + 1;
        Self { round, ..self }
    }

    /// Transform unto equivalent state, with elements sorted by floor positions.
    /// [G1 m1; G0 m0] is equivalent to [G0 m0; G1 m1], since the elements are interchangable, as long as the pairs stay together.
    fn normalize(self) -> Self {
        let n = self.count;
        let mut pairs = [(Floor::First, Floor::First); 7];
        for (i, pair) in pairs[0..n].iter_mut().enumerate() {
            pair.0 = self.floor_of(i);
            pair.1 = self.floor_of(n + i);
        }
        pairs[..n].sort_unstable();
        let mut result = Self { bits: 0, ..self }.with_elevator(self.elevator_floor());
        for (i, pair) in pairs[..n].iter().enumerate() {
            result = result.with_item(i, pair.0);
            result = result.with_item(n + i, pair.1);
        }
        result
    }

    fn add_gen_and_chip(self) -> Self {
        let elevator = self.elevator_floor();
        let n = self.count;
        let mask = !(!0 << (2 * n));
        let generators = self.bits & mask;
        let chips = (self.bits >> (2 * n)) & mask;
        let bits = generators | (chips << (2 * n + 2));
        let count = n + 1;
        Self {
            bits,
            count,
            ..self
        }
        .with_elevator(elevator)
    }

    fn is_safe(self) -> bool {
        // Any uncoupled chips on floor with any generator, safed or not, is unsafe.
        let n = self.count;
        let mut gen_on = [false; 4];
        for i in 0..n {
            gen_on[self.floor_of(i) as usize] = true;
        }
        (0..n).all(|i| {
            let gen_floor = self.floor_of(i);
            let chip_floor = self.floor_of(n + i);
            gen_floor == chip_floor || !gen_on[chip_floor as usize]
        })
    }

    fn is_completed(self) -> bool {
        (0..=2 * self.count).all(|ix| self.floor_of(ix) == Floor::Fourth)
    }

    fn moves(self, queue: &mut VecDeque<Self>) {
        let n = self.count * 2;
        let elevator = self.elevator_floor();
        for new_floor in [elevator.up(), elevator.down()].into_iter().flatten() {
            for i in 0..n {
                if self.floor_of(i) != elevator {
                    continue;
                }
                // Move single item
                let new_state = self
                    .with_next_round()
                    .with_elevator(new_floor)
                    .with_item(i, new_floor);
                if new_state.is_safe() {
                    queue.push_back(new_state);
                }
                for j in i + 1..n {
                    if self.floor_of(j) != elevator {
                        continue;
                    }
                    // Move two items
                    let new_state = new_state.with_item(j, new_floor);
                    if new_state.is_safe() {
                        queue.push_back(new_state);
                    }
                }
            }
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elevator = self.elevator_floor() as u32;
        let n = self.count;
        write!(f, "State({}; ", self.round)?;
        for floor in 0..4 {
            if floor > 0 {
                write!(f, "; ")?;
            }
            let mut sep = if floor == elevator {
                write!(f, "[]")?;
                true
            } else {
                false
            };
            for i in 0..self.count {
                if self.floor_of(i) as u32 == floor {
                    if sep {
                        write!(f, " ")?;
                    }
                    sep = true;
                    write!(f, "G{i}")?;
                }
                if self.floor_of(n + i) as u32 == floor {
                    if sep {
                        write!(f, " ")?;
                    }
                    sep = true;
                    write!(f, "m{i}")?;
                }
            }
        }
        write!(f, ")")
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Facility, ParseError> {
    input.parse()
}

#[aoc(day11, part1)]
fn part_1(facility: &Facility) -> u8 {
    let state = State::from_facility(facility);
    solve(state)
}

#[aoc(day11, part2)]
fn part_2(facility: &Facility) -> u8 {
    let state = State::from_facility(facility)
        .add_gen_and_chip()
        .add_gen_and_chip();
    solve(state)
}

fn solve(state: State) -> u8 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(state);
    while let Some(state) = queue.pop_front() {
        let state = state.normalize();
        if !visited.insert(state) {
            continue;
        }
        if state.is_completed() {
            return state.round;
        }
        state.moves(&mut queue);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.
"
    .trim_ascii();

    #[test]
    fn test_parse() {
        let facility = parse(EXAMPLE).unwrap();
        let expected_materials = &["hydrogen", "lithium"][..];
        let expected_items = &[
            (Item::Generator(0), Floor::Second),
            (Item::Generator(1), Floor::Third),
            (Item::Chip(0), Floor::First),
            (Item::Chip(1), Floor::First),
        ][..];

        assert_eq!(facility.materials, expected_materials);
        assert_eq!(facility.items, expected_items);
    }

    #[test]
    fn test_part_1() {
        let facility = parse(EXAMPLE).unwrap();
        let result = part_1(&facility);
        assert_eq!(result, 11);
    }
}
