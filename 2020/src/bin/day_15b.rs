use std::collections::hash_map::Entry;
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
    while turn < 30_000_000_usize {
        turn += 1;

        match seen_last.get(&last).unwrap() {
            &(_, 0) => {
                // was last seen for the first time, speak 0
                // update count for 0
                match seen_last.entry(0) {
                    Entry::Occupied(mut o) => {
                        let &(recent, _) = o.get();
                        o.insert((turn, recent));
                    }
                    Entry::Vacant(v) => {
                        v.insert((turn, 0));
                    }
                };
                last = 0;
            }
            &(recent, prev) => {
                // seen before; speak recent - prev
                let spoken = recent - prev;
                match seen_last.entry(spoken) {
                    Entry::Occupied(mut o) => {
                        let &(r, _) = o.get();
                        o.insert((turn, r));
                    }
                    Entry::Vacant(v) => {
                        v.insert((turn, 0));
                    }
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

util::read_main!();
