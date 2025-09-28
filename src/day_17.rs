use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn to_u8(self) -> u8 {
        match self {
            Self::Up => b'U',
            Self::Down => b'D',
            Self::Left => b'L',
            Self::Right => b'R',
        }
    }

    const fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    fn apply(self, (r, c): (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => r.checked_sub(1).map(|r1| (r1, c)),
            Self::Right => (c + 1 < 4).then_some((r, c + 1)),
            Self::Down => (r + 1 < 4).then_some((r + 1, c)),
            Self::Left => c.checked_sub(1).map(|c1| (r, c1)),
        }
    }
}

#[derive(Clone)]
struct State {
    ctx: md5::Context,
    steps: String,
    pos: (usize, usize),
}

impl State {
    fn new(seed: &[u8]) -> Self {
        let mut ctx = md5::Context::new();
        ctx.consume(seed);
        Self {
            ctx,
            steps: String::new(),
            pos: (0, 0),
        }
    }

    fn make_move(&self, dir: Dir) -> Option<Self> {
        let pos = dir.apply(self.pos)?;
        let mut ctx = self.ctx.clone();
        ctx.consume([dir.to_u8()]);
        let mut steps = self.steps.clone();
        steps.push(dir.to_u8() as char);
        Some(Self { ctx, steps, pos })
    }

    fn get_hash(&self) -> [u8; 16] {
        self.ctx.clone().finalize().0
    }

    fn get_open_doors(&self) -> [bool; 4] {
        let [up_down, left_right, ..] = self.get_hash();
        [
            up_down >> 4 >= 0xb,
            up_down & 0xf >= 0xb,
            left_right >> 4 >= 0xb,
            left_right & 0xf >= 0xb,
        ]
    }

    fn enqueue_moves(&self, queue: &mut VecDeque<Self>) {
        for (open, dir) in self.get_open_doors().into_iter().zip(Dir::all()) {
            if open && let Some(next) = self.make_move(dir) {
                queue.push_back(next);
            }
        }
    }

    fn is_goal(&self) -> bool {
        self.pos == (3, 3)
    }
}

#[aoc(day17, part1)]
fn part_1(input: &[u8]) -> String {
    let start_state = State::new(input);
    let mut queue = VecDeque::new();
    queue.push_back(start_state);

    while let Some(state) = queue.pop_front() {
        if state.is_goal() {
            return state.steps;
        }
        state.enqueue_moves(&mut queue);
    }
    String::new()
}

#[aoc(day17, part2)]
fn part_2(input: &[u8]) -> usize {
    let start_state = State::new(input);
    let mut queue = VecDeque::new();
    queue.push_back(start_state);

    let mut longest = 0;
    while let Some(state) = queue.pop_front() {
        if state.is_goal() {
            longest = state.steps.len();
        } else {
            state.enqueue_moves(&mut queue);
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part_1_false_start() {
        let state = State::new(b"hijkl");
        assert_eq!(state.get_open_doors(), [true, true, true, false]);

        let state_down = state.make_move(Dir::Down).unwrap();
        assert_eq!(state_down.get_open_doors(), [true, false, true, true]);

        let state_down_right = state_down.make_move(Dir::Right).unwrap();
        assert_eq!(
            state_down_right.get_open_doors(),
            [false, false, false, false]
        );

        let state_down_up = state_down.make_move(Dir::Up).unwrap();
        assert_eq!(state_down_up.get_open_doors(), [false, false, false, true]);

        let state_down_up_right = state_down_up.make_move(Dir::Right).unwrap();
        assert_eq!(
            state_down_up_right.get_open_doors(),
            [false, false, false, false]
        );
    }

    #[test_case(b"ihgpwlah" => "DDRRRD")]
    #[test_case(b"kglvqrro" => "DDUDRLRRUDRD")]
    #[test_case(b"ulqzkmiv" => "DRURDRUDDLLDLUURRDULRLDUUDDDRR")]
    fn test_part_1(input: &[u8]) -> String {
        part_1(input)
    }

    #[test_case(b"ihgpwlah" => 370)]
    #[test_case(b"kglvqrro" => 492)]
    #[test_case(b"ulqzkmiv" => 830)]
    fn test_part_2(input: &[u8]) -> usize {
        part_2(input)
    }
}
