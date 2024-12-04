use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let grid: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .collect();

    grid.iter()
        .filter_map(|(&(x, y), c)| {
            if *c == 'A' {
                return Some((x, y));
            }
            None
        })
        .flat_map(|(x, y)| {
            [
                // M M
                //  A
                // S S
                (
                    (x - 1, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                    (x + 1, y + 1),
                ),
                // S S
                //  A
                // M M
                (
                    (x - 1, y + 1),
                    (x + 1, y + 1),
                    (x - 1, y - 1),
                    (x + 1, y - 1),
                ),
                // M S
                //  A
                // M S
                (
                    (x - 1, y - 1),
                    (x - 1, y + 1),
                    (x + 1, y - 1),
                    (x + 1, y + 1),
                ),
                // S M
                //  A
                // S M
                (
                    (x + 1, y - 1),
                    (x + 1, y + 1),
                    (x - 1, y - 1),
                    (x - 1, y + 1),
                ),
            ]
            .into_iter()
            .filter(|(m1, m2, s1, s2)| {
                let neighbors = grid
                    .get(&m1)
                    .zip(grid.get(&m2))
                    .zip(grid.get(&s1))
                    .zip(grid.get(&s2));

                matches!(neighbors, Some(((('M', 'M'), 'S'), 'S')))
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(solve(input), 9);
    }
}

util::read_main!();
