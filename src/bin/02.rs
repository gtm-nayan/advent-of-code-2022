use std::{ops::Add, str::Lines};

#[repr(u8)]
enum Rps {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Rps {
    pub fn result(&self, other: &Self) -> RPSResult {
        use Rps::*;
        match (self, other) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => RPSResult::Draw,
            (Paper, Rock) | (Scissors, Paper) | (Rock, Scissors) => RPSResult::Win,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => RPSResult::Loss,
        }
    }
}

impl From<char> for Rps {
    fn from(value: char) -> Self {
        use Rps::*;
        match value {
            'X' | 'A' => Rock,
            'Y' | 'B' => Paper,
            'Z' | 'C' => Scissors,
            _ => unreachable!(),
        }
    }
}

#[repr(u8)]
enum RPSResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl RPSResult {
    pub fn suggest(&self, opponent: Rps) -> Rps {
        use RPSResult::*;
        use Rps::*;
        match (self, opponent) {
            (Win, Rock) | (Loss, Scissors) | (Draw, Paper) => Paper,
            (Win, Paper) | (Loss, Rock) | (Draw, Scissors) => Scissors,
            (Win, Scissors) | (Loss, Paper) | (Draw, Rock) => Rock,
        }
    }
}

impl From<char> for RPSResult {
    fn from(value: char) -> Self {
        use RPSResult::*;
        match value {
            'X' => Loss,
            'Y' => Draw,
            'Z' => Win,
            _ => unreachable!(),
        }
    }
}

struct Input<'a>(Lines<'a>);

impl Iterator for Input<'_> {
    type Item = (char, char);

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.0.by_ref().next()?;
        let mut chars = line.chars();

        let Some(first) = chars.next() else {return None;};
        let Some(second) = chars.nth(1) else {unreachable!()};

        Some((first, second))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Input(input.lines())
        .map(|(opp, me)| {
            let me: Rps = me.into();

            me.result(&opp.into()) as u32 + me as u32
        })
        .reduce(Add::add)
}

pub fn part_two(input: &str) -> Option<u32> {
    Input(input.lines())
        .map(|(opp, suggestion)| {
            let suggestion: RPSResult = suggestion.into();
            let me = suggestion.suggest(opp.into());
            suggestion as u32 + me as u32
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
