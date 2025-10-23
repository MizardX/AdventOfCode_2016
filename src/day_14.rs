use std::collections::VecDeque;
use std::fmt::Write;

use smallvec::SmallVec;

/// Iterator over hashes of `<seed> + <pos>`, with incrementing `pos`
struct NibbleGenerator {
    ctx: md5::Context,
    buf: String,
    pos: u64,
    repeat_hashings: usize,
}

impl NibbleGenerator {
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

    fn count_nibbles(hash: [u8; 16]) -> (Option<u8>, SmallVec<[u8; 5]>) {
        let mut count = 0;
        let mut prev = 99;
        let mut nib3 = None;
        let mut nibs5 = SmallVec::new();
        for nib in hash.into_iter().flat_map(|b| [b >> 4, b & 0xF]) {
            if nib == prev {
                count += 1;
            } else {
                if count >= 3 && nib3.is_none() {
                    nib3 = Some(prev);
                }
                if count >= 5 {
                    nibs5.push(prev);
                }
                prev = nib;
                count = 1;
            }
        }
        if count >= 3 && nib3.is_none() {
            nib3 = Some(prev);
        }
        if count >= 5 {
            nibs5.push(prev);
        }
        (nib3, nibs5)
    }

    fn into_hex(hash: [u8; 16]) -> [u8; 32] {
        const HEX: &[u8] = b"0123456789abcdef";
        let mut hex = [0; 32];
        for (i, &byte) in hash.iter().enumerate() {
            let (hi, lo) = (byte >> 4, byte & 0xF);
            hex[2 * i] = HEX[hi as usize];
            hex[2 * i + 1] = HEX[lo as usize];
        }
        hex
    }
}

impl Iterator for NibbleGenerator {
    type Item = (Option<u8>, SmallVec<[u8; 5]>);

    fn next(&mut self) -> Option<Self::Item> {

        self.buf.clear();
        write!(&mut self.buf, "{}", self.pos).unwrap();
        self.pos += 1;

        let mut ctx = self.ctx.clone();
        ctx.consume(self.buf.as_bytes());
        let mut hash = ctx.finalize().0;

        for _ in 0..self.repeat_hashings {
            let mut ctx = md5::Context::new();
            ctx.consume(Self::into_hex(hash));
            hash = ctx.finalize().0;
        }

        Some(Self::count_nibbles(hash))
    }
}

/// Iterator over the password characters, in accordance to the puzzle description
struct PasswordGenerator {
    index: usize,
    generator: NibbleGenerator,
    window: VecDeque<(Option<u8>, SmallVec<[u8; 5]>)>,
    verifications: [u16; 16],
}

impl PasswordGenerator {
    fn new(salt: &[u8], repeat_hashings: usize) -> Self {
        let mut generator = NibbleGenerator::new(salt, repeat_hashings);
        let mut verifications = [0; 16];
        let window = generator
            .by_ref()
            .take(1000)
            .inspect(|(_, nibs5)| {
                for &nib5 in nibs5 {
                    verifications[nib5 as usize] += 1;
                }
            })
            .collect();
        Self {
            index: 0,
            generator,
            window,
            verifications,
        }
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

            let (next_nib3, next_nibs5) = self.generator.next().unwrap();
            for &nib5 in &next_nibs5 {
                self.verifications[nib5 as usize] += 1;
            }
            self.window.push_back((next_nib3, next_nibs5));

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
