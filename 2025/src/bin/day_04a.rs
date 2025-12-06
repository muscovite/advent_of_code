use std::collections::HashSet;

fn accessible(grid: &HashSet<(isize, isize)>, coord: (isize, isize)) -> bool {
    [
        // up
        (coord.0, coord.1 - 1),
        // down
        (coord.0, coord.1 + 1),
        // left
        (coord.0 - 1, coord.1),
        // right
        (coord.0 + 1, coord.1),
        // up-left
        (coord.0 - 1, coord.1 - 1),
        // up-right
        (coord.0 + 1, coord.1 - 1),
        // down-left
        (coord.0 - 1, coord.1 + 1),
        // down-right
        (coord.0 + 1, coord.1 + 1),
    ]
    .into_iter()
    .filter(|pos| grid.contains(pos))
    .count()
        < 4
}

fn solve(input: &str) -> usize {
    let grid: HashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().flat_map(move |(x, c)| {
                if c != '@' {
                    return None;
                }
                Some((x as isize, y as isize))
            })
        })
        .collect();
    grid.iter()
        .filter(|&coord| accessible(&grid, *coord))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(solve(input), 13);
    }
}

util::read_main!();
