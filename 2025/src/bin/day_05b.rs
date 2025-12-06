use std::cmp;
fn solve(input: &str) -> usize {
    let (fresh, _) = input.trim().split_once("\n\n").unwrap();
    let mut fresh_ranges: Vec<_> = fresh
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            start..=end
        })
        .collect();
    fresh_ranges.sort_by_key(|r| *r.start());

    let mut curr_range = fresh_ranges[0].clone();
    let mut sum = 0;
    for range in &fresh_ranges[1..] {
        if range.start() > curr_range.end() {
            // no overlap
            sum += curr_range.count();
            curr_range = range.clone();
            continue;
        }
        let new_end = cmp::max(curr_range.end(), range.end());
        curr_range = *curr_range.start()..=*new_end;
    }
    sum += curr_range.count();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_test_case() {
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
        assert_eq!(solve(input), 14);
    }

    #[test]
    fn fully_included() {
        let input = r"6-7
1-10

1
5
8
11
17
32";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn many_partial_overlaps() {
        let input = r"3-4
5-6
3-5
6-8
7-8

1
5
8
11
17
32";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn has_exact_overlaps() {
        let input = r"6-7
3-5
3-5
3-5

1
5
8
11
17
32";
        assert_eq!(solve(input), 5);
    }

    #[test]
    fn no_overlaps() {
        let input = r"3-5
10-14
16-20

1
5
8
11
17
32";
        assert_eq!(solve(input), 13);
    }
}

util::read_main!();
