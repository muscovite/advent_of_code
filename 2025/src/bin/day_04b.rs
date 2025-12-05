use std::collections::HashMap;

fn accessible(grid: &HashMap<(isize, isize), char>, coord: (isize, isize)) -> bool {
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
    .filter(|pos| grid.get(pos).is_some())
    .count()
        < 4
}

fn solve(input: &str) -> usize {
    let mut grid: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .filter(|(_, c)| *c == '@')
        .collect();

    let mut rolls = 0;
    loop {
        let new_grid: HashMap<_, _> = grid
            .clone()
            .into_iter()
            .filter(|&(coord, _)| !accessible(&grid, coord))
            .collect();

        if new_grid.len() == grid.len() {
            break;
        }
        rolls += grid.len() - new_grid.len();
        grid = new_grid;
    }

    rolls
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
        assert_eq!(solve(input), 43);
    }
}

util::read_main!();
