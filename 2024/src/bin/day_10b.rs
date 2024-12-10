use std::collections::{HashMap, VecDeque};

// even index == file, odd index == empty space
fn solve(input: &str) -> usize {
    let grid: HashMap<(isize, isize), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c.to_digit(10).unwrap()))
        })
        .collect();

    let mut positions: VecDeque<((isize, isize), u32)> = grid
        .iter()
        .filter_map(|(&pos, height)| {
            if *height == 0 {
                return Some((pos, 0));
            }
            None
        })
        .collect();

    let mut score = 0;
    while let Some(((x, y), curr_height)) = positions.pop_front() {
        let next_spots = [
            // up
            (x, y - 1),
            // down
            (x, y + 1),
            // left
            (x - 1, y),
            // right
            (x + 1, y),
        ];

        positions.extend(
            next_spots
                .into_iter()
                .filter_map(|pos| match grid.get(&pos) {
                    Some(height) => {
                        if *height == curr_height + 1 {
                            if *height == 9 {
                                score += 1;
                                return None;
                            } else {
                                return Some((pos, *height));
                            }
                        }
                        None
                    }
                    None => None,
                }),
        );
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"0123
1234
8765
9876";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case2() {
        let input = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(solve(input), 36);
    }
}

util::read_main!();
