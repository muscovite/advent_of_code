use std::char;
use std::collections::HashSet;

fn is_valid(input: &str) -> bool {
    true
}

fn make_rules(input: &str) -> Vec<String> {
    let mut terminal_states = vec![false; input.lines().count()];
    let mut terminal_rules = HashSet::new();
    let mut rules: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            // skip pass index
            match &l[2..] {
                r#" "a""# => {
                    terminal_rules.insert(i);
                    " a ".to_string()
                }
                r#" "b""# => {
                    terminal_rules.insert(i);
                    " b ".to_string()
                }
                _ => l[2..].to_string() + " ",
            }
        })
        .collect();

    dbg!(&rules);

    // first pass: expand all rules
    // XXX: doesn't preserve subrules yet
    loop {
        let curr = terminal_rules.iter().next();
        match curr {
            Some(&idx) => {
                terminal_rules.remove(&idx);
                let search = &format!("{}", idx);
                for j in 0..rules.len() {
                    if j == idx {
                        terminal_states[idx] = true;
                        continue;
                    }
                    rules[j] = rules[j].replace(search, &format!("({})", &rules[idx].trim()));

                    // if this is now a terminal rule, add to terminals list
                    dbg!(&rules[j], idx);
                    if !rules[j].chars().any(char::is_numeric) && !terminal_states[j] {
                        terminal_rules.insert(j);
                    }
                }
            }
            None => break,
        }
        dbg!(&terminal_rules);
        dbg!(&terminal_states);
        dbg!(&rules);
    }

    // second pass: clean up white space, split on pipes?
    rules
}

fn solve(input: &str) -> usize {
    let mut input = input.trim().split("\n\n");

    // construct rules
    // NB: only ever two subrules
    let rules = make_rules(input.next().unwrap());

    // check messages
    input
        .next()
        .unwrap()
        .lines()
        .filter(|l| is_valid(l))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        assert_eq!(solve(input), 2);
    }
}

util::read_main!();
