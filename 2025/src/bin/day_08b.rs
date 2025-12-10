use std::cmp;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Coord {
    fn pythag(&self, other: &Coord) -> usize {
        (self.x.abs_diff(other.x)).pow(2)
            + (self.y.abs_diff(other.y)).pow(2)
            + (self.z.abs_diff(other.z)).pow(2)
    }
}

#[derive(Debug, Clone)]
struct CoordPair {
    coord1: Coord,
    coord2: Coord,
    distance: usize,
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
                z: nums.next().unwrap(),
            }
        })
        .collect();
    let num_coords = coords.len();
    let mut pairs: Vec<CoordPair> = Vec::with_capacity(((num_coords - 1) * num_coords) / 2);
    for i in 0..num_coords {
        for j in i + 1..num_coords {
            let coord1 = coords[i];
            let coord2 = coords[j];
            let distance = coord1.pythag(&coord2);
            pairs.push(CoordPair {
                coord1,
                coord2,
                distance: distance,
            });
        }
    }
    pairs.sort_by_key(|pair| pair.distance);
    let mut circuits: Vec<HashSet<Coord>> = coords
        .into_iter()
        .map(|c| {
            let mut hashset = HashSet::new();
            hashset.insert(c);
            hashset
        })
        .collect();
    for (i, pair) in pairs.iter().enumerate() {
        let (idx1, _) = circuits
            .iter()
            .enumerate()
            .find(|(_, c)| c.contains(&pair.coord1))
            .unwrap();
        let (idx2, _) = circuits
            .iter()
            .enumerate()
            .find(|(_, c)| c.contains(&pair.coord2))
            .unwrap();
        if idx1 == idx2 {
            continue;
        }
        if circuits.len() == 2 {
            return pair.coord1.x * pair.coord2.x;
        }
        let removed = circuits.swap_remove(cmp::max(idx1, idx2));
        circuits[cmp::min(idx1, idx2)].extend(removed.iter());
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve(input), 25272);
    }
}

util::read_main!();
