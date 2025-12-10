use std::cmp::{Ordering, max, min};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn area(&self, other: &Coord) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn get_area(
    horiz_lines: &Vec<(RangeInclusive<usize>, usize)>,
    coord1: &Coord,
    coord2: &Coord,
) -> Option<usize> {
    for x in min(coord1.x, coord2.x)..=max(coord1.x, coord2.x) {
        for y_bound in vec![min(coord1.y, coord2.y), max(coord1.y, coord2.y)].into_iter() {
            let num_intersections = horiz_lines
                .iter()
                .filter(|(x_range, y)| x_range.contains(&x) && *y >= y_bound)
                .count();
            dbg!(coord1, coord2, num_intersections);
            if num_intersections == 0 {
                // on an edge
                continue;
            }
            if num_intersections % 2 == 0 {
                return None;
            }
        }
    }
    Some(coord1.area(&coord2))
}

fn solve(input: &str) -> usize {
    let coords: Vec<Coord> = input
        .trim()
        .lines()
        .map(|l| {
            let mut nums = l.split(",").map(|d| d.parse().unwrap());
            Coord {
                x: nums.next().unwrap(),
                y: nums.next().unwrap(),
            }
        })
        .collect();

    let horiz_lines: Vec<_> = coords
        .windows(2)
        .filter_map(|w| match w[0].x.cmp(&w[1].x) {
            Ordering::Greater => Some((w[1].x..=w[0].x, w[0].y)),
            Ordering::Less => Some((w[0].x..=w[1].x, w[0].y)),
            _ => None,
        })
        .collect();

    let num_coords = coords.len();
    let mut areas: HashSet<usize> = HashSet::new();
    for i in 0..num_coords {
        for j in i + 1..num_coords {
            let coord1 = coords[i];
            let coord2 = coords[j];
            match get_area(&horiz_lines, &coord1, &coord2) {
                Some(area) => {
                    areas.insert(area);
                }
                None => (),
            }
        }
    }
    *areas.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve(input), 24);
    }
}

util::read_main!();
