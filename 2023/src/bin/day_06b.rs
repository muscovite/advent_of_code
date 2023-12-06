fn solve(input: &str) -> usize {
    let (times, distances) = input.split_once("\n").unwrap();
    let time: u64 = times
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = distances
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    (1..time)
        .skip_while(|t| (time - t) * t <= distance)
        .take_while(|t| (time - t) * t > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve(input), 71503);
    }
}

util::read_main!();
