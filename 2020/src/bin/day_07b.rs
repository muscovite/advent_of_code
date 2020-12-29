use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

lazy_static! {
    static ref PARENT_RE: Regex =
        Regex::new(r"(?P<parent>\w+ \w+) bags contain(?P<rest>.+)").unwrap();
    static ref RULE_RE: Regex =
        Regex::new(r" (?P<count>\d+) (?P<rule_type>\w+ \w+) (bag|bags)(\.)*").unwrap();
}

fn solve(input: &str) -> usize {
    let mut bag_map: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in input.trim().lines() {
        // Get root/parent bag color
        let parent_captures = PARENT_RE.captures(l).unwrap();
        let parent = parent_captures.name("parent").unwrap().as_str();
        // Get child bag colors
        let rest = parent_captures.name("rest").unwrap().as_str();
        let children: Vec<&str> = rest
            .split(',')
            .filter_map(|chunk| {
                if chunk == " no other bags." {
                    return None;
                }
                let child_capture = RULE_RE.captures(chunk).unwrap();
                let quantity: usize = child_capture
                    .name("count")
                    .unwrap()
                    .as_str()
                    .parse()
                    .unwrap();
                let color = child_capture.name("rule_type").unwrap().as_str();
                Some(std::iter::repeat(color).take(quantity).collect::<Vec<_>>())
            })
            .flatten()
            .collect();

        if children.len() > 0 {
            bag_map.insert(parent, children);
        }
    }

    // BFS to find total number of bags needed
    let mut count: usize = 0;
    let mut remaining = VecDeque::from_iter(bag_map.get("shiny gold").unwrap());

    while remaining.len() > 0 {
        let curr = remaining.pop_front().unwrap();
        count += 1;
        if !bag_map.contains_key(curr) {
            continue;
        }
        remaining.extend(bag_map.get(curr).unwrap());
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(solve(input), 32);
    }

    #[test]
    fn case2() {
        let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(solve(input), 126);
    }
}

advent_2020::read_main!();
