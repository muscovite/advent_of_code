use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u64,
    pos: (usize, usize),
}

// Thanks, Rust BinaryHeap docs!
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        // thanks Rust docs, again
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn new(c: char, x: usize, y: usize) -> Node {
        let cost = c.to_digit(10).unwrap() as u64;
        Node { cost, pos: (x, y) }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let grid: Vec<Vec<Node>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Node::new(c, x, y))
                    .collect()
            })
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Grid {
            grid,
            width,
            height,
        }
    }

    fn node_neighbors(&self, node: &Node) -> Vec<Node> {
        let x = node.pos.0;
        let y = node.pos.1;

        let left = x.checked_sub(1).map(|left| (left, y));
        let up = y.checked_sub(1).map(|up| (x, up));

        let right = Some(x + 1)
            .filter(|&x| x < self.width)
            .map(|right| (right, y));
        let down = Some(y + 1)
            .filter(|&y| y < self.height)
            .map(|down| (x, down));

        [left, right, up, down]
            .into_iter()
            .flatten()
            .map(|(x, y)| self.grid[y][x].clone())
            .collect()
    }
}

fn solve(input: &str) -> u64 {
    let grid = Grid::new(input.trim());

    // Frontier
    let mut heap = BinaryHeap::new();
    // Seen
    let mut seen: HashMap<(usize, usize), u64> = HashMap::new();

    let start = &grid.grid[0][0];
    seen.insert((0, 0), start.cost);
    heap.push(start.clone());

    // uniform cost algorithm - shortest path from point A to B
    while let Some(node) = heap.pop() {
        // reached the end
        if node.pos == (grid.width - 1, grid.height - 1) {
            // first node has no "cost"
            return node.cost - grid.grid[0][0].cost;
        }

        // enqueue all neighbors with cumulative cost as priority
        grid.node_neighbors(&node)
            .into_iter()
            .for_each(|mut new_node| {
                new_node.cost += node.cost;

                match seen.get_mut(&new_node.pos) {
                    Some(existing_cost) => {
                        if new_node.cost < *existing_cost {
                            *existing_cost = new_node.cost;
                            heap.push(new_node);
                        }
                    }
                    None => {
                        seen.insert(new_node.pos, new_node.cost);
                        heap.push(new_node);
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
        let input = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        assert_eq!(solve(input), 40);
    }
}

util::read_main!();
