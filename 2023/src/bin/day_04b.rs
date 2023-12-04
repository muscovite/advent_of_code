use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    let game_matches: Vec<_> = input
        .lines()
        .map(|l| {
            let (_, l) = l.split_once(":").unwrap();
            let (win_nums, our_nums) = l.split_once("|").unwrap();
            let win_nums: HashSet<_> = win_nums.split_whitespace().collect();
            let our_nums: HashSet<_> = our_nums.split_whitespace().collect();

            win_nums.intersection(&our_nums).count()
        })
        .collect();

    let mut game_counts: Vec<u32> = vec![1; game_matches.len()];
    for (idx, &num_matches) in game_matches.iter().enumerate() {
        if num_matches == 0 {
            continue;
        }

        for i in 1..=num_matches {
            game_counts[idx + i as usize] += game_counts[idx];
        }
    }

    game_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve(input), 30);
    }
}

util::read_main!();
