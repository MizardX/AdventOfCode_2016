use std::collections::VecDeque;
use std::fmt::Write;

use smallvec::SmallVec;

/// Iterator over hashes of `<seed> + <pos>`, with incrementing `pos`
struct HashGenerator {
    ctx: md5::Context,
    buf: String,
    pos: u64,
    repeat_hashings: usize,
}

impl HashGenerator {
    fn new(input: &[u8], repeat_hashings: usize) -> Self {
        let mut ctx = md5::Context::new();
        ctx.consume(input);
        let buf = String::new();
        let pos = 0;
        Self {
            ctx,
            buf,
            pos,
            repeat_hashings,
        }
    }
}

impl Iterator for HashGenerator {
    type Item = [u8; 16];

    fn next(&mut self) -> Option<Self::Item> {
        const HEX: &[u8] = b"0123456789abcdef";

        self.buf.clear();
        write!(&mut self.buf, "{}", self.pos).unwrap();
        self.pos += 1;

        let mut ctx = self.ctx.clone();
        ctx.consume(self.buf.as_bytes());
        let mut hash = ctx.finalize().0;

        for _ in 0..self.repeat_hashings {
            let mut hex = [0; 32];
            for (i, nib) in Nibs::new(&hash).enumerate() {
                hex[i] = HEX[nib as usize];
            }
            let mut ctx = md5::Context::new();
            ctx.consume(hex);
            hash = ctx.finalize().0;
        }

        Some(hash)
    }
}

/// Iterator over the "nibs" (4-bit chunks) of a `[u8]`
struct Nibs<'a> {
    data: &'a [u8],
    index: usize,
    low: bool,
}

impl<'a> Nibs<'a> {
    const fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            index: 0,
            low: false,
        }
    }
}

impl Iterator for Nibs<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&byte) = self.data.get(self.index) {
            if self.low {
                self.index += 1;
                self.low = false;
                Some(byte & 0xF)
            } else {
                self.low = true;
                Some(byte >> 4)
            }
        } else {
            None
        }
    }
}

/// Iterator over chunks of consecutive duplicate items.
struct Chunked<I, T> {
    source: I,
    current: T,
    count: usize,
}

impl<I, T> Chunked<I, T>
where
    I: Iterator<Item = T>,
    T: Default,
{
    fn new(source: I) -> Self {
        Self {
            source,
            current: T::default(),
            count: 0,
        }
    }
}

impl<I, T> Iterator for Chunked<I, T>
where
    I: Iterator<Item = T>,
    T: Default + Copy + PartialEq,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        for next in self.source.by_ref() {
            if next == self.current {
                self.count += 1;
            } else {
                let item = (self.count, self.current);
                self.current = next;
                self.count = 1;
                if item.0 > 0 {
                    return Some(item);
                }
            }
        }
        if self.count > 0 {
            let item = (self.count, self.current);
            self.current = T::default();
            self.count = 0;
            return Some(item);
        }
        None
    }
}

/// Iterator over the password characters, in accordance to the puzzle description
struct PasswordGenerator {
    index: usize,
    generator: HashGenerator,
    window: VecDeque<(Option<u8>, SmallVec<[u8; 5]>)>,
    verifications: [u16; 16],
}

impl PasswordGenerator {
    fn new(salt: &[u8], repeat_hashings: usize) -> Self {
        let mut generator = HashGenerator::new(salt, repeat_hashings);
        let mut verifications = [0; 16];
        let window = generator
            .by_ref()
            .take(1000)
            .map(|hash| Self::hash_into_nibs35(hash, &mut verifications))
            .collect();
        Self {
            index: 0,
            generator,
            window,
            verifications,
        }
    }

    fn hash_into_nibs35(
        hash: [u8; 16],
        verifications: &mut [u16; 16],
    ) -> (Option<u8>, SmallVec<[u8; 5]>) {
        let mut nib3 = None;
        let mut nibs5 = SmallVec::<[u8; 5]>::new();
        for (size, nib) in Chunked::new(Nibs::new(&hash)) {
            if nib3.is_none() && size >= 3 {
                nib3 = Some(nib);
            }
            if size >= 5 {
                nibs5.push(nib);
                verifications[nib as usize] += 1;
            }
        }
        (nib3, nibs5)
    }
}

impl Iterator for PasswordGenerator {
    type Item = (u8, usize);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (nib3, nibs5) = self.window.pop_front().unwrap();
            for nib5 in nibs5 {
                self.verifications[nib5 as usize] -= 1;
            }
            self.window.push_back(
                self.generator
                    .next()
                    .map(|hash| Self::hash_into_nibs35(hash, &mut self.verifications))
                    .unwrap(),
            );
            self.index += 1;
            if let Some(nib3) = nib3
                && self.verifications[nib3 as usize] > 0
            {
                return Some((nib3, self.index - 1));
            }
        }
    }
}

#[aoc(day14, part1)]
fn part_1(input: &[u8]) -> usize {
    PasswordGenerator::new(input, 0).nth(63).unwrap().1
}

#[aoc(day14, part2)]
fn part_2(input: &[u8]) -> usize {
    PasswordGenerator::new(input, 2016).nth(63).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let result = part_1(b"abc");
        assert_eq!(result, 22_728);
    }

    #[test]
    fn test_part_2() {
        let result = part_2(b"abc");
        assert_eq!(result, 22_551);
    }
}
