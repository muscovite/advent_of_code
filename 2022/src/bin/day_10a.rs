fn emit(cycle: i32) -> bool {
    // 20th, 60th, 100th, 140th, 180th, and 220th cycles
    return cycle == 20 || cycle % 40 == 20;
}

fn solve(input: &str) -> i32 {
    let mut cycle = 1;
    let mut register = 1;
    let mut sum = 0;

    for l in input.trim().lines() {
        match l.split_once(" ") {
            Some((_, diff)) => {
                if emit(cycle) {
                    sum += cycle * register;
                }
                cycle += 1;
                if emit(cycle) {
                    sum += cycle * register;
                }
                cycle += 1;
                register += diff.parse::<i32>().unwrap();
            }
            None => {
                if emit(cycle) {
                    sum += cycle * register;
                }
                cycle += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(solve(input), 13140);
    }
}

util::read_main!();
