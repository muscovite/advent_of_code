use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Node {
    cost: usize,
    pos: (usize, usize),
}

impl Node {
    fn new(c: char, x: usize, y: usize) -> Node {
        let cost = c.to_digit(10).unwrap() as usize;
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
        let chunk: Vec<Vec<Node>> = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| Node::new(c, x, y))
                    .collect()
            })
            .collect();

        let chunk_height = chunk.len();
        let chunk_width = chunk[0].len();
        let mut grid: Vec<Vec<Node>> = vec![vec![]; chunk_height * 5];

        // useful for not having so many for loops!
        for (x, y) in (0..5).flat_map(|y| (0..5).map(move |x| (x, y))) {
            let cur_y = y * chunk_height;
            let cur_x = x * chunk_width;

            for (cx, cy, node) in chunk
                .iter()
                .enumerate()
                .flat_map(|(cy, row)| row.iter().enumerate().map(move |(cx, node)| (cx, cy, node)))
            {
                let mut val = node.cost + x + y;
                if val > 9 {
                    val -= 9;
                }
                grid[cur_y + cy].push(Node {
                    cost: val,
                    pos: (cur_x + cx, cur_y + cy),
                });
            }
        }

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

fn solve(input: &str) -> usize {
    let grid = Grid::new(input.trim());

    // Frontier
    // BinaryHeap<cmp::Reverse<Node>> -> min heap without having to manually
    // implement Ord
    let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
    // Seen
    let mut seen: HashMap<(usize, usize), usize> = HashMap::new();

    let start = &grid.grid[0][0];
    seen.insert((0, 0), start.cost);
    heap.push(Reverse(start.clone()));

    // uniform cost algorithm - shortest path from point A to B
    while let Some(Reverse(node)) = heap.pop() {
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
                            heap.push(Reverse(new_node));
                        }
                    }
                    None => {
                        seen.insert(new_node.pos, new_node.cost);
                        heap.push(Reverse(new_node));
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
        assert_eq!(solve(input), 315);
    }
}

util::read_main!();
