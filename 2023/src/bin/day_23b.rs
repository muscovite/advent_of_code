use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum Tile {
    Ground,
    Wall,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '^' | 'v' | '<' | '>' | '.' => Tile::Ground,
            '#' => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

fn neighbors((x, y): (usize, usize), map: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
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

        if x == map[0].len() || y == map.len() || map[y][x] == Tile::Wall {
            return None;
        }

        Some((x, y))
    })
    .collect()
}

// uniform cost search: minimize theoretical max minus how far I've gone?

fn solve(input: &str) -> usize {
    let map: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect();

    let theoretical_max = map.len() * map[0].len();
    let start = (1, 0);
    let end = (map[0].len() - 2, map.len() - 1);

    let mut frontier: BinaryHeap<(Reverse<usize>, (usize, usize), BTreeSet<(usize, usize)>)> =
        BinaryHeap::from_iter([(Reverse(theoretical_max), start, BTreeSet::new())]);
    while let Some((_, coord, seen)) = frontier.pop() {
        if coord == end {
            return seen.len();
        }
        let next: Vec<(usize, usize)> = neighbors(coord, &map)
            .into_iter()
            .filter(|c| !seen.contains(c))
            .collect();
        for n in next {
            let mut v = seen.clone();
            v.insert(n);
            frontier.push((Reverse(theoretical_max - seen.len()), n, v));
        }
    }
    unreachable!()
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
        assert_eq!(solve(input), 154);
    }
}

util::read_main!();
