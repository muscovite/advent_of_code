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
            if *c == 'X' {
                return Some((x, y));
            }
            None
        })
        // get M positions
        .flat_map(|(x, y)| {
            [
                // up
                ((x, y - 1), (0, -1)),
                // down
                ((x, y + 1), (0, 1)),
                // left
                ((x - 1, y), (-1, 0)),
                // right
                ((x + 1, y), (1, 0)),
                // top left
                ((x - 1, y - 1), (-1, -1)),
                // top right
                ((x + 1, y - 1), (1, -1)),
                // bottom left
                ((x - 1, y + 1), (-1, 1)),
                // bottom right
                ((x + 1, y + 1), (1, 1)),
            ]
            .into_iter()
            .filter_map(|((x, y), (dx, dy))| {
                let c = grid.get(&(x, y))?;
                if *c == 'M' {
                    return Some(((x, y), (dx, dy)));
                }
                None
            })
        })
        // get A positions
        .filter_map(|((x, y), (dx, dy))| {
            let new_pos = (x + dx, y + dy);
            grid.get(&new_pos).filter(|&&c| c == 'A')?;
            Some((new_pos, (dx, dy)))
        })
        // get S positions
        .filter_map(|((x, y), (dx, dy))| {
            let new_pos = (x + dx, y + dy);
            grid.get(&new_pos).filter(|&&c| c == 'S')?;
            Some((new_pos, (dx, dy)))
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
        assert_eq!(solve(input), 18);
    }
}

util::read_main!();
