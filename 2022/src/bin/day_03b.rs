use std::collections::HashSet;

fn get_priority(b: u8) -> usize {
    match b < 97 {
        // uppercase
        true => (b - 38).into(),
        // lowercase
        false => (b - 96).into(),
    }
}

fn solve(input: &str) -> usize {
    // Lowercase item types a through z have priorities 1 through 26. a == 97
    // Uppercase item types A through Z have priorities 27 through 52. A == 65
    let lines: Vec<_> = input.trim().lines().collect();

    lines
        .chunks(3)
        .map(|chunk| {
            let sets = chunk
                .iter()
                .map(|group| group.bytes().collect::<HashSet<_>>());
            let badge = sets
                .reduce(|acc, set| &acc & &set)
                .unwrap()
                .into_iter()
                .next()
                .unwrap();
            get_priority(badge)
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve(input), 70);
    }
}

util::read_main!();
