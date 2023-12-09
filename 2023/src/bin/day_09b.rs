fn sum_of_diffs(nums: &Vec<i32>) -> i32 {
    let mut diffs = Vec::with_capacity(nums.len() - 1);
    for i in 0..nums.len() - 1 {
        diffs.push(nums[i + 1] - nums[i]);
    }

    if diffs.iter().min() == diffs.iter().max() {
        return diffs[0];
    }

    diffs.last().unwrap() + sum_of_diffs(&diffs)
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let nums: Vec<i32> = l
                .split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .rev() // lol
                .collect();
            nums.last().unwrap() + sum_of_diffs(&nums)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(input), 2);
    }
}

util::read_main!();
