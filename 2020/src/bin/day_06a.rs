fn get_count(input: &str) -> usize {
    let mut answers: [bool; 26] = [false; 26];
    for c in input.chars().filter(|&c| c != '\n') {
        answers[c as usize - 'a' as usize] = true;
    }
    answers.iter().filter(|&&e| e).count()
}

fn solve(input: &str) -> usize {
    input.trim().split("\n\n").map(get_count).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve(input), 11);
    }
}

advent_2020::read_main!();
