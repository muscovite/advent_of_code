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
            let mut sum = 0;
            let mut curr_idx = 0;
            (0..=11 as u16).enumerate().rev().for_each(|(i, pow)| {
                let l2 = &l[curr_idx..=l.len() - (i + 1)];
                let new_idx = find_max_idx(l2);
                sum += (l2[new_idx] - b'0') as usize * 10_usize.pow(pow as u32);
                curr_idx = curr_idx + new_idx + 1;
            });
            sum
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
        assert_eq!(solve(input), 3121910778619);
    }
}

util::read_main!();
