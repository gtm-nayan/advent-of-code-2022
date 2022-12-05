#![feature(iter_advance_by)]
#![feature(assert_matches)]
#![feature(get_many_mut)]

use std::collections::VecDeque;

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

impl Move {
    pub fn from_line(line: &str) -> Self {
        let mut chars = line.chars();

        let mut get_num = move || {
            chars
                .by_ref()
                .skip_while(|ch| !ch.is_ascii_digit())
                .take_while(|ch| ch.is_ascii_digit())
                .fold(0, |acc, ch| acc * 10 + (ch as u8 - b'0')) as usize
        };

        let count = get_num();
        let from = get_num() - 1;
        let to = get_num() - 1;

        Move { from, to, count }
    }
}

fn parse_crate_line(input: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    input
        .chars()
        .enumerate()
        .filter(|(_, ch)| ch.is_ascii_alphabetic())
        .map(|(pos, ch)| (pos / 4, ch))
}

fn parse_input(input: &str) -> (Stacks, impl Iterator<Item = Move> + '_) {
    let first = input.lines().next().unwrap();
    let mut crates = vec![VecDeque::<char>::new(); (first.len() + 1) / 4];

    let mut lines = input.lines();

    lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .flat_map(parse_crate_line)
        .for_each(|(stack, cr)| {
            let stack = crates.get_mut(stack).unwrap();

            stack.push_front(cr)
        });

    let stacks = Stacks {
        stacks: crates.into_iter().map(Vec::from).collect(),
    };

    let moves = lines.map(Move::from_line);

    (stacks, moves)
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
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_parse_move() {
        let line = "move 1 from 2 to 1";

        assert_matches!(
            Move::from_line(line),
            Move {
                from: 1,
                to: 0,
                count: 1
            }
        )
    }

    #[test]
    fn test_parse_crate_line() {
        let line = "[A]     [B]";

        assert!(parse_crate_line(line).eq([(0_usize, 'A'), (2_usize, 'B')].into_iter()))
    }

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
