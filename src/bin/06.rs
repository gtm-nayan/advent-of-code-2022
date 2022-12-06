#![feature(iter_next_chunk)]
#![feature(array_windows)]
use std::collections::VecDeque;

#[derive(Debug)]
struct Buf {
    inner: VecDeque<u8>,
    last_dup: Option<u8>,
    char_count: [u8; 26],
}

impl Buf {
    fn new(it: impl IntoIterator<Item = u8>) -> Self {
        let mut char_count: [u8; 26] = Default::default();

        let mut me = Self {
            inner: VecDeque::from_iter(
                it.into_iter()
                    .inspect(|&ch| char_count[Self::char_to_idx(ch)] += 1),
            ),
            last_dup: None,
            char_count,
        };
        me.last_dup = me.find_dup().copied();
        me
    }

    fn char_to_idx(ch: u8) -> usize {
        (ch - b'a') as usize
    }

    fn get_count(&self, ch: u8) -> u8 {
        self.char_count[Self::char_to_idx(ch)]
    }

    fn dec_count(&mut self, ch: u8) {
        self.char_count[Self::char_to_idx(ch)] -= 1;
    }

    fn inc_count(&mut self, ch: u8) {
        self.char_count[Self::char_to_idx(ch)] += 1;
    }

    fn advance(&mut self, item: u8) -> Option<u8> {
        let evicted = self.inner.pop_front();
        self.inner.push_back(item);

        self.inc_count(item);
        let e = unsafe { evicted.unwrap_unchecked() };
        self.dec_count(e);

        if evicted == self.last_dup && self.get_count(e) < 2 {
            self.last_dup = self.find_dup().copied()
        }

        evicted
    }

    fn find_dup(&self) -> Option<&u8> {
        self.inner.iter().rev().find(|ch| self.get_count(**ch) > 1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    const N: usize = 4;
    input
        .as_bytes()
        .array_windows::<N>()
        .zip(N..)
        .find_map(|(a, count)| {
            for i in 1..N {
                for j in 0..i {
                    if a[i] == a[j] {
                        return None;
                    }
                }
            }
            Some(count as u32)
        })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bytes = input.bytes();

    let mut buf = Buf::new(
        bytes
            .by_ref()
            .next_chunk::<14>()
            .expect("Bad input: Not enough bytes to start"),
    );

    for (b, count) in bytes.zip(15..) {
        buf.advance(b);

        if buf.last_dup.is_none() {
            return Some(count);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
