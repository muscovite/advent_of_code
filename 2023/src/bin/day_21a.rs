use std::collections::{HashMap, VecDeque};

#[cfg(not(test))]
const TARGET_STEPS: usize = 64;
#[cfg(test)]
const TARGET_STEPS: usize = 6;

fn neighbors((x, y): (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    [
        // left
        (x.checked_sub(1), Some(y)),
        // right
        (Some(x + 1), Some(y)),
        // up
        (Some(x), y.checked_sub(1)),
        // down
        (Some(x), Some(y + 1)),
    ]
    .iter()
    .filter_map(|(x, y)| {
        if x.is_none() || y.is_none() {
            return None;
        }

        let (x, y) = (x.unwrap(), y.unwrap());

        if x == map[0].len() || y == map.len() || map[y][x] == '#' {
            return None;
        }

        Some((x, y))
    })
    .collect()
}

fn solve(input: &str) -> usize {
    let mut start: Option<(usize, usize)> = None;
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some((x, y));
                    }
                    c
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();

    let mut frontier: VecDeque<(_, usize)> = VecDeque::from_iter([(start, 0)]);
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
    while let Some((coord, steps)) = frontier.pop_front() {
        if steps == TARGET_STEPS {
            continue;
        }
        let next = neighbors(coord, &map);
        for n in next {
            if seen.insert(n, steps + 1).is_none() {
                frontier.push_back((n, steps + 1))
            }
        }
    }
    seen.into_iter().filter(|(_, steps)| steps % 2 == 0).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(solve(input), 16);
    }
}

util::read_main!();
