use std::collections::HashSet;

fn solve(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter_map(|l| {
            let (target, rest) = l.split_once(":").unwrap();
            let target = target.parse::<usize>().unwrap();
            let nums: Vec<usize> = rest.trim().split(" ").map(|n| n.parse().unwrap()).collect();

            let mut possibilties: HashSet<usize> = HashSet::new();
            possibilties.insert(nums[0]);
            for num in &nums[1..] {
                possibilties = possibilties
                    .into_iter()
                    .flat_map(|subtotal| {
                        [
                            subtotal + num,
                            subtotal * num,
                            format!["{subtotal}{num}"].parse::<usize>().unwrap(),
                        ]
                    })
                    .collect();
            }

            if possibilties.contains(&target) {
                return Some(target);
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve(input), 3749);
    }
}

util::read_main!();
