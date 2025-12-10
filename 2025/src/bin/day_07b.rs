use std::collections::{HashMap, HashSet, VecDeque, hash_map::Entry};

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

    let mut timelines = 0;
    let mut beams: HashMap<(usize, usize), usize> = HashMap::new();
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    frontier.push_back(start.unwrap());
    beams.insert(start.unwrap(), 1);
    while let Some(coord) = frontier.pop_front() {
        let (x, y) = coord;
        let timelines_coord = *beams.get(&coord).unwrap();
        if y >= grid_len {
            timelines += timelines_coord;
            continue;
        }
        let next: (usize, usize) = (x, y + 1);
        let mut update_beam = |new_beam| match beams.entry(new_beam) {
            Entry::Occupied(o) => {
                *o.into_mut() += timelines_coord;
            }
            Entry::Vacant(v) => {
                frontier.push_back(new_beam);
                v.insert(timelines_coord);
            }
        };
        if splitters.contains(&next) {
            let beam_l = (x - 1, y + 1);
            let beam_r = (x + 1, y + 1);
            update_beam(beam_l);
            update_beam(beam_r);
            continue;
        }
        update_beam(next);
    }

    timelines
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
        assert_eq!(solve(input), 40);
    }
}

util::read_main!();
