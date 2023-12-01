fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .map(|nums: Vec<_>| (10 * nums.iter().next().unwrap()) + nums.iter().last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve(input), 142);
    }
}

util::read_main!();
