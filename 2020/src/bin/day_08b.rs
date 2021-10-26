use std::collections::HashSet;

fn execute(instructions: &Vec<(&str, i32)>) -> (bool, i32) {
    let mut accumulator: i32 = 0;
    let mut executed: HashSet<usize> = HashSet::new();
    let end_idx = instructions.len();

    let mut idx = 0;
    loop {
        if idx == end_idx {
            return (true, accumulator);
        }

        if executed.contains(&idx) {
            return (false, accumulator);
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

fn solve(input: &str) -> i32 {
    let mut instructions: Vec<(&str, i32)> = input
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

    for idx in 0..instructions.len() {
        let (command, num) = instructions[idx];
        let new_command = match command {
            "nop" => "jmp",
            "jmp" => "nop",
            _ => command,
        };
        instructions[idx] = (new_command, num);
        let (terminated, accumulator) = execute(&instructions);
        if terminated {
            return accumulator;
        }
        instructions[idx] = (command, num);
    }

    panic!("shouldn't reach here")
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
        assert_eq!(solve(input), 8);
    }
}

util::read_main!();
