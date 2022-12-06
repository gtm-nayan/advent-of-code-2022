#![feature(iter_next_chunk)]
use std::collections::VecDeque;

#[derive(Debug)]
struct Buf {
    inner: VecDeque<u8>,
    last_dup: Option<u8>,
}

impl Buf {
    fn new(it: impl IntoIterator<Item = u8>) -> Self {
        let mut me = Self {
            inner: VecDeque::from_iter(it),
            last_dup: None,
        };
        me.last_dup = me.find_dup().copied();
        me
    }

    fn advance(&mut self, item: u8) -> Option<u8> {
        let evicted = self.inner.pop_front();
        self.inner.push_back(item);

        if evicted == self.last_dup {
            self.last_dup = self.find_dup().copied()
        }

        evicted
    }

    fn find_dup(&self) -> Option<&u8> {
        self.inner
            .iter()
            .rev()
            .enumerate()
            .find_map(|(idx, item)| self.inner.iter().rev().skip(idx + 1).find(|i| item.eq(i)))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bytes = input.bytes();

    let mut buf = Buf::new(
        bytes
            .by_ref()
            .next_chunk::<4>()
            .expect("Bad input: Not enough bytes to start"),
    );

    for (b, count) in bytes.zip(5..) {
        buf.advance(b);

        if buf.last_dup.is_none() {
            return Some(count);
        }
    }
    None
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
