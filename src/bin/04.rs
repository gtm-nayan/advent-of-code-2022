#![feature(slice_split_at_unchecked)]
use std::iter::from_fn;

use memchr::memchr;

fn split_and_parse(needle: u8, input: &[u8]) -> (u8, &[u8]) {
    let loc = unsafe { memchr(needle, input).unwrap_unchecked() };

    let (head, mut tail) = unsafe { input.split_at_unchecked(loc) };

    tail = unsafe { tail.split_at_unchecked(1).1 };

    let head = head
        .iter()
        .rev()
        .enumerate()
        .map(|(i, ch)| (ch - b'0') * 10_u8.pow(i as u32))
        .sum();

    (head, tail)
}

pub fn pairs(input: &str) -> impl Iterator<Item = (u8, u8, u8, u8)> + '_ {
    let mut slice = input.as_bytes();

    from_fn(move || {
        if slice.is_empty() {
            return None;
        }

        let (first_elf_start, tail) = split_and_parse(b'-', slice);
        let (first_elf_end, tail) = split_and_parse(b',', tail);
        let (second_elf_start, tail) = split_and_parse(b'-', tail);
        let (second_elf_end, tail) = split_and_parse(b'\n', tail);
        slice = tail;

        Some((
            first_elf_start,
            first_elf_end,
            second_elf_start,
            second_elf_end,
        ))
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        pairs(input)
            .filter(|(a, b, c, d)| (a <= c) && (d <= b) || (c <= a) && (b <= d))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        pairs(input)
            .filter(|(a, b, c, d)| a.max(c) <= b.min(d))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
