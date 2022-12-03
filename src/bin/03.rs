#![feature(iter_array_chunks)]

fn get_bucket(ch: u8) -> usize {
    1 + (ch - if ch >= b'a' { b'a' } else { b'A' - 26 }) as usize
}

fn bitset(it: impl IntoIterator<Item = u8>) -> u64 {
    it.into_iter()
        .map(get_bucket)
        .fold(0_u64, |acc, b| acc | (1 << b))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (head, tail) = line.as_bytes().split_at(line.len() / 2);

                let head = bitset(head.iter().copied());
                let tail = bitset(tail.iter().copied());

                (head & tail).trailing_zeros()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .array_chunks::<3>()
            .map(|group| {
                let [a, b, c] = group.map(str::bytes).map(bitset);
                (a & b & c).trailing_zeros()
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
