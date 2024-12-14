fn solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            let mut chunk = chunk.lines();
            let (a_x, a_y) = chunk
                .next()
                .unwrap()
                .strip_prefix("Button A: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();
            let (a_x, a_y) = (a_x.parse::<usize>().unwrap(), a_y.parse::<usize>().unwrap());

            let (b_x, b_y) = chunk
                .next()
                .unwrap()
                .strip_prefix("Button B: X+")
                .unwrap()
                .split_once(", Y+")
                .unwrap();
            let (b_x, b_y) = (b_x.parse::<usize>().unwrap(), b_y.parse::<usize>().unwrap());

            let (prize_x, prize_y) = chunk
                .next()
                .unwrap()
                .strip_prefix("Prize: X=")
                .unwrap()
                .split_once(", Y=")
                .unwrap();
            let (prize_x, prize_y) = (
                prize_x.parse::<usize>().unwrap(),
                prize_y.parse::<usize>().unwrap(),
            );

            let Some(tokens) = (1..100)
                .filter_map(|a_presses| {
                    if a_presses * a_x > prize_x || (prize_x - (a_presses * a_x)) % b_x != 0 {
                        return None;
                    }
                    let b_presses = (prize_x - (a_presses * a_x)) / b_x;
                    if a_presses * a_y + b_presses * b_y != prize_y {
                        return None;
                    }
                    Some(3 * a_presses + b_presses)
                })
                .min()
            else {
                return 0;
            };

            for pow_10 in 1..=13 {
                let target = 10 * pow_10;
                let prev_target = 10 * (pow_10 - 1);
                if let Some(new_tokens) = (prev_target / a_x..target / a_x)
                    .filter_map(|a_presses| {
                        if (target - (a_presses * a_x)) % b_x != 0 {
                            return None;
                        }
                        let b_presses = (target - (a_presses * a_x)) / b_x;
                        if a_presses * a_y + b_presses * b_y != target {
                            return None;
                        }
                        Some(3 * a_presses + b_presses)
                    })
                    .min()
                {
                    return ((10000000000000 - 10 ^ pow_10) * new_tokens) + tokens;
                } else {
                    continue;
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case2() {
        let input = r"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450";
        assert_eq!(solve(input), 0);
    }
}

util::read_main!();
