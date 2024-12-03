fn get_result(chunk: &str) -> usize {
    chunk
        .split("mul(")
        .filter_map(|chunk| {
            let (chunk, _) = chunk.split_once(")")?;
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

fn solve(input: &str) -> usize {
    let mut total = 0;
    let mut window = input;

    while let Some(dont_idx) = window.find("don't()") {
        let (valid, rest) = window.split_at(dont_idx);
        total += get_result(valid);

        let Some(do_idx) = rest.find("do()") else {
            // nothing left
            return total;
        };
        window = &rest[do_idx..];
    }
    // everything left is valid
    total + get_result(window)
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
