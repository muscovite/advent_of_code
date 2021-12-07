use std::cmp;

fn solve(input: &str) -> i32 {
    let mut positions: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    positions.sort_unstable();

    let average =
        ((positions.iter().sum::<i32>() as f32) / (positions.len() as f32)).round() as i32;

    let average2 = positions.iter().sum::<i32>() / positions.len() as i32;

    // lol
    cmp::min(
        positions
            .iter()
            .map(|pos| (1..=(pos - average).abs()).sum::<i32>())
            .sum(),
        positions
            .iter()
            .map(|pos| (1..=(pos - average2).abs()).sum::<i32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"16,1,2,0,4,2,7,1,2,14";
        assert_eq!(solve(input), 168);
    }

    #[test]
    fn case2() {
        let input = r"1,2,4";
        assert_eq!(solve(input), 4);
    }
}

util::read_main!();
