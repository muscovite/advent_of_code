fn solve(input: &str) -> i32 {
    let mut positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    positions.sort_unstable();

    let median = positions[positions.len() / 2];

    positions.iter().map(|pos| (pos - median).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve(input), 37);
    }
}

util::read_main!();
