fn brute_force(input: &str) -> u64 {
    let mut input_iter = input.trim().lines();
    // don't care about first line anymore
    input_iter.next();

    let buses: Vec<_> = input_iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, bus)| {
            if bus == "x" {
                return None;
            }
            Some((idx as u64, bus.parse::<u64>().unwrap()))
        })
        .collect();
    let (_, first_bus) = buses[0];
    let (last_idx, last_bus) = buses[buses.len() - 1];
    let mut time = 0;
    loop {
        // find next candidate time
        while (time + last_idx) % last_bus != 0 {
            time += first_bus;
        }

        // check everything in between
        let mut passed = true;
        for (b_idx, b) in buses[1..buses.len() - 1].iter() {
            if (time + b_idx) % b != 0 {
                passed = false;
                time += first_bus;
                break;
            }
        }

        if passed {
            return time;
        }
    }
}

#[derive(Debug)]
struct Bus {
    remainder: u64,
    id: u64,
}

fn faster(input: &str) -> u64 {
    // solve for each pair
    let mut input_iter = input.trim().lines();
    // don't care about first line anymore
    input_iter.next();

    let mut buses = input_iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, bus)| {
            if bus == "x" {
                return None;
            }
            let id = bus.parse::<u64>().unwrap();
            Some(Bus {
                // mod to avoid underflow
                remainder: id - (idx as u64 % id) as u64,
                id: id,
            })
        });
    let first = buses.next().unwrap();
    buses
        .fold(first, |prev, next| {
            // Calculate remainders and ids (steps) for each pair
            // Eg:
            // n % 3 (first.id) = 0
            // n % 5 (next.id) = 4 (next.id - next.pos = next.rem)
            // Solve for n using second equation, increasing by first.id
            // each loop
            let mut n = prev.remainder;
            while n % next.id != next.remainder {
                n += prev.id;
            }
            Bus {
                remainder: n,
                id: prev.id * next.id,
            }
        })
        .remainder
}

fn solve(input: &str) -> u64 {
    // brute_force(input)
    faster(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_case() {
        let input = r"939
3,5,7";
        assert_eq!(solve(input), 54);
    }

    #[test]
    fn case1() {
        let input = r"939
7,13,x,x,59,x,31,19";
        assert_eq!(solve(input), 1068781);
    }

    #[test]
    fn case2() {
        let input = r"939
67,7,59,61";
        assert_eq!(solve(input), 754018);
    }

    #[test]
    fn case3() {
        let input = r"939
67,x,7,59,61";
        assert_eq!(solve(input), 779210);
    }

    #[test]
    fn case4() {
        let input = r"939
67,7,x,59,61";
        assert_eq!(solve(input), 1261476);
    }

    #[test]
    fn case5() {
        let input = r"939
1789,37,47,1889";
        assert_eq!(solve(input), 1202161486);
    }

    #[test]
    fn case6() {
        let input = r"939
17,x,13,19";
        assert_eq!(solve(input), 3417);
    }
}

util::read_main!();
