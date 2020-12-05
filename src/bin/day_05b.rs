use std::collections::HashSet;

fn seat_id(input: &str) -> usize {
    let mut row = 0;
    let mut col = 0;

    let mut row_bit = 1;
    let mut col_bit = 1;
    for c in input.chars().rev() {
        match c {
            'B' => {
                row |= row_bit;
                row_bit <<= 1;
            }
            'F' => row_bit <<= 1,
            'R' => {
                col |= col_bit;
                col_bit <<= 1;
            }
            'L' => col_bit <<= 1,
            _ => panic!("whaaaat"),
        };
    }
    (row * 8) + col
}

fn solve(input: &str) -> usize {
    let all_ids: HashSet<_> = (0..1024).collect();
    let occupied = input.trim().lines().map(|line| seat_id(line)).collect();
    let candidates = all_ids.difference(&occupied);
    for &candidate in candidates {
        if occupied.contains(&(candidate - 1)) && occupied.contains(&(candidate + 1)) {
            return candidate;
        }
    }
    panic!("this was unexpected");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_score() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
    #[test]
    fn highest() {
        let input = r"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        assert_eq!(solve(input), 820);
    }
}

advent_2020::read_main!();
