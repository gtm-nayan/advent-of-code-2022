#![feature(array_chunks)]
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
    let sum = take_lines(input)
        .map(|(opp, me)| {
            const MY_ROCK: u8 = b'X';
            const MY_PAPER: u8 = b'Y';
            const MY_SCISSOR: u8 = b'Z';

            match (me, opp) {
                (MY_ROCK, PAPER) => _ROCK,
                (MY_ROCK, ROCK) => 3 + _ROCK,
                (MY_ROCK, SCISSOR) => 6 + _ROCK,

                (MY_PAPER, SCISSOR) => _PAPER,
                (MY_PAPER, PAPER) => 3 + _PAPER,
                (MY_PAPER, ROCK) => 6 + _PAPER,

                (MY_SCISSOR, SCISSOR) => 3 + _SCISSOR,
                (MY_SCISSOR, PAPER) => 6 + _SCISSOR,
                (MY_SCISSOR, ROCK) => _SCISSOR,

                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = take_lines(input)
        .map(|(opp, suggestion)| {
            const LOSE_AGAINST: u8 = b'X';
            const DRAW_WITH: u8 = b'Y';
            const WIN_AGAINST: u8 = b'Z';

            match (suggestion, opp) {
                (DRAW_WITH, ROCK) => 3 + _ROCK,
                (DRAW_WITH, PAPER) => 3 + _PAPER,
                (DRAW_WITH, SCISSOR) => 3 + _SCISSOR,

                (WIN_AGAINST, SCISSOR) => 6 + _ROCK,
                (WIN_AGAINST, ROCK) => 6 + _PAPER,
                (WIN_AGAINST, PAPER) => 6 + _SCISSOR,

                (LOSE_AGAINST, PAPER) => _ROCK,
                (LOSE_AGAINST, SCISSOR) => _PAPER,
                (LOSE_AGAINST, ROCK) => _SCISSOR,

                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        })
        .sum();
    Some(sum)
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
