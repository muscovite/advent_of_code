use std::cmp::Ordering;

#[derive(Clone, Copy)]
enum Dir {
    Asc,
    Desc,
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
            let mut dir: Option<Dir> = None;
            for arr in v.windows(2) {
                let a = arr[0];
                let b = arr[1];
                match (a.cmp(&b), dir) {
                    (Ordering::Equal, _) => return false,
                    (Ordering::Less, None) => dir = Some(Dir::Asc),
                    (Ordering::Greater, None) => dir = Some(Dir::Desc),
                    (Ordering::Less, Some(Dir::Desc)) => return false,
                    (Ordering::Greater, Some(Dir::Asc)) => return false,
                    _ => (),
                }
                let diff = a.abs_diff(b);
                if diff < 1 || diff > 3 {
                    return false;
                }
            }
            true
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
        assert_eq!(solve(input), 2);
    }
}

util::read_main!();
