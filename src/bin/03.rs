#![feature(iter_array_chunks)]

fn bitset(it: &[u8]) -> u64 {
    it.iter().fold(0_u64, |acc, b| acc | (1 << (b - 64)))
}

fn intersection_priority(bits: u64) -> u32 {
    let n = bits.trailing_zeros() as u8 + 64;

    (if n >= b'a' {
        n - b'a' + 1
    } else {
        n - b'A' + 26 + 1
    } as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (head, tail) = line.as_bytes().split_at(line.len() / 2);

                let head = bitset(head);
                let tail = bitset(tail);

                intersection_priority(head & tail)
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
                let [a, b, c] = group.map(str::as_bytes).map(bitset);
                intersection_priority(a & b & c)
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
