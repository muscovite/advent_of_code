trait Repeated {
    fn is_repeated(&self) -> bool;
}
impl Repeated for usize {
    fn is_repeated(&self) -> bool {
        // Ex: 38593859, closet power of 10 = 7, so pow = 4
        let pow = self.ilog10().div_ceil(2);
        // Ex: 3859
        let pattern = self / (10_usize.pow(pow));
        // Ex. 38593859 - 3859 = 3859 * 10^4
        self - pattern == pattern * 10_usize.pow(pow)
    }
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let mut start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            let mut invalid_sum = 0;

            while start < end {
                let curr_pow = start.ilog10();
                let next = end.min(10_usize.pow(curr_pow + 1) - 1);
                if curr_pow % 2 == 1 {
                    let candidate_range = start.max(10_usize.pow(curr_pow))..=next;
                    invalid_sum += candidate_range.filter(|n| n.is_repeated()).sum::<usize>();
                }
                start = next + 1;
            }

            invalid_sum
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), 1227775554);
    }
}

util::read_main!();
