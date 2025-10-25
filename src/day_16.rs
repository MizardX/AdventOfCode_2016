struct DragonCurve {
    root: Vec<bool>,
}

impl DragonCurve {
    fn get(&self, index: usize) -> bool {
        // The sequence is made up in three parts interleved:
        // 1. Root value
        // 3. Inverted root value
        // 2. Curve sequence
        // The root value and the inverted root value repeat every (2n+2) characters.
        // Every (n+1)'th character is from the dragon curve sequence.
        let root_len = self.root.len();
        let root_index = index % (2 * root_len + 2);
        if root_index < root_len {
            self.root[root_index]
        } else if root_index > root_len && root_index <= 2 * root_len {
            !self.root[2 * root_len - root_index]
        } else {
            let n = (index + 1) / (root_len + 1);
            Self::dragon_curve(n)
        }
    }

    /// Index into the dragon curve sequence
    #[inline]
    const fn dragon_curve(n: usize) -> bool {
        // Get the bit above the lowest 1-bit of the index. This is equivalent to the dragon curve sequence.
        // ref: https://rosettacode.org/wiki/Dragon_curve
        (n >> (n.trailing_zeros() + 1)) & 1 == 1
    }

    fn get_range_xnor(&self, start: usize, end: usize) -> bool {
        let len = self.root.len();
        let cycle_len = 2 * len + 2;
        let cycles_start = start.next_multiple_of(cycle_len);
        let cycles_end = end.next_multiple_of(cycle_len).saturating_sub(cycle_len);
        let mut result = true;
        if cycles_end <= cycles_start {
            // No full cycle within the range, fall back to single samples
            for ix in start..end {
                result ^= !self.get(ix);
            }
            return result;
        }
        for ix in start..cycles_start {
            result ^= !self.get(ix);
        }

        // Within each full cycle, the root and inverted root will almost
        // cancel eachother out.
        // It is just the parity of it's length that affects the result.
        // Of course, the dragon curve padding still matters
        let dragon_start = (cycles_start + len + 1) / (len + 1);
        let dragon_count = (cycles_end - cycles_start) / (len + 1); // even
        if dragon_count & 2 == 2 && len & 1 == 1 {
            result ^= true;
        }
        for ix in dragon_start..dragon_start + dragon_count {
            result ^= !Self::dragon_curve(ix);
        }

        for ix in cycles_end..end {
            result ^= !self.get(ix);
        }
        result
    }
}

#[aoc_generator(day16)]
fn parse(input: &[u8]) -> DragonCurve {
    let root = input.iter().map(|&ch| ch == b'1').collect();
    DragonCurve { root }
}

#[aoc(day16, part1)]
fn part_1(curve: &DragonCurve) -> String {
    checksum(curve, 272)
}

#[aoc(day16, part2)]
fn part_2(curve: &DragonCurve) -> String {
    checksum(curve, 35_651_584)
}

fn checksum(curve: &DragonCurve, disk_len: usize) -> String {
    let chunk_size = 1 << disk_len.trailing_zeros();
    let mut checksum = String::new();
    for i in (0..disk_len).step_by(chunk_size) {
        let value = curve.get_range_xnor(i, i + chunk_size);
        checksum.push(if value { '1' } else { '0' });
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve() {
        let curve = parse(b"10000");

        let result = (0..23)
            .map(|i| if curve.get(i) { b'1' } else { b'0' })
            .collect::<Vec<_>>();

        assert_eq!(result, b"10000011110010000111110");
    }

    #[test]
    fn test_checksum() {
        let curve = parse(b"10000");

        let result = checksum(&curve, 20);

        assert_eq!(result, "01100");
    }
}
