#![feature(binary_heap_into_iter_sorted)]
use std::{collections::BinaryHeap, ops::Add, str::Lines};

struct Elfs<'a>(Lines<'a>);

impl Iterator for Elfs<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .by_ref()
            .map_while(|line| line.parse().ok()) // A bit of trickery, empty line will fail parsing so no need to check it separately
            .reduce(Add::add)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Elfs(input.lines()).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Elfs(input.lines())
        .collect::<BinaryHeap<u32>>()
        .into_iter_sorted()
        .take(3)
        .reduce(Add::add)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
