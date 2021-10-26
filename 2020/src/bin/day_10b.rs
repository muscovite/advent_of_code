fn solve(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    adapters.sort();
    adapters.insert(0, 0);

    // Store # of paths that you can take to reach this adapter
    let mut paths = vec![0; adapters.len()];
    // Seed 0 with 1
    paths[0] = 1;

    // We know there's a max allowed delta of 3 between adapters, so
    // there's a max number of adapters to look ahead of 3
    // Basically, the number of ways to get to a given adapter is the
    // cumulative sum of ways to get to all the adapters before it
    for i in 0..adapters.len() - 1 {
        for j in 1..=3 {
            // Continue if at end of array or delta too large
            if i + j > adapters.len() - 1 || adapters[i + j] - adapters[i] > 3 {
                break;
            }
            paths[i + j] += paths[i];
        }
    }
    paths[paths.len() - 1]
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
        assert_eq!(solve(input), 8);
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
        assert_eq!(solve(input), 19208);
    }

    #[test]
    fn case3() {
        let input = r"1
4
5
7
8";
        assert_eq!(solve(input), 3);
    }
}

util::read_main!();
