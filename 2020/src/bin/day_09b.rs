fn find_invalid(input: &Vec<usize>, window_len: usize) -> usize {
    let (mut left, mut right) = (0, window_len - 1);
    for idx in window_len..input.len() {
        let target = input[idx];
        let mut found = false;
        for potential_idx in left..=right {
            let curr_val = input[potential_idx];
            if curr_val >= target {
                continue;
            }

            let search_val = target - curr_val;
            let match_idx = input[left..=right].iter().position(|&n| n == search_val);
            if match_idx.is_some() && match_idx.unwrap() != potential_idx {
                found = true;
                break;
            }
        }

        if !found {
            return target;
        }

        left += 1;
        right += 1;
    }
    5
}

fn find_weakness(input: &Vec<usize>, target: usize) -> usize {
    let (mut left, mut right) = (0, 1);
    loop {
        let sum: usize = input[left..=right].iter().sum();
        if sum == target {
            let mut sorted = input[left..=right].iter().collect::<Vec<_>>();
            sorted.sort();
            return sorted[0] + sorted[sorted.len() - 1];
        }
        // too small
        if sum < target {
            right += 1;
            continue;
        }
        // too big
        right -= 1;
        left += 1;
    }
}

fn solve(input: &str) -> usize {
    let nums = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    let invalid = find_invalid(&nums, 25);
    find_weakness(&nums, invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let nums = input.trim().lines().map(|l| l.parse().unwrap()).collect();
        assert_eq!(find_invalid(&nums, 5), 127);
        assert_eq!(find_weakness(&nums, 127), 62);
    }
}

advent_2020::read_main!();
