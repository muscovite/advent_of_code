fn hash(s: &str) -> usize {
    s.bytes().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve(input: &str) -> usize {
    input.split(",").map(|s| hash(s)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 1320);
    }

    #[test]
    fn case2() {
        let input = r"HASH";
        assert_eq!(solve(input), 52);
    }
}

util::read_main!();
