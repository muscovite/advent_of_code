const FACTOR: usize = 1000000 - 1;

fn solve(input: &str) -> usize {
    let empty_rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter_map(|(idx, l)| {
            if l.chars().any(|c| c != '.') {
                return None;
            }
            Some(idx)
        })
        .collect();

    let empty_cols: Vec<char> =
        input
            .lines()
            .fold(vec!['.'; input.lines().count()], |mut empty_cols, l| {
                for (idx, c) in l.char_indices() {
                    if c != '.' {
                        empty_cols[idx] = '#';
                    }
                }
                empty_cols
            });
    let empty_cols: Vec<usize> = empty_cols
        .into_iter()
        .enumerate()
        .filter_map(|(idx, c)| {
            if c == '.' {
                return Some(idx);
            }
            None
        })
        .collect();

    let galaxies: Vec<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices().filter_map(move |(x, c)| {
                if c != '.' {
                    return Some((x, y));
                }
                None
            })
        })
        .collect();

    galaxies
        .iter()
        .flat_map(|g1| {
            galaxies.iter().filter_map(|g2| {
                let min_x = g1.0.min(g2.0);
                let max_x = g1.0.max(g2.0);
                let min_y = g1.1.min(g2.1);
                let max_y = g1.1.max(g2.1);

                // overlapping cols
                let dx = empty_cols
                    .iter()
                    .filter(|x| (min_x..max_x).contains(x))
                    .count();

                let dy = empty_rows
                    .iter()
                    .filter(|y| (min_y..max_y).contains(y))
                    .count();

                // overlapping rows

                Some((dx * FACTOR) + (max_x - min_x) + (dy * FACTOR) + (max_y - min_y))
            })
        })
        .sum::<usize>()
        / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(solve(input), 8410);
    }
}

util::read_main!();
