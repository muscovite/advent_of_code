use std::collections::{BTreeSet, HashSet};
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Copy)]
enum FixedPos {
    X(i32),
    Y(i32),
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum VarPos {
    X(RangeInclusive<i32>),
    Y(RangeInclusive<i32>),
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct CoordRange {
    fixed: FixedPos,
    range: VarPos,
}

impl CoordRange {
    fn contains(&self, (other_x, other_y): (i32, i32)) -> bool {
        match self.fixed {
            FixedPos::X(x) => {
                if x != other_x {
                    return false;
                }
            }
            FixedPos::Y(y) => {
                if y != other_y {
                    return false;
                }
            }
        }

        match &self.range {
            VarPos::X(x) => return x.contains(&other_x),
            VarPos::Y(y) => return y.contains(&other_y),
        }
    }

    fn len(&self) -> usize {
        match &self.range {
            VarPos::X(x) => (x.end() - x.start()) as usize,
            VarPos::Y(y) => (y.end() - y.start()) as usize,
        }
    }
}

fn solve(input: &str) -> usize {
    let mut dig_spots: Vec<CoordRange> = vec![];
    let mut curr = (0, 0);

    for l in input.lines() {
        let mut l = l.split_whitespace();
        let dir = l.next().unwrap();
        let dist: i32 = l.next().unwrap().parse().unwrap();
        // ignore color for now

        match dir {
            "U" => {
                dig_spots.push(CoordRange {
                    fixed: FixedPos::X(curr.0),
                    range: VarPos::Y(curr.1 - dist..=curr.1),
                });
                curr = (curr.0, curr.1 - dist);
            }
            "D" => {
                dig_spots.push(CoordRange {
                    fixed: FixedPos::X(curr.0),
                    range: VarPos::Y(curr.1..=curr.1 + dist),
                });
                curr = (curr.0, curr.1 + dist);
            }
            "L" => {
                dig_spots.push(CoordRange {
                    fixed: FixedPos::Y(curr.1),
                    range: VarPos::X(curr.0 - dist..=curr.0),
                });
                curr = (curr.0 - dist, curr.1);
            }
            "R" => {
                dig_spots.push(CoordRange {
                    fixed: FixedPos::Y(curr.1),
                    range: VarPos::X(curr.0..=curr.0 + dist),
                });
                curr = (curr.0 + dist, curr.1);
            }
            _ => panic!("unknown direction"),
        }
    }

    // Find some starting point
    let min_x = dig_spots
        .iter()
        .map(|c| match c.fixed {
            FixedPos::X(x) => x,
            _ => match &c.range {
                VarPos::X(range) => *range.start(),
                _ => unreachable!(),
            },
        })
        .min()
        .unwrap();
    let max_x = dig_spots
        .iter()
        .map(|c| match c.fixed {
            FixedPos::X(x) => x,
            _ => match &c.range {
                VarPos::X(range) => *range.end(),
                _ => unreachable!(),
            },
        })
        .max()
        .unwrap();
    let min_y = dig_spots
        .iter()
        .map(|c| match c.fixed {
            FixedPos::Y(y) => y,
            _ => match &c.range {
                VarPos::Y(range) => *range.start(),
                _ => unreachable!(),
            },
        })
        .min()
        .unwrap();

    let mut edges: Vec<(i32, i32)> = (min_x..=max_x)
        .map(|x| (x, min_y + 1))
        .filter(|coord| dig_spots.iter().any(|c| c.contains(*coord)))
        .collect();
    edges.sort_by_key(|(x, _)| *x);
    let flood_start = (edges[0].0 + 1, edges[0].1);

    let mut frontier = BTreeSet::from_iter([flood_start]);
    let mut seen = HashSet::new();
    let mut lagoon_size = 0;
    while let Some((x, y)) = frontier.pop_first() {
        lagoon_size += 1;
        seen.insert((x, y));
        frontier.extend(
            [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|coord| {
                    !dig_spots.iter().any(|c| c.contains(*coord)) && !seen.contains(&(coord))
                }),
        );
        if frontier.len() == 0 {
            return lagoon_size + dig_spots.into_iter().map(|c| c.len()).sum::<usize>();
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(solve(input), 62);
    }
}

util::read_main!();
