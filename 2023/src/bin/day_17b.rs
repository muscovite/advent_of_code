use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Copy)]
enum Dir {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Copy)]
struct Node {
    pos: (usize, usize),
    dir: Dir,
}

fn node_neighbors(node: &Node, grid: &Vec<Vec<usize>>) -> Vec<Node> {
    let mut neighbors = vec![];
    let (x, y) = node.pos;

    // up = 0, down = 1, left = 2, right = 3
    let forbidden_dirs: [u8; 4] = match node.dir {
        Dir::Up(dist) => {
            if dist < 4 {
                [0, 1, 1, 1]
            } else if dist == 10 {
                [1, 1, 0, 0]
            } else {
                [0, 1, 0, 0]
            }
        }
        Dir::Down(dist) => {
            if dist < 4 {
                [1, 0, 1, 1]
            } else if dist == 10 {
                [1, 1, 0, 0]
            } else {
                [1, 0, 0, 0]
            }
        }
        Dir::Left(dist) => {
            if dist < 4 {
                [1, 1, 0, 1]
            } else if dist == 10 {
                [0, 0, 1, 1]
            } else {
                [0, 0, 0, 1]
            }
        }
        Dir::Right(dist) => {
            if dist < 4 {
                [1, 1, 1, 0]
            } else if dist == 10 {
                [0, 0, 1, 1]
            } else {
                [0, 0, 1, 0]
            }
        }
    };

    // up
    if forbidden_dirs[0] != 1 {
        if let Some((x, y)) = y.checked_sub(1).map(|up| (x, up)) {
            let new_dist = match node.dir {
                Dir::Up(dist) => dist + 1,
                _ => 1,
            };
            neighbors.push(Node {
                pos: (x, y),
                dir: Dir::Up(new_dist),
            })
        }
    }

    // down
    if forbidden_dirs[1] != 1 {
        if let Some((x, y)) = Some(y + 1)
            .filter(|&y| y < grid.len())
            .map(|down| (x, down))
        {
            let new_dist = match node.dir {
                Dir::Down(dist) => dist + 1,
                _ => 1,
            };
            neighbors.push(Node {
                pos: (x, y),
                dir: Dir::Down(new_dist),
            })
        }
    }

    // left
    if forbidden_dirs[2] != 1 {
        if let Some((x, y)) = x.checked_sub(1).map(|left| (left, y)) {
            let new_dist = match node.dir {
                Dir::Left(dist) => dist + 1,
                _ => 1,
            };
            neighbors.push(Node {
                pos: (x, y),
                dir: Dir::Left(new_dist),
            })
        }
    }

    // right
    if forbidden_dirs[3] != 1 {
        if let Some((x, y)) = Some(x + 1)
            .filter(|&x| x < grid[0].len())
            .map(|right| (right, y))
        {
            let new_dist = match node.dir {
                Dir::Right(dist) => dist + 1,
                _ => 1,
            };
            neighbors.push(Node {
                pos: (x, y),
                dir: Dir::Right(new_dist),
            })
        }
    }

    neighbors
}

// uniform cost search
fn solve(input: &str) -> usize {
    let grid: Vec<Vec<usize>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    // frontier
    let mut heap: BinaryHeap<(Reverse<usize>, Node)> = BinaryHeap::new();
    let start1 = Node {
        pos: (0, 0),
        dir: Dir::Right(0),
    };
    let start2 = Node {
        pos: (0, 0),
        dir: Dir::Down(0),
    };
    heap.push((Reverse(0), start1));
    heap.push((Reverse(0), start2));
    // seen
    let mut seen: HashSet<Node> = HashSet::new();
    seen.insert(start1);
    seen.insert(start2);

    while let Some((Reverse(cost), node)) = heap.pop() {
        // reached the end
        if node.pos == (grid[0].len() - 1, grid.len() - 1) {
            let dist = match node.dir {
                Dir::Up(dist) => dist,
                Dir::Down(dist) => dist,
                Dir::Left(dist) => dist,
                Dir::Right(dist) => dist,
            };
            if dist >= 4 {
                return cost;
            }
        }

        // enqueue all neighbors with cumulative cost as priority
        node_neighbors(&node, &grid)
            .into_iter()
            .for_each(|new_node| {
                if seen.insert(new_node) {
                    let new_cost = grid[new_node.pos.1][new_node.pos.0];
                    heap.push((Reverse(cost + new_cost), new_node));
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
        let input = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(solve(input), 94);
    }

    #[test]
    fn case2() {
        let input = r"111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(solve(input), 71);
    }
}

util::read_main!();
