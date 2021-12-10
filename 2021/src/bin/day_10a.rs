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

fn find_first_invalid(l: &str) -> Option<char> {
    let mut stack = vec![];
    for c in l.trim().chars() {
        if is_open(c) {
            stack.push(c)
        } else if is_close(c) {
            if !is_paired(stack.pop().unwrap(), c) {
                return Some(c);
            }
        }
    }

    None
}

fn solve(input: &str) -> u32 {
    let input = input.trim().lines();

    input
        .map(|l| match find_first_invalid(l) {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            None => 0,
            _ => unreachable!("why are you here"),
        })
        .sum()
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
        assert_eq!(solve(input), 26397);
    }
}

util::read_main!();
