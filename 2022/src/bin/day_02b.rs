fn solve(input: &str) -> usize {
    // 1 for Rock, 2 for Paper, and 3 for Scissors
    // 0 if you lost, 3 if the round was a draw, and 6 if you won
    // A for Rock, B for Paper, and C for Scissors
    // X lose, Y draw, Z win
    input
        .trim()
        .lines()
        .map(|l| match l.trim() {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            _ => panic!("unhandled combination"),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"A Y
B X
C Z";
        assert_eq!(solve(input), 12);
    }
}

util::read_main!();
