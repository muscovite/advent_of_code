use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

#[derive(Debug)]
struct OctoGame {
    state: HashMap<Coordinate, u32>,
    flashes: usize,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn neighbors(&self) -> Vec<Coordinate> {
        let x = self.x;
        let y = self.y;

        let left = x.checked_sub(1).map(|left| Coordinate { x: left, y });
        let top = y.checked_sub(1).map(|top| Coordinate { x, y: top });

        let right = Some(x + 1)
            .filter(|&x| x < 10)
            .map(|right| Coordinate { x: right, y });
        let bottom = Some(y + 1)
            .filter(|&y| y < 10)
            .map(|bottom| Coordinate { x, y: bottom });

        let top_left = if left.is_none() || top.is_none() {
            None
        } else {
            Some(Coordinate { x: x - 1, y: y - 1 })
        };

        let top_right = if right.is_none() || top.is_none() {
            None
        } else {
            Some(Coordinate { x: x + 1, y: y - 1 })
        };

        let bottom_left = if left.is_none() || bottom.is_none() {
            None
        } else {
            Some(Coordinate { x: x - 1, y: y + 1 })
        };

        let bottom_right = if right.is_none() || bottom.is_none() {
            None
        } else {
            Some(Coordinate { x: x + 1, y: y + 1 })
        };

        vec![
            left,
            right,
            top,
            bottom,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Display for OctoGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..10).try_for_each(|y| {
            (0..10)
                .try_for_each(|x| write!(f, "{}", self.state.get(&Coordinate { x, y }).unwrap()))?;
            writeln!(f, "")
        })
    }
}

impl OctoGame {
    fn new(input: &str) -> OctoGame {
        let octomap: HashMap<Coordinate, u32> = input
            .trim()
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .enumerate()
            .map(|(idx, level)| {
                (
                    Coordinate {
                        x: idx % 10,
                        y: idx / 10,
                    },
                    level,
                )
            })
            .collect();

        OctoGame {
            state: octomap,
            flashes: 0,
        }
    }

    fn play(&mut self, iters: u32) -> usize {
        for _ in 0..iters {
            self.step();
        }
        self.flashes
    }

    fn step(&mut self) {
        // Increase all counters by one
        self.state.values_mut().for_each(|v| *v += 1);

        let mut to_flash: Vec<Coordinate> = self
            .state
            .iter()
            .filter(|(_, &v)| v > 9)
            .map(|(&k, _)| k)
            .collect();
        let mut already_flashed: HashSet<Coordinate> = to_flash.clone().into_iter().collect();

        while let Some(coord) = to_flash.pop() {
            // Reset state
            let entry = self.state.get_mut(&coord).unwrap();
            *entry = 0;

            let neighbors = coord.neighbors();
            let new_to_check: Vec<_> = neighbors
                .iter()
                .filter(|coord| {
                    if already_flashed.contains(coord) {
                        return false;
                    }

                    let entry = self.state.get_mut(coord).unwrap();
                    *entry += 1;
                    *entry > 9
                })
                .collect();

            to_flash.extend(new_to_check.iter().copied()); // or .clone()
            already_flashed.extend(new_to_check);
        }

        self.flashes += already_flashed.len();
    }
}

fn solve(input: &str) -> usize {
    let mut octogame = OctoGame::new(input);
    octogame.play(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        assert_eq!(solve(input), 1656);
    }
}

util::read_main!();
