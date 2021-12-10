// ): 3 points.
// ]: 57 points.
// }: 1197 points.
// >: 25137 points.

fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn is_close(c: char) -> bool {
    c == ')' || c == ']' || c == '}' || c == '>'
}

fn is_paired(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => panic!("invalid char"),
    }
}

fn get_score(l: &str) -> Option<u64> {
    let mut stack = vec![];
    for c in l.trim().chars() {
        if is_open(c) {
            stack.push(c)
        } else if is_close(c) {
            if !is_paired(stack.pop().unwrap(), c) {
                return None;
            }
        }
    }

    let score = stack.into_iter().rev().fold(0, |score, c| {
        let sub_score = match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("invalid char"),
        };

        (score * 5) + sub_score
    });
    Some(score)
}

fn solve(input: &str) -> u64 {
    let mut scores: Vec<u64> = input.trim().lines().filter_map(get_score).collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(solve(input), 288957);
    }
}

util::read_main!();
