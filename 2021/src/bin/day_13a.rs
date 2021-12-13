use std::{collections::HashSet, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate(u32, u32);

impl Coordinate {
    fn get_new_dot(&self, instruction: &Instruction) -> Coordinate {
        match instruction {
            Instruction::X(v) => {
                if self.0 <= *v {
                    self.clone()
                } else {
                    let diff = self.0 - v;
                    Coordinate(v - diff, self.1)
                }
            }
            Instruction::Y(v) => {
                if self.1 <= *v {
                    return self.clone();
                } else {
                    let diff = self.1 - v;
                    Coordinate(self.0, v - diff)
                }
            }
        }
    }
}

impl FromStr for Coordinate {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Coordinate(x.parse()?, y.parse()?))
    }
}

#[derive(Debug)]
enum Instruction {
    X(u32),
    Y(u32),
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, instruction) = s.split_once("fold along ").unwrap();
        let (dir, val) = instruction.split_once("=").unwrap();
        let val: u32 = val.parse().unwrap();

        if dir == "x" {
            Ok(Instruction::X(val))
        } else {
            Ok(Instruction::Y(val))
        }
    }
}

fn solve(input: &str) -> usize {
    let (dots, instructions) = input.trim().split_once("\n\n").unwrap();
    let mut dots: HashSet<_> = dots
        .lines()
        .map(|s| Coordinate::from_str(s).unwrap())
        .collect();

    let instructions = instructions
        .lines()
        .map(|s| Instruction::from_str(s).unwrap());

    for instruction in instructions {
        let new_dots = dots.iter().map(|d| d.get_new_dot(&instruction)).collect();
        dots = new_dots;
        break;
    }

    dots.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        assert_eq!(solve(input), 17);
    }
}

util::read_main!();
