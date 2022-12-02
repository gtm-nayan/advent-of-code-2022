#![feature(binary_heap_into_iter_sorted)]
use std::{collections::BinaryHeap, ops::Add};

pub fn calories(input: &str) -> impl Iterator<Item = u32> + '_ {
    let mut it = input.lines();
    std::iter::from_fn(move || {
        it.by_ref()
            .map_while(|line| line.parse().ok())
            .reduce(Add::add)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    calories(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    calories(input)
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
