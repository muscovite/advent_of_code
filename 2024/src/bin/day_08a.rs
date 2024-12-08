use std::collections::{HashMap, HashSet};

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
                antinodes.extend([(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)].iter().filter(
                    |(x, y)| {
                        let (x, y) = (*x, *y);
                        if x < 0 || x >= max_x || y < 0 || y >= max_y {
                            return false;
                        }
                        true
                    },
                ));
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
        assert_eq!(solve(input), 14);
    }
}

util::read_main!();
