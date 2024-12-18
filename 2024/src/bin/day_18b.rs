use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

fn simulate(input: &str, steps: usize, exit_pos: (isize, isize)) -> bool {
    let corrupted_spaces: HashSet<(isize, isize)> = input
        .trim()
        .lines()
        .take(steps)
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    // frontier
    let mut heap: BinaryHeap<(Reverse<usize>, (isize, isize))> = BinaryHeap::new();
    heap.push((Reverse(0), (0, 0)));

    // seen
    let mut seen: HashMap<_, _> = HashMap::new();
    seen.insert((0, 0), 0);

    while let Some((Reverse(cost), pos)) = heap.pop() {
        // reached the end
        if pos == exit_pos {
            return true;
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
        .filter_map(|(dx, dy)| {
            let new_space = (pos.0 + dx, pos.1 + dy);
            if !corrupted_spaces.contains(&new_space)
                && (0..=exit_pos.0).contains(&new_space.0)
                && (0..=exit_pos.1).contains(&new_space.1)
            {
                return Some(new_space);
            }
            None
        })
        .for_each(|new_space| {
            let new_cost = cost + 1;
            match seen.get(&new_space) {
                Some(&found_cost) => {
                    if found_cost > new_cost {
                        seen.insert(new_space, new_cost);
                        heap.push((Reverse(new_cost), new_space));
                    }
                }
                None => {
                    seen.insert(new_space, new_cost);
                    heap.push((Reverse(new_cost), new_space));
                }
            }
        });
    }

    false
}

fn solve(input: &str) -> &str {
    for steps in 1024..input.lines().count() {
        if !simulate(input, steps, (70, 70)) {
            return input.lines().nth(steps - 1).unwrap();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!(simulate(input, 21, (6, 6)), false);
    }
}

util::read_main!();
