fn indexes_to_check(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> [Option<(usize, usize)>; 4] {
    let left = x.checked_sub(1).map(|left| (left, y));
    let up = y.checked_sub(1).map(|up| (x, up));

    let right = Some(x + 1).filter(|&x| x < width).map(|right| (right, y));
    let down = Some(y + 1).filter(|&y| y < height).map(|down| (x, down));

    [left, right, up, down]
}

fn solve(input: &str) -> u32 {
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
            if indexes_to_check(x, y, width, height)
                .into_iter()
                .flatten()
                .all(|idx| grid[idx.1][idx.0] > grid[y][x])
            {
                Some(grid[y][x] + 1)
            } else {
                None
            }
        })
    });

    low_points.sum()
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
        assert_eq!(solve(input), 15);
    }
}

util::read_main!();
