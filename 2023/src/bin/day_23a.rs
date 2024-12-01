use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum Tile {
    Up,
    Down,
    Left,
    Right,
    Ground,
    Wall,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '^' => Tile::Up,
            'v' => Tile::Down,
            '<' => Tile::Left,
            '>' => Tile::Right,
            '.' => Tile::Ground,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

fn neighbors((x, y): (usize, usize), map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let curr = map[y][x];
    [
        // left
        if curr != Tile::Left && curr != Tile::Ground {
            None
        } else {
            Some((x.checked_sub(1), Some(y)))
        },
        // right
        if curr != Tile::Right && curr != Tile::Ground {
            None
        } else {
            Some((Some(x + 1), Some(y)))
        },
        // up
        if curr != Tile::Up && curr != Tile::Ground {
            None
        } else {
            Some((Some(x), y.checked_sub(1)))
        },
        // down
        if curr != Tile::Down && curr != Tile::Ground {
            None
        } else {
            Some((Some(x), Some(y + 1)))
        },
    ]
    .iter()
    .flatten()
    .filter_map(|(x, y)| {
        if x.is_none() || y.is_none() {
            return None;
        }

        let (x, y) = (x.unwrap(), y.unwrap());

        if x == map[0].len() || y == map.len() || map[y][x] == Tile::Wall {
            return None;
        }

        Some((x, y))
    })
    .collect()
}

fn solve(input: &str) -> usize {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect();

    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    let mut frontier: VecDeque<(_, HashSet<(usize, usize)>)> =
        VecDeque::from_iter([(start, HashSet::new())]);
    let mut max_steps = 0;
    while let Some((coord, seen)) = frontier.pop_front() {
        if coord == end {
            max_steps = max_steps.max(seen.len());
        }
        let next: Vec<(usize, usize)> = neighbors(coord, &map)
            .into_iter()
            .filter(|c| !seen.contains(c))
            .collect();
        for n in next {
            let mut v = seen.clone();
            v.insert(n);
            frontier.push_back((n, v));
        }
    }
    max_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(solve(input), 94);
    }
}

util::read_main!();
