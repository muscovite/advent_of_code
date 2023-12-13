use std::collections::HashMap;

// thanks for the implementation, someone on the internet
fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for inner in &mut v {
        inner.reverse();
    }
    (0..v[0].len())
        .map(|_| {
            v.iter_mut()
                .map(|inner| inner.pop().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn num_to_left(grid: &Vec<Vec<char>>) -> Option<usize> {
    let mut lines: HashMap<Vec<char>, (usize, Vec<usize>)> = HashMap::new();
    for (idx, line) in grid.iter().enumerate() {
        match lines.get_mut(line) {
            Some((count, indices)) => {
                *count = *count + 1;
                indices.push(idx);
            }
            None => {
                lines.insert(line.clone(), (1, vec![idx]));
            }
        }
    }
    let lines: HashMap<_, _> = lines
        .into_iter()
        .filter(|(_, (count, _))| *count > 1)
        .collect();

    for (_, (_, mut indices)) in lines.into_iter() {
        while let Some(i) = indices.pop() {
            for j in indices.iter() {
                if i.abs_diff(*j) > 1 {
                    continue;
                }

                // Possible reflection
                let min = i.min(*j);
                let max = i.max(*j);

                for i in 0.. {
                    if grid[min - i] != grid[max + i] {
                        break;
                    }
                    if min - i == 0 || max + i == grid.len() - 1 {
                        return Some(min + 1);
                    }
                }
            }
        }
    }
    None
}

fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            // Test for horizontal line reflection
            let grid: Vec<Vec<char>> = pattern.lines().map(|l| l.chars().collect()).collect();
            match num_to_left(&grid) {
                Some(count) => {
                    return count * 100;
                }
                None => {}
            }

            // Test for vertical line reflection
            let grid: Vec<Vec<char>> = transpose(grid);
            match num_to_left(&grid) {
                Some(count) => {
                    return count;
                }
                None => {}
            }
            panic!("should not reach")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), 405);
    }
}

util::read_main!();
