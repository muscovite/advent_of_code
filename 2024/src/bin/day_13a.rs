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

            if let Some(tokens) = (1..=100)
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
            {
                return tokens;
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
        assert_eq!(solve(input), 280);
    }

    #[test]
    fn case1() {
        let input = r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case2() {
        let input = r"
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450";
        assert_eq!(solve(input), 200);
    }

    #[test]
    fn case3() {
        let input = r"
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case4() {
        let input = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!(solve(input), 480);
    }
}

util::read_main!();
