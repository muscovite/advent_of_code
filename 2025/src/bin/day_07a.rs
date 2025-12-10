use std::collections::{HashSet, VecDeque};

fn solve(input: &str) -> usize {
    let mut start: Option<(usize, usize)> = None;
    let grid_len = input.lines().count();
    let splitters: HashSet<(usize, usize)> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as usize, y as usize), c))
        })
        .inspect(|(coord, c)| {
            if *c == 'S' {
                start = Some(*coord);
            }
        })
        .filter_map(|(coord, c)| match c {
            '^' => Some(coord),
            _ => None,
        })
        .collect();

    let mut splitters_hit = 0;
    let mut beams: HashSet<(usize, usize)> = HashSet::new();
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back(start.unwrap());
    beams.insert(start.unwrap());
    while let Some((x, y)) = frontier.pop_front() {
        if y >= grid_len {
            continue;
        }
        let next: (usize, usize) = (x, y + 1);
        if splitters.contains(&next) {
            splitters_hit += 1;
            let beam_l = (x - 1, y + 1);
            let beam_r = (x + 1, y + 1);
            if beams.insert(beam_l) {
                frontier.push_back(beam_l);
            }
            if beams.insert(beam_r) {
                frontier.push_back(beam_r);
            }
            continue;
        }
        if beams.insert(next) {
            frontier.push_back(next);
        }
    }

    splitters_hit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve(input), 21);
    }
}

util::read_main!();
