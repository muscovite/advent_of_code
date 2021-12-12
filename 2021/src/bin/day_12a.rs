use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Cave<'a> {
    Upper(&'a str),
    Lower(&'a str),
    Start,
    End,
}

impl<'a> Cave<'a> {
    fn new(input: &'a str) -> Cave<'a> {
        if input == "start" {
            Cave::Start
        } else if input == "end" {
            Cave::End
        } else if input.chars().any(|c| c.is_ascii_lowercase()) {
            Cave::Lower(input)
        } else {
            Cave::Upper(input)
        }
    }
}

fn solve(input: &str) -> u32 {
    let connections = input
        .trim()
        .lines()
        .map(|line| line.split_once('-').unwrap());

    let start = Cave::new("start");
    let end = Cave::new("end");

    let mut caves: HashMap<Cave, Vec<Cave>> = HashMap::new();
    connections.for_each(|(k, v)| {
        let k = Cave::new(k);
        let v = Cave::new(v);

        if k != end && v != start {
            caves.entry(k).or_insert(vec![]).push(v);
        }
        if k != start && v != end {
            caves.entry(v).or_insert(vec![]).push(k);
        }
    });

    let mut frontier = vec![vec![start]];
    let mut num_paths = 0;

    while let Some(path) = frontier.pop() {
        let mut path = path;
        let curr_cave = path.pop().unwrap();

        if curr_cave == end {
            num_paths += 1;
            continue;
        }

        if let Cave::Lower(_) = curr_cave {
            if path.contains(&curr_cave) {
                continue;
            }
        }

        let new_caves = caves.get(&curr_cave).unwrap();
        frontier.extend(new_caves.iter().map(|&new_cave| {
            let mut new_path = path.clone();
            new_path.push(curr_cave);
            new_path.push(new_cave);
            new_path
        }));
    }
    num_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn case2() {
        let input = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        assert_eq!(solve(input), 19);
    }

    #[test]
    fn case3() {
        let input = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        assert_eq!(solve(input), 226);
    }
}

util::read_main!();
