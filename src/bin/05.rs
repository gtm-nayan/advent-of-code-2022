#![feature(iter_advance_by)]
#![feature(assert_matches)]
#![feature(get_many_mut)]

use std::iter::from_fn;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn perform(&mut self, m: Move, rev: bool) {
        let [source, dest] = self.stacks.get_many_mut([m.from, m.to]).unwrap();

        let remove_from = source.len() - m.count;
        let drain = source.drain(remove_from..);

        if rev {
            dest.extend(drain.rev())
        } else {
            dest.extend(drain)
        };
    }

    pub fn message(&self) -> String {
        self.stacks.iter().filter_map(|v| v.last()).collect()
    }
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_input(input: &str) -> (Stacks, impl Iterator<Item = Move> + '_) {
    let mut chars = input.chars();

    let num = (memchr::memchr(b'\n', input.as_bytes()).unwrap() + 1) / 4;

    let mut stacks = vec![Vec::new(); num];

    chars
        .by_ref()
        .enumerate()
        .skip(1)
        .step_by(2)
        .take_while(|&(_, ch)| ch != 'm')
        .filter(|(_, ch)| ch.is_ascii_uppercase())
        .for_each(|(pos, ch)| {
            unsafe { stacks.get_unchecked_mut((pos / 4) % num) }.push(ch);
        });

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let moves = from_fn(move || {
        chars.next()?;
        let mut get_num = || {
            chars
                .by_ref()
                .skip_while(|ch| !ch.is_ascii_digit())
                .take_while(|ch| ch.is_ascii_digit())
                .fold(0, |acc, ch| acc * 10 + (ch as u8 - b'0')) as usize
        };

        let count = get_num();
        let from = get_num() - 1;
        let to = get_num() - 1;

        Some(Move { from, to, count })
    });

    (Stacks { stacks }, moves)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    moves.for_each(|m| {
        stacks.perform(m, true);
    });

    Some(stacks.message())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    moves.for_each(|m| {
        stacks.perform(m, false);
    });

    Some(stacks.message())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input).unwrap(), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input).unwrap(), "MCD");
    }
}
