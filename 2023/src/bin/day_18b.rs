use std::cmp::Ordering;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct CoordRange {
    x: i64,
    y: RangeInclusive<i64>,
}

impl Ord for CoordRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}

impl PartialOrd for CoordRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl CoordRange {
    // Hello again, day 5
    // Return area to add and new ranges. We'll always remove self and other
    fn overlaps(&self, other: &CoordRange) -> Option<(usize, Vec<CoordRange>)> {
        // self contains other
        // [......{*******}..]
        if self.y.start() <= other.y.start() && self.y.end() >= other.y.end() {
            let mut rest = vec![];

            if self.y.start() < other.y.start() {
                // leading
                rest.push(CoordRange {
                    x: self.x,
                    y: *self.y.start()..=*other.y.start(),
                });
            }

            if self.y.end() > other.y.end() {
                // trailing
                rest.push(CoordRange {
                    x: self.x,
                    y: *other.y.end()..=*self.y.end(),
                });
            }
            return Some((
                //overlap
                ((1 + (self.x - other.x) as usize)
                    * (1 + (other.y.end() - other.y.start()) as usize)),
                // rest
                rest,
            ));
        }
        // other contains self
        // {-----[****]-}
        else if other.y.start() <= self.y.start() && other.y.end() >= self.y.end() {
            let mut rest = vec![];

            if other.y.start() < self.y.start() {
                // leading
                rest.push(CoordRange {
                    x: other.x,
                    y: *other.y.start()..=*self.y.start(),
                });
            }

            if other.y.end() > self.y.end() {
                // trailing
                rest.push(CoordRange {
                    x: other.x,
                    y: *self.y.end()..=*other.y.end(),
                });
            }

            return Some((
                //overlap
                ((1 + (self.x - other.x) as usize)
                    * (1 + (self.y.end() - self.y.start()) as usize)),
                //rest
                rest,
            ));
        }
        // self range starts lower
        // [....{****]----}
        else if self.y.start() <= other.y.start()
            && self.y.end() > other.y.start()
            && other.y.end() > self.y.end()
        {
            return Some((
                //overlap
                ((1 + (self.x - other.x) as usize)
                    * (1 + (self.y.end() - other.y.start()) as usize)),
                //rest
                vec![
                    CoordRange {
                        x: other.x,
                        y: *self.y.end()..=*other.y.end(),
                    },
                    CoordRange {
                        x: self.x,
                        y: *self.y.start()..=*other.y.start(),
                    },
                ],
            ));
        }
        // other range starts lower
        // {-----[******}....]
        else if other.y.start() <= self.y.start()
            && other.y.end() > self.y.start()
            && self.y.end() > other.y.end()
        {
            return Some((
                //overlap
                ((1 + (self.x - other.x) as usize)
                    * (1 + (other.y.end() - self.y.start()) as usize)),
                //rest
                vec![
                    CoordRange {
                        x: self.x,
                        y: *other.y.end()..=*self.y.end(),
                    },
                    CoordRange {
                        x: other.x,
                        y: *other.y.start()..=*self.y.start(),
                    },
                ],
            ));
        }

        None
    }
}

fn solve(input: &str) -> usize {
    let mut dig_spots: Vec<CoordRange> = vec![];
    let mut curr = (0, 0);

    for l in input.lines() {
        // The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number.
        // The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.
        let (_, hex) = l.strip_suffix(")").unwrap().split_once(" (#").unwrap();
        let dist = &hex[0..hex.len() - 1];
        let dist = i64::from_str_radix(dist, 16).unwrap();

        // We only care about vertical lines now
        match hex.chars().last().unwrap() {
            '3' => {
                // up
                dig_spots.push(CoordRange {
                    x: curr.0,
                    y: (curr.1 - dist..=curr.1),
                });
                curr = (curr.0, curr.1 - dist);
            }
            '1' => {
                // down
                dig_spots.push(CoordRange {
                    x: curr.0,
                    y: curr.1..=curr.1 + dist,
                });
                curr = (curr.0, curr.1 + dist);
            }
            '2' => {
                // left
                curr = (curr.0 - dist, curr.1);
            }
            '0' => {
                // right
                curr = (curr.0 + dist, curr.1);
            }
            _ => panic!("unknown direction"),
        }
    }
    dig_spots.sort();

    let mut lagoon_size = 0;
    let mut horizontal_overlaps: Vec<CoordRange> = vec![];
    while let Some(curr) = dig_spots.pop() {
        // find first intersecting line
        let res = dig_spots.iter().enumerate().rev().find_map(|(idx, l)| {
            if curr.x <= l.x {
                return None;
            }
            match curr.overlaps(l) {
                Some(res) => Some((idx, res)),
                _ => None,
            }
        });
        match res {
            Some((index, (area, rest))) => {
                let other = &dig_spots[index];
                horizontal_overlaps.extend([
                    CoordRange {
                        x: *curr.y.start().max(other.y.start()),
                        y: other.x..=curr.x,
                    },
                    CoordRange {
                        x: *curr.y.end().min(other.y.end()),
                        y: other.x..=curr.x,
                    },
                ]);

                dig_spots.remove(index);
                lagoon_size += area;
                dig_spots.extend(rest);
            }
            _ => unreachable!(),
        }
        dig_spots.sort();
    }

    // calculate overlaps
    while let Some(curr) = horizontal_overlaps.pop() {
        let results: Vec<_> = horizontal_overlaps
            .iter()
            .filter_map(|l| {
                if curr.x != l.x {
                    return None;
                }
                match curr.overlaps(l) {
                    Some(res) => Some(res),
                    _ => None,
                }
            })
            .collect();
        for (area, _) in results {
            lagoon_size -= area;
        }
    }

    lagoon_size
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
        assert_eq!(solve(input), 952408144115);
    }
}

util::read_main!();
