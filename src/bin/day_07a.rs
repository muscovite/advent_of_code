use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
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
        let children = rest.split(',').filter_map(|chunk| {
            if chunk == " no other bags." {
                return None;
            }
            let child_capture = RULE_RE.captures(chunk).unwrap();
            Some(child_capture.name("rule_type").unwrap().as_str())
        });

        // Add parent to each child's parent list
        for child in children {
            bag_map.entry(child).or_insert(vec![]).push(parent);
        }
    }

    // BFS to find all bags that contain gold
    let mut gold_holders: HashSet<&str> = HashSet::new();
    let mut remaining = VecDeque::from_iter(bag_map.get("shiny gold").unwrap());

    while remaining.len() > 0 {
        let curr = remaining.pop_front().unwrap();
        gold_holders.insert(curr);
        if !bag_map.contains_key(curr) {
            continue;
        }
        for color in bag_map.get(curr).unwrap() {
            remaining.push_back(color);
        }
    }

    gold_holders.len()
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
        assert_eq!(solve(input), 4);
    }
}

advent_2020::read_main!();
