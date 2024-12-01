use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let mut vec1 = vec![];
    let mut freq_map: HashMap<usize, usize> = HashMap::new();

    input
        .lines()
        .map(|l| l.split_ascii_whitespace())
        .for_each(|mut splits| {
            vec1.push(splits.next().unwrap().parse::<usize>().unwrap());

            let key = splits.next().unwrap().parse::<usize>().unwrap();
            *freq_map.entry(key).or_insert(0) += 1;
        });

    vec1.iter()
        .map(|val| freq_map.get(val).or(Some(&0)).unwrap() * val)
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
        assert_eq!(solve(input), 31);
    }
}

util::read_main!();
