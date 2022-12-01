fn solve(input: &str) -> usize {
    let mut cals: Vec<usize> = input
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|food| food.parse::<usize>().unwrap()).sum())
        .collect();

    cals.sort_by(|a, b| b.cmp(a));
    cals.into_iter().take(3).sum()
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
        assert_eq!(solve(input), 45000);
    }
}

util::read_main!();
