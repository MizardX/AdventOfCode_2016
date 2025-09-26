#[aoc(day7, part1)]
fn part_1(input: &[u8]) -> usize {
    input
        .split(|&ch| ch == b'\n')
        .filter(|&line| supports_tls(line))
        .count()
}

fn supports_tls(address: &[u8]) -> bool {
    let mut local_ok = false;
    for (i, chunk) in address.split(|&ch| matches!(ch, b'[' | b']')).enumerate() {
        if i & 1 == 0 {
            local_ok = local_ok || chunk.windows(4).any(is_abba);
        } else if chunk.windows(4).any(is_abba) {
            return false;
        }
    }
    local_ok
}

const fn is_abba(window: &[u8]) -> bool {
    if let &[a, b, b1, a1] = window {
        a != b && a == a1 && b == b1
    } else {
        false
    }
}

#[aoc(day7, part2)]
fn part_2(input: &[u8]) -> usize {
    input
        .split(|&ch| ch == b'\n')
        .filter(|&line| supports_ssl(line))
        .count()
}

fn supports_ssl(address: &[u8]) -> bool {
    let mut seen_local = [0_u32; 26];
    let mut seen_super = [0_u32; 26];
    for (i, chunk) in address.split(|&ch| matches!(ch, b'[' | b']')).enumerate() {
        if i & 1 == 0 {
            for (a, b) in chunk.windows(3).filter_map(is_aba) {
                if seen_super[(b - b'a') as usize] & 1 << (a - b'a') != 0 {
                    return true;
                }
                seen_local[(a - b'a') as usize] |= 1 << (b - b'a');
            }
        } else {
            for (a, b) in chunk.windows(3).filter_map(is_aba) {
                if seen_local[(b - b'a') as usize] & 1 << (a - b'a') != 0 {
                    return true;
                }
                seen_super[(a - b'a') as usize] |= 1 << (b - b'a');
            }
        }
    }
    false
}

const fn is_aba(window: &[u8]) -> Option<(u8, u8)> {
    if let &[a, b, a1] = window
        && a != b
        && a == a1
    {
        Some((a, b))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(b"abba[mnop]qrst" => true)]
    #[test_case(b"abcd[bddb]xyyx" => false)]
    #[test_case(b"aaaa[qwer]tyui" => false)]
    #[test_case(b"ioxxoj[asdfgh]zxcvbn" => true)]
    fn tests_supports_tls(address: &[u8]) -> bool {
        supports_tls(address)
    }

    #[test_case(b"aba[bab]xyz" => true)]
    #[test_case(b"xyx[xyx]xyx" => false)]
    #[test_case(b"aaa[kek]eke" => true)]
    #[test_case(b"zazbz[bzb]cdb" => true)]
    fn test_supports_ssl(address: &[u8]) -> bool {
        supports_ssl(address)
    }
}
