use std::cmp::Ordering;

fn is_safe(v: &[usize]) -> bool {
    let mut order: Option<Ordering> = None;
    v.windows(2).all(|arr| {
        let a = arr[0];
        let b = arr[1];
        match (a.cmp(&b), order) {
            (Ordering::Equal, _) => return false,
            (ord, None) => order = Some(ord),
            (ord, Some(expected)) => {
                if ord != expected {
                    return false;
                }
            }
        }
        a.abs_diff(b) <= 3
    })
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .filter(|v: &Vec<usize>| {
            // No need to check the untruncated version, because if the original
            // is valid, chopping off either end still keeps it valid
            (0..v.len()).any(|i| {
                let mut v2 = v.clone();
                v2.remove(i);
                is_safe(&v2)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(solve(input), 4);
    }
}

util::read_main!();
