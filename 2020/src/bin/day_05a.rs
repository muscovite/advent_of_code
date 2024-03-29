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
    input.trim().lines().map(seat_id).max().unwrap()
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

util::read_main!();
