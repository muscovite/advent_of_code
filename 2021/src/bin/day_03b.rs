fn bit_match(diagnostics: &Vec<Vec<char>>, is_oxygen: bool, idx: usize) -> char {
    let num_lines = diagnostics.len() as u32;
    let count = diagnostics
        .iter()
        .map(|line| line[idx].to_digit(2).unwrap())
        .sum::<u32>();

    let val = 2 * (num_lines - count);
    if val == num_lines {
        if is_oxygen {
            '1'
        } else {
            '0'
        }
    } else if val > num_lines {
        if is_oxygen {
            '0'
        } else {
            '1'
        }
    } else {
        if is_oxygen {
            '1'
        } else {
            '0'
        }
    }
}

fn solve(input: &str) -> u32 {
    let mut oxy_diagnostics: Vec<Vec<char>> =
        input.trim().lines().map(|l| l.chars().collect()).collect();
    let mut co2_diagnostics = oxy_diagnostics.clone();
    let line_len = oxy_diagnostics[0].len();

    let mut oxygen_rating: String = "".to_string();
    for i in 0..line_len {
        let bit_match = bit_match(&oxy_diagnostics, true, i);
        oxy_diagnostics.retain(|f| f[i] == bit_match);
        if oxy_diagnostics.len() == 1 {
            oxygen_rating = oxy_diagnostics[0].iter().collect();
            break;
        }
    }

    let mut co2_rating: String = "".to_string();
    for i in 0..line_len {
        let bit_match = bit_match(&co2_diagnostics, false, i);
        co2_diagnostics.retain(|f| f[i] == bit_match);
        if co2_diagnostics.len() == 1 {
            co2_rating = co2_diagnostics[0].iter().collect();
            break;
        }
    }

    let oxygen_rating = u32::from_str_radix(oxygen_rating.as_str(), 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_rating.as_str(), 2).unwrap();

    oxygen_rating * co2_rating
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
        assert_eq!(solve(input), 230);
    }
}

util::read_main!();
