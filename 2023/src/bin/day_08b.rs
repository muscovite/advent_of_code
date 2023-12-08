use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Node {
    A(String),
    Z(String),
    Reg(String),
}

impl Node {
    fn new(s: &str) -> Node {
        match s.chars().last() {
            Some('A') => Node::A(s.to_owned()),
            Some('Z') => Node::Z(s.to_owned()),
            _ => Node::Reg(s.to_owned()),
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        let remainder = a % b;
        gcd(b, remainder)
    }
}

fn solve(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let nodes: HashMap<_, _> = nodes
        .lines()
        .map(|l| {
            // format: AAA = (BBB, CCC)
            (
                Node::new(&l[0..=2]),
                (Node::new(&l[7..=9]), Node::new(&l[12..=14])),
            )
        })
        .collect();

    let mut currs: Vec<&Node> = nodes
        .iter()
        .filter_map(|(node, _)| {
            if matches!(node, Node::A(_)) {
                return Some(node);
            }
            None
        })
        .collect();

    let mut first_z = vec![0; currs.len()];

    for (n, c) in instructions.chars().cycle().enumerate() {
        for i in 0..currs.len() {
            if first_z[i] > 0 {
                continue;
            }

            currs[i] = match c {
                'L' => &nodes.get(currs[i]).unwrap().0,
                'R' => &nodes.get(currs[i]).unwrap().1,
                _ => panic!("invalid instruction"),
            };

            if matches!(currs[i], Node::Z(_)) {
                first_z[i] = n + 1;
            }
        }

        if first_z.iter().all(|&i| i > 0) {
            break;
        }
    }

    // find LCM ?? products / greatest common denominator
    // Does this always work or does it just happen to work??
    // Unanswered questions:
    // - What if there was a leadup before the cycle starts?
    // - What if there's another Z-node mid cycle?
    first_z
        .into_iter()
        .reduce(|a, b| a / gcd(a, b) * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve(input), 6);
    }
}

util::read_main!();
