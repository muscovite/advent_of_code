use std::collections::HashSet;

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
    let num_coords = coords.len();
    let mut areas: HashSet<usize> = HashSet::new();
    for i in 0..num_coords {
        for j in i + 1..num_coords {
            let coord1 = coords[i];
            let coord2 = coords[j];
            let area = coord1.area(&coord2);
            areas.insert(area);
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
        assert_eq!(solve(input), 50);
    }
}

util::read_main!();
