use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> usize {
    let mut start_pos = (0, 0);
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
                start_pos = *pos;
            }
        })
        .collect();

    let mut heading = (0, -1);
    let mut possible_obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut curr_pos = start_pos;
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
        possible_obstacles.insert(curr_pos);
        curr_pos = (curr_pos.0 + heading.0, curr_pos.1 + heading.1);
    }

    possible_obstacles
        .iter()
        .filter(|&obstacle_pos| {
            let mut curr_pos = start_pos;
            let mut heading = (0, -1);
            let mut seen: HashSet<((isize, isize), (isize, isize))> = HashSet::new();
            while let Some(space) = grid.get(&curr_pos) {
                if *space == '#' || curr_pos == *obstacle_pos {
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
                if !seen.insert((curr_pos, heading)) {
                    return true;
                }
                curr_pos = (curr_pos.0 + heading.0, curr_pos.1 + heading.1);
            }
            false
        })
        .count()
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
        assert_eq!(solve(input), 6);
    }
}

util::read_main!();
