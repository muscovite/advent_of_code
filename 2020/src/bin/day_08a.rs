use std::collections::HashSet;

fn solve(input: &str) -> i32 {
    let mut accumulator: i32 = 0;
    let mut executed: HashSet<usize> = HashSet::new();

    let instructions: Vec<(&str, i32)> = input
        .trim()
        .lines()
        .map(|l| {
            let mut parts = l.split(" ");
            (
                parts.next().unwrap(),
                parts
                    .next()
                    .unwrap()
                    .trim_start_matches("+")
                    .parse()
                    .unwrap(),
            )
        })
        .collect();

    let mut idx = 0;
    loop {
        if executed.contains(&idx) {
            return accumulator;
        }
        executed.insert(idx);

        let (command, num) = instructions[idx];

        match command {
            "acc" => {
                accumulator += num;
                idx += 1;
            }
            "nop" => idx += 1,
            "jmp" => idx = (idx as i32 + num) as usize,
            _ => panic!("unexpected command"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve(input), 5);
    }
}

util::read_main!();
