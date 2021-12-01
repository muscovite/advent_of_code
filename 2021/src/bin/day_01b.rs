fn solve(input: &str) -> usize {
    let nums: Vec<u32> = input
        .trim()
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    nums.windows(3)
        .map(|item| item.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|item| item[1] > item[0])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"199
200
208
210
200
207
240
269
260
263";
        assert_eq!(solve(input), 5);
    }
}

util::read_main!();
