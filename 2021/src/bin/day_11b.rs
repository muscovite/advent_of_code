use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
};

#[derive(Debug)]
struct OctoGame {
    state: HashMap<Coordinate, u32>,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn neighbors(&self) -> impl Iterator<Item = Coordinate> {
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

        [
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

        OctoGame { state: octomap }
    }

    fn is_synchronized(&self) -> bool {
        self.state.values().all(|v| *v == 0)
    }

    fn play(&mut self) -> usize {
        let mut steps = 0;
        while !self.is_synchronized() {
            self.step();
            steps += 1;
        }
        steps
    }

    fn step(&mut self) {
        // Increase all counters by one
        self.state.values_mut().for_each(|v| *v = (*v + 1) % 10);

        let mut to_flash: Vec<Coordinate> = self
            .state
            .iter()
            .filter(|(_, &v)| v == 0)
            .map(|(&k, _)| k)
            .collect();
        let mut already_flashed: HashSet<Coordinate> = to_flash.clone().into_iter().collect();

        while let Some(coord) = to_flash.pop() {
            let neighbors = coord.neighbors();
            let new_to_check: Vec<_> = neighbors
                .filter(|coord| {
                    if already_flashed.contains(coord) {
                        return false;
                    }

                    let entry = self.state.get_mut(coord).unwrap();
                    *entry = (*entry + 1) % 10;
                    *entry == 0
                })
                .collect();

            to_flash.extend(new_to_check.iter().copied()); // or .clone()
            already_flashed.extend(new_to_check);
        }
    }
}

fn solve(input: &str) -> usize {
    let mut octogame = OctoGame::new(input);
    octogame.play()
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
        assert_eq!(solve(input), 195);
    }
}

util::read_main!();
