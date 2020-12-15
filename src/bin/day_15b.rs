use std::collections::HashMap;

fn solve(input: &str) -> usize {
    let mut seen_last: HashMap<usize, (usize, usize)> = HashMap::new(); // (num, turn)

    let seeds: Vec<usize> = input
        .trim()
        .split(',')
        .map(|tok| tok.parse().unwrap())
        .collect();

    let mut last = 0;
    // seed seen_last with starting numbers
    for i in 0..seeds.len() {
        last = seeds[i];
        seen_last.insert(seeds[i], (i + 1, 0));
    }

    let mut turn = seeds.len();
    while turn < 30000000 {
        turn += 1;

        match seen_last.get(&last).unwrap() {
            &(_, 0) => {
                // was last seen for the first time, speak 0
                // update count for 0
                match seen_last.get(&0) {
                    Some(&(recent, _)) => seen_last.insert(0, (turn, recent)),
                    None => seen_last.insert(0, (turn, 0)),
                };
                last = 0;
            }
            &(recent, prev) => {
                // seen before; speak recent - prev
                let spoken = recent - prev;
                match seen_last.get(&spoken) {
                    Some(&(r, _)) => seen_last.insert(spoken, (turn, r)),
                    None => seen_last.insert(spoken, (turn, 0)),
                };
                last = spoken;
            }
        };
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"0,3,6";
        assert_eq!(solve(input), 175594);
    }

    #[test]
    fn case2() {
        let input = r"1,3,2";
        assert_eq!(solve(input), 2578);
    }

    #[test]
    fn case3() {
        let input = r"2,1,3";
        assert_eq!(solve(input), 3544142);
    }

    #[test]
    fn case4() {
        let input = r"1,2,3";
        assert_eq!(solve(input), 261214);
    }

    #[test]
    fn case5() {
        let input = r"2,3,1";
        assert_eq!(solve(input), 6895259);
    }

    #[test]
    fn case6() {
        let input = r"3,2,1";
        assert_eq!(solve(input), 18);
    }

    #[test]
    fn case7() {
        let input = r"3,1,2";
        assert_eq!(solve(input), 362);
    }
}

advent_2020::read_main!();
