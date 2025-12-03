fn is_all_substring(str: &str, chunk_len: usize) -> bool {
    let candidate = &str[0..chunk_len];
    let mut idx = chunk_len;
    while idx <= str.len() - chunk_len {
        if &str[idx..idx + chunk_len] != candidate {
            return false;
        }
        idx += chunk_len;
    }
    true
}

trait Repeated {
    fn is_repeated(&self) -> bool;
}
impl Repeated for usize {
    fn is_repeated(&self) -> bool {
        let str_rep = self.to_string();
        let len = str_rep.len();

        if len == 1 {
            // must repeat at least once
            return false;
        }

        // odd numbers
        if len % 2 == 1 {
            // perfect square length: only 1 and square root are possible
            if len.isqrt().pow(2) == len {
                return is_all_substring(&str_rep, 1) || is_all_substring(&str_rep, len.isqrt());
            }
            // else, only 1 is possible
            return is_all_substring(&str_rep, 1);
        }

        // even numbers: any divisor up to len / 2 is possible
        (1..=len / 2)
            .filter(|n| len % n == 0)
            .any(|chunk_len| is_all_substring(&str_rep, chunk_len))
    }
}

fn solve(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();

            (start..=end).filter(|n| n.is_repeated()).sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(input), 4174379265);
    }
}

util::read_main!();
