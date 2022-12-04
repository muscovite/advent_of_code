use std::num::ParseIntError;
use std::str::FromStr;

struct Assignments {
    lower1: u32,
    upper1: u32,
    lower2: u32,
    upper2: u32,
}

// Note to self: range.contains() does similar checks under the hood
impl Assignments {
    fn has_containment(self) -> bool {
        return self.lower1 <= self.lower2 && self.upper1 >= self.upper2
            || self.lower2 <= self.lower1 && self.upper2 >= self.upper1;
    }

    fn has_overlap(self) -> bool {
        return self.upper1 >= self.lower2 && self.lower1 <= self.upper2
            || self.upper2 >= self.lower1 && self.lower2 <= self.upper1;
    }
}

impl FromStr for Assignments {
    type Err = ParseIntError;

    fn from_str(l: &str) -> Result<Self, Self::Err> {
        let mut bounds = l
            .split(',')
            .flat_map(|s| s.split('-').map(|n| n.parse().unwrap()));

        Ok(Assignments {
            lower1: bounds.next().unwrap(),
            upper1: bounds.next().unwrap(),
            lower2: bounds.next().unwrap(),
            upper2: bounds.next().unwrap(),
        })
    }
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<Assignments>().unwrap().has_overlap())
        .filter(|f| *f)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(solve(input), 4);
    }
}

util::read_main!();
