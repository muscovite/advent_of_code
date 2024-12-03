fn solve(input: &str) -> usize {
    input
        .split("mul(")
        .flat_map(|chunks| chunks.split_once(")"))
        .filter_map(|(chunk, _)| {
            let (left, right) = chunk.split_once(",")?;
            let left = left.parse::<usize>().ok()?;
            let right = right.parse::<usize>().ok()?;

            if left > 1000 || right > 1000 {
                return None;
            }
            Some(left * right)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(solve(input), 161);
    }
}

util::read_main!();
