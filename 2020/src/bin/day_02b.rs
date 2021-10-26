use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<rule>[a-z]): (?P<password>[a-z]+)").unwrap();
}

fn solve(input: &str) -> usize {
    fn is_valid(line: &str) -> bool {
        let captures = RE.captures(line).unwrap();
        let min: usize = captures.name("min").unwrap().as_str().parse().unwrap();
        let max: usize = captures.name("max").unwrap().as_str().parse().unwrap();
        let rule = captures
            .name("rule")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password: Vec<char> = captures
            .name("password")
            .unwrap()
            .as_str()
            .chars()
            .collect();

        if password[min - 1] == rule && password[max - 1] == rule {
            false
        } else if password[min - 1] == rule || password[max - 1] == rule {
            true
        } else {
            false
        }
    }

    input.trim().lines().filter(|line| is_valid(line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case2() {
        let input = r"1-3 z: zbcde";
        assert_eq!(solve(input), 1);
    }
}

util::read_main!();
