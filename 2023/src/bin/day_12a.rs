use itertools::Itertools;
use std::collections::HashSet;

// Possible optimization
//
// filtered_powerset: only return index combinations that are valid up to current evaluation point?
// And still need a final check to determine full validity?

fn num_valid(record: &str, counts: &str) -> usize {
    let counts: Vec<usize> = counts.split(",").map(|c| c.parse().unwrap()).collect();
    record
        .char_indices()
        .filter_map(|(idx, c)| if c == '?' { return Some(idx) } else { None })
        .powerset()
        .filter(|changes| {
            let changes: HashSet<_> = changes.iter().collect();
            let record: String = record
                .char_indices()
                .map(|(idx, c)| {
                    if changes.contains(&idx) {
                        '#'
                    } else if c == '?' {
                        '.'
                    } else {
                        c
                    }
                })
                .collect();
            is_valid(record, &counts)
        })
        .count()
}

fn is_valid(record: String, counts: &Vec<usize>) -> bool {
    return record
        .split(".")
        .filter_map(|c| if c != "" { return Some(c.len()) } else { None })
        .collect::<Vec<usize>>()
        == *counts;
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, counts) = l.split_once(" ").unwrap();
            num_valid(springs, counts)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), 21);
    }
}

util::read_main!();
