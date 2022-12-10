fn draw(register: i32, cycle: i32) -> bool {
    (register - 1..=register + 1).contains(&((cycle - 1) % 40))
}

fn solve(input: &str) -> usize {
    let mut cycle: usize = 1;
    let mut register = 1;
    let mut screen = ['.'; 240];

    for l in input.trim().lines() {
        match l.split_once(" ") {
            Some((_, diff)) => {
                if draw(register, cycle as i32) {
                    screen[cycle - 1] = '#';
                }
                cycle += 1;
                if draw(register, cycle as i32) {
                    screen[cycle - 1] = '#';
                }
                cycle += 1;
                register += diff.parse::<i32>().unwrap();
            }
            None => {
                if draw(register, cycle as i32) {
                    screen[cycle - 1] = '#';
                }
                cycle += 1;
            }
        }
    }
    for chunk in screen.chunks(40) {
        let line: String = chunk.iter().collect();
        println!("{line:?}");
    }
    0
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
        solve(input);
    }
}

util::read_main!();
