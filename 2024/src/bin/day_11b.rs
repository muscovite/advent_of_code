use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let mut stones: HashMap<usize, usize> = input
        .trim()
        .split_whitespace()
        .map(|stone| (stone.parse::<usize>().unwrap(), 1))
        .collect();

    for _ in 0..75 {
        let mut new_stones = HashMap::new();
        for (stone, count) in stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
                continue;
            }

            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                let (left, right) = (
                    left.parse::<usize>().unwrap(),
                    right.parse::<usize>().unwrap(),
                );
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
                continue;
            }

            let new_stone = stone * 2024;
            *new_stones.entry(new_stone).or_insert(0) += count;
        }
        stones = new_stones;
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"125 17";
        assert_eq!(solve(input), 55312);
    }
}

util::read_main!();
