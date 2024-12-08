use std::collections::{HashMap, HashSet};
use std::iter::successors;

fn solve(input: &str) -> usize {
    let mut grid: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let input = input.trim();
    let max_x = input.lines().next().unwrap().len() as isize;
    let max_y = input.lines().count() as isize;

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(x, c)| {
                grid.entry(c)
                    .or_insert(Vec::new())
                    .push((x as isize, y as isize));
            });
    });
    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    grid.into_iter().for_each(|(_, locations)| {
        let mut checked: HashSet<(isize, isize)> = HashSet::new();
        for (i, (x1, y1)) in locations.iter().enumerate() {
            for (j, (x2, y2)) in locations.iter().enumerate() {
                // already checked this pair, or picked same antenna
                if checked.contains(&(j as isize, i as isize)) || i == j {
                    continue;
                }
                let (dx, dy) = ((x2 - x1), (y2 - y1));

                antinodes.extend(successors(Some((*x1, *y1)), |(x, y)| {
                    let (new_x, new_y) = (*x - dx, *y - dy);
                    if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
                        return None;
                    }
                    Some((new_x, new_y))
                }));
                antinodes.extend(successors(Some((*x2, *y2)), |(x, y)| {
                    let (new_x, new_y) = (*x + dx, *y + dy);
                    if new_x < 0 || new_x >= max_x || new_y < 0 || new_y >= max_y {
                        return None;
                    }
                    Some((new_x, new_y))
                }));
                checked.insert((i as isize, j as isize));
            }
        }
    });
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(solve(input), 34);
    }
}

util::read_main!();
