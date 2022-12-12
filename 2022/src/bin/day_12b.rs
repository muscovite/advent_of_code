use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let mut start_idx = 0;
    let mut end_idx = 0;
    let mut steps_taken = 0;
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    let map: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|l| l.bytes())
        .enumerate()
        .map(|(i, c)| match c {
            b'S' => {
                start_idx = i;
                0
            }
            b'E' => {
                end_idx = i;
                25
            }
            c => c - b'a',
        })
        .collect();
    let mut visited: HashSet<usize> = HashSet::new();
    let map_height = input.lines().count();
    let map_width = map.len() / map_height;
    visited.insert(start_idx);
    frontier.extend(map.iter().enumerate().filter_map(|(idx, height)| {
        if *height == 0 {
            return Some((idx, 0));
        }
        None
    }));

    frontier.push_back((start_idx, 0));

    while let Some((cur_idx, steps)) = frontier.pop_front() {
        if cur_idx == end_idx {
            steps_taken = steps;
            break;
        }
        let steps = steps + 1;

        // up
        if cur_idx / map_width != 0 {
            let neighbor = cur_idx - map_width;
            if !visited.contains(&neighbor) && map[neighbor] <= map[cur_idx] + 1 {
                frontier.push_back((neighbor, steps));
                visited.insert(neighbor);
            }
        }

        // down
        if cur_idx / map_width != map_height - 1 {
            let neighbor = cur_idx + map_width;
            if !visited.contains(&neighbor) && map[neighbor] <= map[cur_idx] + 1 {
                frontier.push_back((neighbor, steps));
                visited.insert(neighbor);
            }
        }

        // left
        if cur_idx % map_width != 0 {
            let neighbor = cur_idx - 1;
            if !visited.contains(&neighbor) && map[neighbor] <= map[cur_idx] + 1 {
                frontier.push_back((neighbor, steps));
                visited.insert(neighbor);
            }
        }
        // right
        if cur_idx % map_width != map_width - 1 {
            let neighbor = cur_idx + 1;
            if !visited.contains(&neighbor) && map[neighbor] <= map[cur_idx] + 1 {
                frontier.push_back((neighbor, steps));
                visited.insert(neighbor);
            }
        }
    }
    steps_taken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(solve(input), 29);
    }
}

util::read_main!();
