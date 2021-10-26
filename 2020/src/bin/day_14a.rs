use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<value>\d+)$").unwrap();
}

#[derive(Debug)]
struct Mask {
    and: u64,
    or: u64,
}

enum Command {
    ApplyMask(Mask),    // mask for AND-ing, mask for OR-ing
    WriteMem(u64, u64), // index, value
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(|l| {
            if l.starts_with("mask") {
                let mask = l.split(" ").nth(2).unwrap().chars();
                let mut and_mask = 0;
                let mut or_mask = 0;
                let mut one = 1;
                for c in mask.rev() {
                    match c {
                        '0' => {}
                        '1' => {
                            or_mask |= one;
                        }
                        'X' => {
                            and_mask |= one;
                        }
                        _ => unreachable!(),
                    }
                    one <<= 1;
                }
                return Command::ApplyMask(Mask {
                    and: and_mask,
                    or: or_mask,
                });
            } else {
                let captures = MEM_RE.captures(l).unwrap();
                let idx = captures.name("index").unwrap().as_str().parse().unwrap();
                let val = captures.name("value").unwrap().as_str().parse().unwrap();
                return Command::WriteMem(idx, val);
            }
        })
        .collect()
}

fn solve(input: &str) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let commands = parse_commands(input);
    let mut curr_mask = Mask { and: 0, or: 0 };

    for command in commands {
        match command {
            Command::ApplyMask(mask) => curr_mask = mask,
            Command::WriteMem(idx, val) => {
                memory.insert(idx, (val & curr_mask.and) | curr_mask.or);
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(solve(input), 165);
    }
}

util::read_main!();
