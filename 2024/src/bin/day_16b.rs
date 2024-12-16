use std::cmp::Reverse;
use std::collections::hash_map;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut reindeer = None;
    let mut end = None;
    let walls: HashSet<(isize, isize)> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c))
        })
        .filter_map(|(pos, c)| match c {
            'S' => {
                reindeer = Some(pos);
                None
            }
            'E' => {
                end = Some(pos);
                None
            }
            '#' => Some(pos),
            _ => None,
        })
        .collect();
    let reindeer = reindeer.unwrap();
    let end = end.unwrap();

    // frontier
    let mut heap: BinaryHeap<(
        Reverse<usize>,
        ((isize, isize), (isize, isize), Vec<(isize, isize)>),
    )> = BinaryHeap::new();
    heap.push((Reverse(0), (reindeer, (1, 0), vec![reindeer])));

    // seen
    let mut seen: HashMap<_, _> = HashMap::new();
    seen.insert((reindeer, (1, 0)), 0);
    let mut current_best = None;
    let mut best_spots: HashSet<(isize, isize)> = HashSet::new();

    while let Some((Reverse(cost), (pos, dir, path))) = heap.pop() {
        match current_best {
            None => {
                if pos == end {
                    current_best = Some(cost);
                    best_spots.extend(path);
                    continue;
                }
            }
            Some(curr_best) => {
                if cost > curr_best {
                    continue;
                } else if pos == end {
                    best_spots.extend(path);
                    continue;
                }
            }
        }

        let neighbors = [
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
            if !walls.contains(&new_node) {
                return Some((new_node, (dx, dy)));
            }
            None
        })
        .filter_map(|(new_node, new_dir)| {
            let mut new_cost = cost;
            if new_dir == dir {
                new_cost += 1;
            } else {
                new_cost += 1001;
            }

            match seen.entry((new_node, new_dir)) {
                hash_map::Entry::Occupied(mut o) => {
                    if *o.get() >= new_cost {
                        o.insert(new_cost);
                        return Some((new_node, new_dir, new_cost));
                    }
                    return None;
                }
                hash_map::Entry::Vacant(v) => {
                    v.insert(new_cost);
                    return Some((new_node, new_dir, new_cost));
                }
            }
        })
        .map(|(new_node, new_dir, new_cost)| {
            let mut new_path = path.clone();
            new_path.push(new_node);
            (Reverse(new_cost), (new_node, new_dir, new_path))
        });
        heap.extend(neighbors);
    }
    best_spots.len()
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
        assert_eq!(solve(input), 45);
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
        assert_eq!(solve(input), 64);
    }
}

util::read_main!();
