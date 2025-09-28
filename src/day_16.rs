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
        let len = self.root.len();
        let index1 = index % (2 * len + 2);
        if index1 < len {
            self.root[index1]
        } else if index1 > len && index1 <= 2 * len {
            !self.root[2 * len - index1]
        } else {
            // Index into the dragon curve sequence
            let n = (index + 1) / (len + 1);
            // Get the bit above the lowest 1-bit of the index. This is equivalent to the dragon curve sequence.
            // ref: https://rosettacode.org/wiki/Dragon_curve
            let least_significant_bit = n & (!n + 1);
            let above_mask = least_significant_bit * 2;
            n & above_mask != 0
        }
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
    let chunk_size = disk_len & (!disk_len + 1); // lowest bit = maximum power-of-two factor
    let mut checksum = String::new();
    for i in (0..disk_len).step_by(chunk_size) {
        // Each char of the checksum can be calculated in in sequence within each chunk, instead of by halving.
        // ((a xnor b) xnor (c xnor d)) == (((a xnor b) xnor c) xnor d)
        let mut value = true; // Starting with `true`, since `(true xnor a) == a`
        for j in i..i + chunk_size {
            value ^= !curve.get(j);
        }
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

        let result = unsafe {
            String::from_utf8_unchecked(
                (0..23)
                    .map(|i| if curve.get(i) { b'1' } else { b'0' })
                    .collect::<Vec<_>>(),
            )
        };

        assert_eq!(result, "10000011110010000111110");
    }

    #[test]
    fn test_checksum() {
        let curve = parse(b"10000");

        let result = checksum(&curve, 20);

        assert_eq!(result, "01100");
    }
}
