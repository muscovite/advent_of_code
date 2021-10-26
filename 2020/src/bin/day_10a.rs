fn solve(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();

    let mut ones = 0;
    let mut threes = 1; // one for your adapter
    for i in 0..=adapters.len() - 2 {
        match adapters[i + 1] - adapters[i] {
            3 => threes += 1,
            1 => ones += 1,
            _ => panic!("unexpected"),
        }
    }

    if adapters[0] == 1 {
        ones += 1;
    } else {
        threes += 1;
    }

    ones * threes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve(input), 35);
    }

    #[test]
    fn case2() {
        let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(solve(input), 220);
    }
}

util::read_main!();
