fn solve(input: &str) -> usize {
    let nums: Vec<u32> = input.trim().lines().map(|s| s.parse().unwrap()).collect();

    // windows = sliding window
    nums.windows(2).filter(|item| item[1] > item[0]).count()
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
        assert_eq!(solve(input), 7);
    }
}

util::read_main!();
