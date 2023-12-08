use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(|l| {
            // format: AAA = (BBB, CCC)
            (
                l[0..=2].to_owned(),
                (l[7..=9].to_owned(), l[12..=14].to_owned()),
            )
        })
        .collect();

    let mut curr: &String = &"AAA".to_owned();
    for (n, c) in instructions.chars().cycle().enumerate() {
        curr = match c {
            'L' => &nodes.get(curr).unwrap().0,
            'R' => &nodes.get(curr).unwrap().1,
            _ => panic!("invalid instruction"),
        };

        if curr == "ZZZ" {
            return n + 1;
        }
    }
    panic!("should not reach here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn case2() {
        let input = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 6);
    }
}

util::read_main!();
