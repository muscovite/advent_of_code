use std::collections::HashSet;

fn get_count(input: &str) -> usize {
    let mut groups = input.lines().map(|l| l.chars().collect::<HashSet<_>>());

    let seed = groups.next().unwrap();
    groups.fold(seed, |group1, group2| &group1 & &group2).len()
}

fn solve(input: &str) -> usize {
    input.trim().split("\n\n").map(get_count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve(input), 6);
    }
}

util::read_main!();
