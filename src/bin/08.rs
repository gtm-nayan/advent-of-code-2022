#[derive(Clone, Copy, Debug)]
struct Tree(u8);

#[derive(Debug)]
struct Grid {
    width: usize,
    trees: Vec<Tree>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut width = 0;

        let mut bytes = input.bytes();

        let mut trees: Vec<_> = bytes
            .by_ref()
            .take_while(|&ch| ch != b'\n')
            .inspect(|_| width += 1)
            .map(Tree)
            .collect();
        assert_ne!(width, 0);

        trees.extend(bytes.filter(|&ch| ch != b'\n').map(Tree));

        Grid { width, trees }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let Grid { trees, width } = Grid::new(input);

    Some(
        trees[..(trees.len() - width)]
            .iter()
            .enumerate()
            .filter(|&(idx, Tree(height))| {
                let col = idx % width;
                let row_start = idx - col;
                let row_end = row_start + width;

                let Some(_) = (trees[col..idx])
                .iter()
                .step_by(width)
                .rev()
                    .find(|t| t.0.ge(height)) else { return  true};

                let Some(_) = (trees[(idx + width)..])
                    .iter()
                    .step_by(width)
                    .find(|t| t.0.ge(height)) else { return  true};

                let Some(_) = (trees[row_start..idx]).iter()
                .rev().find(|t| t.0.ge(height)) else { return  true};

                let Some(_) = (trees[(idx + 1)..row_end]).iter().find(|t| t.0.ge(height)) else { return  true};

                false
            })
            .count()  + width,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let Grid { trees, width } = Grid::new(input);
    let rows = trees.len() / width;

    trees[..(trees.len() - width)]
        .iter()
        .enumerate()
        .map(|(idx, Tree(height))| {
            let col = idx % width;
            let row_start = idx - col;
            let row_end = row_start + width;

            let trees_above = idx / width;
            let trees_below = rows - trees_above - 1;

            let top = (trees[col..idx])
                .iter()
                .step_by(width)
                .rev()
                .position(|t| t.0.ge(height))
                .map_or(trees_above, |p| p + 1);

            let bottom = (trees[(idx + width)..])
                .iter()
                .step_by(width)
                .position(|t| t.0.ge(height))
                .map_or(trees_below, |p| p + 1);

            let left = (trees[row_start..idx])
                .iter()
                .rev()
                .position(|t| t.0.ge(height))
                .map_or(col, |p| p + 1);

            let right = (trees[(idx + 1)..row_end])
                .iter()
                .position(|t| t.0.ge(height))
                .map_or(width - (col + 1), |p| p + 1);

            top * bottom * left * right
        })
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        dbg!(part_one(&input));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
