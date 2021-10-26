use std::iter;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Operator(char),
    Number(u64),
    OpenParen,
}

fn operate(operator: Token, op1: Token, op2: Token) -> Token {
    match (operator, op1, op2) {
        (Token::Operator('+'), Token::Number(n1), Token::Number(n2)) => return Token::Number(n1 + n2),
        (Token::Operator('*'), Token::Number(n1), Token::Number(n2)) => return Token::Number(n1 * n2),
        _ => unreachable!(),

    }
}

fn solve_eq(input: &str) -> u64 {
    let mut stack = Vec::new();

    // outer loop
    for token in input.split_whitespace() {
        match token.chars().nth(0).unwrap() {
            '+' => stack.push(Token::Operator('+')),
            '*' => stack.push(Token::Operator('*')),
            '0'..='9' => {
                // add number
                let num_digit = token.chars().take_while(|&c| c != ')').count();
                let num = Token::Number(token[..num_digit].parse().unwrap());

                if stack.len() == 0 || !matches!(stack[stack.len()-1], Token::Operator(_)) {
                    stack.push(num)
                } else {
                    let operator = stack.pop().unwrap();
                    let op1 = stack.pop().unwrap();
                    stack.push(operate(operator, op1,num));
                }
                // do close paren logic
                for _ in num_digit..token.len() {
                    let op2 = stack.pop().unwrap();
                    // get rid of open paren
                    stack.pop();

                    match stack.pop() {
                        Some(tok @ Token::Operator(_)) => {
                            let op1 = stack.pop().unwrap();
                            stack.push(operate(tok, op1, op2));
                        }
                        Some(tok) => {
                            // put stuff back
                            stack.push(tok);
                            stack.push(op2);
                        },
                        None => {
                            // put num back
                            stack.push(op2);
                        }
                    }
                }
            },
            '(' => {
                // add parens
                let num_paren = token.chars().take_while(|&c| c == '(').count();
                stack.extend(iter::repeat(Token::OpenParen).take(num_paren));
                // add number
                stack.push(Token::Number(token[num_paren..].parse().unwrap()));
            },
            _ => unreachable!()
        }
    }
    match stack.pop().unwrap() {
        Token::Number(num) => num,
        _ => unreachable!()
    }
}


fn solve(input: &str) -> u64 {
    input.trim().lines().map(|l| solve_eq(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(solve_eq(input), 71);
    }

    #[test]
    fn case2() {
        let input = r"1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(solve_eq(input), 51);
    }

    #[test]
    fn case3() {
        let input = r"2 * 3 + (4 * 5)";
        assert_eq!(solve_eq(input), 26);
    }

    #[test]
    fn case4() {
        let input = r"5 + (8 * 3 + 9 + 3 * 4 * 3)";
        assert_eq!(solve_eq(input), 437);
    }

    #[test]
    fn case5() {
        let input = r"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(solve_eq(input), 12240);
    }

    #[test]
    fn case6() {
        let input = r"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        assert_eq!(solve_eq(input), 13632);
    }

    #[test]
    fn case7() {
        let input = r"6 * 7 * 8 * 9 * ((8 * 3 * 9) * 7 + 2 + 4 * 8 + 2) + 5";
        assert_eq!(solve_eq(input), 36729509);
    }
}

util::read_main!();
