use itertools::Itertools;

fn find_max_idx(arr: &[u8]) -> usize {
    for i in (1..=9).rev() {
        if let Some((idx, _)) = arr
            .iter()
            .find_position(|&b| *b == char::from_digit(i, 10).unwrap() as u8)
        {
            return idx;
        }
    }
    unreachable!()
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            let l = l.as_bytes();
            let first_idx = find_max_idx(&l[..l.len() - 1]);
            let l2 = &l[first_idx + 1..];
            let second_idx = find_max_idx(l2);
            ((l[first_idx] - b'0') * 10 + (l2[second_idx] - b'0')) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(input), 357);
    }
}

util::read_main!();
