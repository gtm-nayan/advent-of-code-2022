#![feature(array_chunks)]
use std::ops::Add;
const ROCK: u8 = b'A';
const PAPER: u8 = b'B';
const SCISSOR: u8 = b'C';

const _ROCK: u32 = 1;
const _PAPER: u32 = 2;
const _SCISSOR: u32 = 3;

fn take_lines(input: &str) -> impl Iterator<Item = (u8, u8)> + '_ {
    input
        .as_bytes()
        .array_chunks::<4>()
        .map(|&[a, _, b, _]| (a, b))
}

pub fn part_one(input: &str) -> Option<u32> {
    take_lines(input)
        .map(|(opp, me)| {
            match (me, opp) {
                // Draws
                (b'X', ROCK) => 3 + _ROCK,
                (b'Y', PAPER) => 3 + _PAPER,
                (b'Z', SCISSOR) => 3 + _SCISSOR,

                // Wins
                (b'X', SCISSOR) => 6 + _ROCK,
                (b'Y', ROCK) => 6 + _PAPER,
                (b'Z', PAPER) => 6 + _SCISSOR,

                // Losses
                (b'X', PAPER) => _ROCK,
                (b'Y', SCISSOR) => _PAPER,
                (b'Z', ROCK) => _SCISSOR,

                _ => unreachable!(),
            }
        })
        .reduce(Add::add)
}

pub fn part_two(input: &str) -> Option<u32> {
    take_lines(input)
        .map(|(opp, suggestion)| {
            match (suggestion, opp) {
                // Draws
                (b'Y', ROCK) => 3 + _ROCK,
                (b'Y', PAPER) => 3 + _PAPER,
                (b'Y', SCISSOR) => 3 + _SCISSOR,

                // Wins
                (b'Z', SCISSOR) => 6 + _ROCK,
                (b'Z', ROCK) => 6 + _PAPER,
                (b'Z', PAPER) => 6 + _SCISSOR,

                // Losses
                (b'X', PAPER) => _ROCK,
                (b'X', SCISSOR) => _PAPER,
                (b'X', ROCK) => _SCISSOR,

                _ => unreachable!(),
            }
        })
        .reduce(Add::add)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
