use std::collections::HashSet;

fn neighbors(x: usize, y: usize, width: usize, height: usize) -> [Option<(usize, usize)>; 4] {
    let left = x.checked_sub(1).map(|left| (left, y));
    let up = y.checked_sub(1).map(|up| (x, up));

    let right = Some(x + 1).filter(|&x| x < width).map(|right| (right, y));
    let down = Some(y + 1).filter(|&y| y < height).map(|down| (x, down));

    [left, right, up, down]
}

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();

    let low_points = (0..width).flat_map(|x| {
        let grid = &grid;
        (0..height).filter_map(move |y| {
            if neighbors(x, y, width, height)
                .into_iter()
                .flatten()
                .all(|idx| grid[idx.1][idx.0] > grid[y][x])
            {
                Some((x, y))
            } else {
                None
            }
        })
    });

    // Intended to BFS, but actually DFSing
    let mut basin_sizes: Vec<_> = low_points
        .map(|(x, y)| {
            let mut visited = HashSet::new();
            let mut to_visit: Vec<(usize, usize)> = vec![(x, y)];

            while let Some(coord) = to_visit.pop() {
                if visited.contains(&coord) {
                    continue;
                }
                let new_points = neighbors(coord.0, coord.1, width, height)
                    .into_iter()
                    .flatten()
                    .filter(|&(x, y)| grid[y][x] != 9);

                to_visit.extend(new_points);
                visited.insert(coord);
            }

            visited.len()
        })
        .collect();

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"2199943210
3987894921
9856789892
8767896789
9899965678";
        assert_eq!(solve(input), 1134);
    }
}

util::read_main!();
