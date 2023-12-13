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

fn get_counts(grid: &Vec<Vec<char>>) -> Vec<usize> {
    let mut lines: HashMap<Vec<char>, (usize, Vec<usize>)> = HashMap::new();
    // Hash lines, keeping track of counts and indexes
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
    // Ignore lines that didn't appear at least once
    let lines: Vec<_> = lines
        .into_iter()
        .filter(|(_, (count, _))| *count > 1)
        .collect();

    let mut solutions: Vec<usize> = Vec::new();
    for (_, (_, mut indices)) in lines.into_iter() {
        while let Some(i) = indices.pop() {
            for j in indices.iter() {
                // Potential reflection point must involve two matching lines
                // next to each other
                if i.abs_diff(*j) > 1 {
                    continue;
                }

                // Check validity of the pair as a reflection point
                let min = i.min(*j);
                let max = i.max(*j);

                for i in 0.. {
                    if grid[min - i] != grid[max + i] {
                        break;
                    }
                    if min - i == 0 || max + i == grid.len() - 1 {
                        solutions.push(min + 1);
                        break;
                    }
                }
            }
        }
    }
    solutions
}

fn num_to_left(grid: &Vec<Vec<char>>, grid_transposed: &Vec<Vec<char>>) -> Vec<usize> {
    let mut solutions: Vec<usize> = Vec::new();
    // Test for horizontal line reflection
    let sub_solns = get_counts(&grid);
    if sub_solns.len() > 0 {
        solutions.extend(sub_solns.into_iter().map(|e| e * 100));
    }

    // Test for vertical line reflection
    let sub_solns = get_counts(&grid_transposed);
    if sub_solns.len() > 0 {
        solutions.extend(sub_solns);
    }
    return solutions;
}

fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let mut grid: Vec<Vec<char>> = pattern.lines().map(|l| l.chars().collect()).collect();
            let mut transposed: Vec<Vec<char>> = transpose(grid.clone());
            let orig = num_to_left(&grid, &transposed)[0];
            for x in 0..grid[0].len() {
                for y in 0..grid.len() {
                    // flip one space
                    match grid[y][x] {
                        '.' => grid[y][x] = '#',
                        '#' => grid[y][x] = '.',
                        _ => (),
                    }
                    match transposed[x][y] {
                        '.' => transposed[x][y] = '#',
                        '#' => transposed[x][y] = '.',
                        _ => (),
                    }

                    // NB: we're guaranteed 0, 1, or 2 results
                    let potential_new = num_to_left(&grid, &transposed);
                    match potential_new.into_iter().filter(|p| *p != orig).next() {
                        Some(val) => return val,
                        None => (),
                    }

                    // reset
                    match grid[y][x] {
                        '.' => grid[y][x] = '#',
                        '#' => grid[y][x] = '.',
                        _ => (),
                    }
                    match transposed[x][y] {
                        '.' => transposed[x][y] = '#',
                        '#' => transposed[x][y] = '.',
                        _ => (),
                    }
                }
            }
            // if we get here, then fixing the smudge still gave the same result
            orig
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
        assert_eq!(solve(input), 400);
    }
}

util::read_main!();
