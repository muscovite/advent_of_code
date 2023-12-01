const VALID_NUMBERS: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn get_num(num_str: &str) -> u32 {
    match num_str {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("not a number!"),
    }
}

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|l| {
            // Get all matches within input string
            let indices: Vec<_> = VALID_NUMBERS
                .iter()
                .filter(|&&num_str| l.contains(num_str))
                .flat_map(|num_str| l.match_indices(num_str).collect::<Vec<_>>())
                .collect();
            let min = indices.iter().min_by_key(|(i, _)| i).unwrap();
            let max = indices.iter().max_by_key(|(i, _)| i).unwrap();
            (10 * get_num(min.1)) + get_num(max.1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(solve(input), 281);
    }
}

util::read_main!();
