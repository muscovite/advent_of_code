use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NOTE_RE: Regex = Regex::new(
        r"^(?P<field>[ \w]+): (?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)$"
    )
    .unwrap();
}

fn solve(input: &str) -> usize {
    let mut fields = [""; 1000];
    let mut input = input.trim().split("\n\n");

    // notes
    let notes = input.next().unwrap();
    for note in notes.lines() {
        let caps = NOTE_RE.captures(note).unwrap();
        let field_name = caps.name("field").unwrap().as_str();

        for (l, u) in [("lower1", "upper1"), ("lower2", "upper2")].iter() {
            let lower: usize = caps.name(l).unwrap().as_str().parse().unwrap();
            let upper: usize = caps.name(u).unwrap().as_str().parse().unwrap();
            for i in lower..=upper {
                fields[i] = field_name;
            }
        }
    }

    // your ticket - ignore for now
    input.next();

    // other tickets
    let mut others = input.next().unwrap().lines();
    // skip "nearby tickets:"
    others.next();
    others
        .map(|o| {
            o.split(',')
                .filter_map(|num| {
                    let n: usize = num.parse().unwrap();
                    if fields[n] != "" {
                        return None;
                    }
                    Some(n)
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(solve(input), 71);
    }
}

advent_2020::read_main!();
