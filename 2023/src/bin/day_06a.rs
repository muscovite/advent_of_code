fn solve(input: &str) -> usize {
    let (times, distances) = input.split_once("\n").unwrap();
    let times = times
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap());
    let distances = distances
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<u32>().unwrap());

    let ways_to_beat = times.zip(distances).map(|(time, distance)| {
        (1..time)
            .skip_while(|t| (time - t) * t <= distance)
            .take_while(|t| (time - t) * t > distance)
            .count()
    });

    ways_to_beat.product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(solve(input), 288);
    }
}

util::read_main!();
