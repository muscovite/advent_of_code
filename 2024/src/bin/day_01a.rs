fn solve(input: &str) -> usize {
    let mut vec1 = vec![];
    let mut vec2 = vec![];

    input
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .for_each(|mut splits| {
            vec1.push(splits.next().unwrap().parse::<usize>().unwrap());
            vec2.push(splits.next().unwrap().parse::<usize>().unwrap());
        });

    vec1.sort();
    vec2.sort();

    vec1.iter()
        .zip(vec2.iter())
        .map(|(e1, &e2)| e1.abs_diff(e2))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(solve(input), 11);
    }
}

util::read_main!();
