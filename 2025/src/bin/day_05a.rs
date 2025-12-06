fn solve(input: &str) -> usize {
    let (fresh, ingredients) = input.trim().split_once("\n\n").unwrap();
    let fresh_ranges: Vec<_> = fresh
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            start..=end
        })
        .collect();

    ingredients
        .lines()
        .filter(|l| {
            let num = l.parse::<usize>().unwrap();
            fresh_ranges.iter().any(|range| range.contains(&num))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(input), 3);
    }
}

util::read_main!();
