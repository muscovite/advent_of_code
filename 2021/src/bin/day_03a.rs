fn solve(input: &str) -> u32 {
    let diagnostics: Vec<&str> = input.trim().lines().collect();
    let line_len = diagnostics[0].len();
    let num_lines = diagnostics.len() as u32;

    let result = diagnostics.iter().fold(vec![0; line_len], |mut acc, line| {
        line.chars()
            .enumerate()
            .for_each(|(i, char)| acc[i] += char.to_digit(2).unwrap());
        acc
    });

    // Puzzle statement implies there are no ties
    let (gamma_str, eps_str): (String, String) = result
        .iter()
        .map(|val| {
            if 2 * val > num_lines {
                ('0', '1')
            } else {
                ('1', '0')
            }
        })
        .unzip();

    let gamma = u32::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let eps = u32::from_str_radix(eps_str.as_str(), 2).unwrap();

    gamma * eps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        assert_eq!(solve(input), 198);
    }
}

util::read_main!();
