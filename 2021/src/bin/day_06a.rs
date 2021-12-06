fn solve(input: &str) -> u32 {
    let mut fishes: [u32; 9] = [0; 9];
    input.trim().split(',').for_each(|s| {
        let idx = s.parse::<usize>().unwrap();
        fishes[idx] += 1;
    });

    (0..80).for_each(|_| {
        let tmp = fishes[0];

        (1..=8).for_each(|i| {
            fishes[i - 1] = fishes[i];
        });

        // old fish reset
        fishes[6] += tmp;
        // new fish spawn
        fishes[8] = tmp;
    });
    fishes.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"3,4,3,1,2";
        assert_eq!(solve(input), 5934);
    }
}

util::read_main!();
