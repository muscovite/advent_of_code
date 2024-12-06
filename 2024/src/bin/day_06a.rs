use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let mut curr_pos = (0, 0);
    let grid: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .inspect(|(pos, c)| {
            if *c == '^' {
                curr_pos = *pos;
            }
        })
        .collect();

    let mut heading = (0, -1);
    let mut seen: HashSet<(isize, isize)> = HashSet::new();
    while let Some(space) = grid.get(&curr_pos) {
        if *space == '#' {
            // back up
            curr_pos = (curr_pos.0 - heading.0, curr_pos.1 - heading.1);
            // turn
            heading = match heading {
                // up -> right
                (0, -1) => (1, 0),
                // right -> down
                (1, 0) => (0, 1),
                // down -> left
                (0, 1) => (-1, 0),
                // left -> up
                (-1, 0) => (0, -1),
                _ => panic!("???"),
            };
        }
        seen.insert(curr_pos);
        curr_pos = (curr_pos.0 + heading.0, curr_pos.1 + heading.1);
    }

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(solve(input), 41);
    }
}

util::read_main!();
