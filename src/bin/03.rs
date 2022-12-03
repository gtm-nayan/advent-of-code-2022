#![feature(iter_array_chunks)]
use std::iter::from_fn;

const N_BUCKETS: usize = 64;

static BUCKETS: [bool; N_BUCKETS] = [false; N_BUCKETS];

const fn get_priority(ch: usize) -> usize {
    if ch < 26 {
        ch + 27
    } else {
        ch - 31
    }
}

fn get_bucket(ch: u8) -> usize {
    (ch - b'A') as usize
}

fn rucksacks(input: &str) -> impl Iterator<Item = usize> + '_ {
    let mut lines = input.lines();

    from_fn(move || {
        let line = lines.next()?.as_bytes();

        let mut left = BUCKETS;
        let mut right = BUCKETS;

        let (head, tail) = line.split_at((line.len() + 1) / 2);

        head.iter().zip(tail).for_each(|(&a, &b)| {
            left[get_bucket(a)] = true;
            right[get_bucket(b)] = true;
        });

        (0..N_BUCKETS)
            .find(|&i| left[i] & right[i])
            .map(get_priority)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(rucksacks(input).sum::<usize>() as u32)
}

fn rucksacks_2(input: &str) -> impl Iterator<Item = usize> + '_ {
    let mut groups = input.lines().array_chunks::<3>();

    from_fn(move || {
        let [a, b, c] = groups.next()?.map(|line| {
            let mut buf = BUCKETS;
            line.bytes().map(get_bucket).for_each(|idx| buf[idx] = true);
            buf
        });

        (0..N_BUCKETS)
            .find(|&i| a[i] & b[i] & c[i])
            .map(get_priority)
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(rucksacks_2(input).sum::<usize>() as u32)
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
