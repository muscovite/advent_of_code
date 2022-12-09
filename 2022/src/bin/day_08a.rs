use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let forest: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // rows
    for (y, row) in forest.iter().enumerate() {
        // left to right
        let mut max = row[0];
        visible.extend(
            row.into_iter()
                .enumerate()
                .filter_map(|(x, &height)| {
                    if height > max {
                        max = height;
                        return Some((x, y));
                    }
                    None
                })
                .chain(std::iter::once((0, y))),
        );

        // right to left
        max = *row.last().unwrap();
        visible.extend(
            row.into_iter()
                .enumerate()
                .rev()
                .filter_map(|(x, &height)| {
                    if height > max {
                        max = height;
                        return Some((x, y));
                    }
                    None
                })
                .chain(std::iter::once((row.len() - 1, y))),
        );
    }

    // columns
    let width = forest[0].len();
    for col in 0..width {
        // top to bottom
        let mut max = forest[0][col];
        visible.extend(
            forest
                .iter()
                .map(|row| row[col])
                .enumerate()
                .filter_map(|(y, height)| {
                    if height > max {
                        max = height;
                        return Some((col, y));
                    }
                    None
                })
                .chain(std::iter::once((col, 0))),
        );

        // bottom to top
        max = forest[forest.len() - 1][col];
        visible.extend(
            forest
                .iter()
                .map(|row| row[col])
                .enumerate()
                .rev()
                .filter_map(|(y, height)| {
                    if height > max {
                        max = height;
                        return Some((col, y));
                    }
                    None
                })
                .chain(std::iter::once((col, forest.len() - 1))),
        );
    }

    visible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"30373
25512
65332
33549
35390";
        assert_eq!(solve(input), 21);
    }
}

util::read_main!();
