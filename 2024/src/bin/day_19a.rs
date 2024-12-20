use std::{
    collections::{hash_map, HashMap, HashSet},
    vec,
};

#[derive(Debug, Clone)]
struct TrieNode {
    c: char,
    has_terminal: bool,
    tier: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new(c: char, is_terminal: bool) -> TrieNode {
        TrieNode {
            c: c,
            has_terminal: is_terminal,
            tier: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Trie {
    starting_tier: HashMap<char, TrieNode>,
}

impl Trie {
    fn new(towels: &str) -> Trie {
        let mut trie = Trie {
            starting_tier: HashMap::new(),
        };

        for towel in towels.split(", ") {
            trie.insert(towel);
        }

        trie
    }

    fn insert(&mut self, chunk: &str) {
        let last_idx = chunk.len() - 1;
        let mut curr_tier = &mut self.starting_tier;

        for i in 0..=last_idx {
            let c = chunk.chars().nth(i).unwrap();
            match curr_tier.entry(c) {
                hash_map::Entry::Occupied(mut o) => {
                    o.get_mut().has_terminal |= i == last_idx;
                    curr_tier = &mut o.into_mut().tier;
                }
                hash_map::Entry::Vacant(v) => {
                    let new_node = TrieNode::new(c, i == last_idx);
                    curr_tier = &mut v.insert(new_node).tier;
                }
            }
        }
    }

    fn get_start_node(&self, c: char) -> Option<&TrieNode> {
        self.starting_tier.get(&c)
    }

    fn can_make_pattern(&self, pattern: &str) -> bool {
        let last_idx = pattern.len() - 1;
        let Some(start_node) = self.get_start_node(pattern.chars().next().unwrap()) else {
            return false;
        };

        let mut frontier: Vec<&TrieNode> = vec![start_node];

        for i in 1..=last_idx {
            let mut next_nodes: Vec<&TrieNode> = vec![];
            let c = pattern.chars().nth(i).unwrap();
            let mut terminals: HashSet<char> = HashSet::new();
            while let Some(node) = frontier.pop() {
                if node.has_terminal {
                    if i == last_idx {
                        return true;
                    }
                    terminals.insert(c);
                }
                if let Some(next_node) = node.tier.get(&c) {
                    next_nodes.push(next_node);
                };
            }
            next_nodes.extend(terminals.into_iter().flat_map(|c| self.get_start_node(c)));
            if next_nodes.len() == 0 {
                break;
            }
            frontier = next_nodes;
        }

        false
    }
}

fn solve(input: &str) -> usize {
    let (towels, patterns) = input.trim().split_once("\n\n").unwrap();

    let towels: Trie = Trie::new(towels);

    patterns
        .lines()
        .filter(|l| {
            let foo = towels.can_make_pattern(l);
            foo
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn case2() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

ubwu";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case3() {
        let input = r"r, wr, b, g, bwu, rb, gb, br

bbrgwb";
        assert_eq!(solve(input), 0);
    }
}

util::read_main!();
