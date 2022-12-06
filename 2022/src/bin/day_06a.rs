fn solve(input: &str) -> usize {
    const WINDOW_LEN: usize = 4;
    let stream = input.trim().as_bytes();
    let iter = stream.windows(WINDOW_LEN);

    for (i, window) in iter.enumerate() {
        let mut window = window.to_vec();
        window.sort();

        if window
            .windows(2)
            .any(|adjacency| adjacency[0] == adjacency[1])
        {
            continue;
        }

        return i + WINDOW_LEN;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve(input), 7);
    }

    #[test]
    fn case2() {
        let input = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve(input), 5);
    }

    #[test]
    fn case3() {
        let input = r"nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn case4() {
        let input = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn case5() {
        let input = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve(input), 11);
    }
}

util::read_main!();
