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
    input
        .trim()
        .lines()
        .map(|l| {
            let mut item_set1 = HashSet::new();
            let mut item_set2 = HashSet::new();
            let part_size = l.len() / 2;
            // string slices!
            let part1 = &l[..part_size];
            let part2 = &l[part_size..];

            part1.bytes().for_each(|b| {
                item_set1.insert(get_priority(b));
            });
            part2.bytes().for_each(|b| {
                item_set2.insert(get_priority(b));
            });
            item_set1.intersection(&item_set2).next().unwrap().clone()
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
        assert_eq!(solve(input), 157);
    }
}

util::read_main!();
