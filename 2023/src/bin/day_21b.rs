use std::collections::{HashMap, VecDeque};

#[cfg(not(test))]
const TARGET_STEPS: usize = 26501365;
#[cfg(test)]
const TARGET_STEPS: usize = 50;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct CoordType {
    coord: (usize, usize),
    universe: (i32, i32),
}

fn neighbors(coord: CoordType, map: &Vec<Vec<char>>) -> [Option<CoordType>; 4] {
    let left = {
        let mut universe = coord.universe.clone();
        let (x, y) = coord.coord;
        let x = match x.checked_sub(1) {
            Some(x) => x,
            None => {
                universe.0 -= 1;
                map[0].len() - 1
            }
        };

        if map[x][y] == '#' {
            None
        } else {
            Some(CoordType {
                coord: (x, coord.coord.1),
                universe: universe,
            })
        }
    };

    let right = {
        let mut universe = coord.universe.clone();
        let (x, y) = coord.coord;
        let x = if (x + 1) % map[0].len() == 0 {
            universe.0 += 1;
            0
        } else {
            x + 1
        };

        if map[x][y] == '#' {
            None
        } else {
            Some(CoordType {
                coord: (x, coord.coord.1),
                universe: universe,
            })
        }
    };

    let up = {
        let mut universe = coord.universe.clone();
        let (x, y) = coord.coord;
        let y = match y.checked_sub(1) {
            Some(val) => val,
            None => {
                universe.1 -= 1;
                map.len() - 1
            }
        };

        if map[x][y] == '#' {
            None
        } else {
            Some(CoordType {
                coord: (x, y),
                universe: universe,
            })
        }
    };

    let down = {
        let mut universe = coord.universe.clone();
        let (x, y) = coord.coord;
        let y = if (y + 1) % map.len() == 0 {
            universe.1 += 1;
            0
        } else {
            y + 1
        };
        if map[x][y] == '#' {
            None
        } else {
            Some(CoordType {
                coord: (x, y),
                universe: universe,
            })
        }
    };
    [left, right, up, down]
}

fn solve(input: &str) -> usize {
    let mut start: Option<CoordType> = None;
    let map: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Some(CoordType {
                            coord: (x, y),
                            universe: (0, 0),
                        });
                    }
                    c
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();
    dbg!(map[0].len(), map.len());
    dbg!(&start);

    let mut frontier: VecDeque<(_, usize)> = VecDeque::from_iter([(start, 0)]);
    let mut seen: HashMap<CoordType, usize> = HashMap::new();
    while let Some((coord, steps)) = frontier.pop_front() {
        if steps == TARGET_STEPS {
            continue;
        }
        let next = neighbors(coord, &map).into_iter().flatten();
        for n in next {
            if seen.insert(n.clone(), steps + 1).is_none() {
                frontier.push_back((n, steps + 1))
            }
        }
    }
    seen.into_iter()
        .filter(|(_, steps)| {
            if TARGET_STEPS % 2 == 0 {
                steps % 2 == 0
            } else {
                steps % 2 == 1
            }
        })
        .count()
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
        assert_eq!(solve(input), 1594);
    }
}

util::read_main!();
