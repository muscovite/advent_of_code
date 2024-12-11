fn generate(stone: usize, steps: usize) -> usize {
    if steps == 25 {
        return 1;
    }
    let steps = steps + 1;

    if stone == 0 {
        return generate(1, steps);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        return generate(left.parse::<usize>().unwrap(), steps)
            + generate(right.parse::<usize>().unwrap(), steps);
    }

    return generate(stone * 2024, steps);
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .split_whitespace()
        .map(|stone| generate(stone.parse::<usize>().unwrap(), 0))
        .sum()
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
