use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
enum Space {
    Reindeer,
    Wall,
    Empty,
    End,
}

impl Space {
    fn new(c: char) -> Space {
        match c {
            'S' => Space::Reindeer,
            'E' => Space::End,
            '#' => Space::Wall,
            '.' => Space::Empty,
            _ => unreachable!("unexpected space type"),
        }
    }
}

fn solve(input: &str) -> usize {
    let mut reindeer = None;
    let grid: HashMap<(isize, isize), Space> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), Space::new(c)))
        })
        .inspect(|(coord, space)| {
            if *space == Space::Reindeer {
                reindeer = Some(*coord);
            }
        })
        .collect();
    let reindeer = reindeer.unwrap();

    // frontier
    let mut heap: BinaryHeap<(Reverse<usize>, ((isize, isize), (isize, isize)))> =
        BinaryHeap::new();
    heap.push((Reverse(0), (reindeer, (1, 0))));

    // seen
    let mut seen: HashMap<_, _> = HashMap::new();
    seen.insert((reindeer, (1, 0)), 0);

    while let Some((Reverse(cost), (pos, dir))) = heap.pop() {
        // reached the end
        if grid.get(&(pos)) == Some(&Space::End) {
            return cost;
        }

        [
            // up
            (0, -1),
            // down
            (0, 1),
            // left
            (-1, 0),
            // right
            (1, 0),
        ]
        .into_iter()
        // can't go backwards
        .filter(|(dx, dy)| (dx * -1, dy * -1) != dir)
        .filter_map(|(dx, dy)| {
            let new_node = (pos.0 + dx, pos.1 + dy);
            if grid.get(&new_node) != Some(&Space::Wall) {
                return Some((new_node, (dx, dy)));
            }
            None
        })
        .for_each(|(new_node, new_dir)| {
            let mut new_cost = cost;
            if new_dir == dir {
                new_cost += 1;
            } else {
                new_cost += 1001;
            }
            match seen.get(&(new_node, new_dir)) {
                Some(&found_cost) => {
                    if found_cost > new_cost {
                        seen.insert((new_node, new_dir), new_cost);
                        heap.push((Reverse(new_cost), (new_node, new_dir)));
                    }
                }
                None => {
                    seen.insert((new_node, new_dir), new_cost);
                    heap.push((Reverse(new_cost), (new_node, new_dir)));
                }
            }
        });
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(solve(input), 7036);
    }

    #[test]
    fn case2() {
        let input = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(solve(input), 11048);
    }
}

util::read_main!();
