const DIAL_MAX: u16 = 100;

fn solve(input: &str) -> usize {
    let (_, zeroes_seen) = input.trim().lines().map(|l| l.split_at(1)).fold(
        (50, 0),
        |(curr_num, mut zeroes_seen), (dir, num_clicks)| {
            let num_clicks = num_clicks.parse::<u16>().unwrap();
            let diff = match dir {
                "L" => {
                    zeroes_seen += num_clicks / DIAL_MAX;
                    if num_clicks % DIAL_MAX >= curr_num && curr_num != 0 {
                        zeroes_seen += 1;
                    }
                    DIAL_MAX - (num_clicks % DIAL_MAX)
                }
                "R" => {
                    zeroes_seen += (curr_num + num_clicks) / DIAL_MAX;
                    num_clicks
                }
                _ => unreachable!(),
            };
            let curr_num = (curr_num + diff) % DIAL_MAX;
            (curr_num, zeroes_seen)
        },
    );
    zeroes_seen as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve(input), 6);
    }
}

util::read_main!();
