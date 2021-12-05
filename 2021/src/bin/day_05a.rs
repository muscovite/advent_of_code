use std::{collections::HashMap, str::FromStr};

type Coordinate = (u32, u32);

struct Vent {
    start: Coordinate,
    end: Coordinate,
}

impl Vent {
    fn positions(&self) -> Vec<Coordinate> {
        // vertical line
        if self.start.0 == self.end.0 {
            let fixed = self.start.0;
            if self.start.1 > self.end.1 {
                (self.end.1..=self.start.1).map(|y| (fixed, y)).collect()
            } else {
                (self.start.1..=self.end.1).map(|y| (fixed, y)).collect()
            }
        } else if self.start.1 == self.end.1 {
            // horizontal line
            let fixed = self.start.1;
            if self.start.0 > self.end.0 {
                (self.end.0..=self.start.0).map(|x| (x, fixed)).collect()
            } else {
                (self.start.0..=self.end.0).map(|x| (x, fixed)).collect()
            }
        } else {
            vec![]
        }
    }
}

impl FromStr for Vent {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s.split_once(" -> ").unwrap();
        let coord_start = elems.0.split_once(',').unwrap();
        let coord_end = elems.1.split_once(',').unwrap();

        Ok(Vent {
            start: (
                coord_start.0.parse().unwrap(),
                coord_start.1.parse().unwrap(),
            ),
            end: (coord_end.0.parse().unwrap(), coord_end.1.parse().unwrap()),
        })
    }
}

fn solve(input: &str) -> usize {
    let mut vent_map = HashMap::new();
    let vents: Vec<Vent> = input
        .trim()
        .lines()
        .map(|l| Vent::from_str(l).unwrap())
        .collect();

    for vent in vents {
        for pos in vent.positions() {
            vent_map
                .entry(pos)
                .and_modify(|v| *v += 1)
                .or_insert(1 as u32);
        }
    }

    vent_map.values().filter(|&&v| v >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(solve(input), 5);
    }
}

util::read_main!();
