fn solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|food| food.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(solve(input), 24000);
    }
}

util::read_main!();
