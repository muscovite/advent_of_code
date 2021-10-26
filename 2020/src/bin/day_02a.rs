use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<rule>[a-z]): (?P<password>[a-z]+)").unwrap();
}

fn solve(input: &str) -> usize {
    fn is_valid(line: &str) -> bool {
        let captures = RE.captures(line).unwrap();
        let min = captures.name("min").unwrap().as_str().parse().unwrap();
        let max = captures.name("max").unwrap().as_str().parse().unwrap();
        let rule = captures
            .name("rule")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let count = captures
            .name("password")
            .unwrap()
            .as_str()
            .chars()
            .filter(|&c| c == rule)
            .count();

        if count >= min && count <= max {
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
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn case2() {
        let input = r"1-3 z: abcde";
        assert_eq!(solve(input), 0);
    }
}

util::read_main!();
