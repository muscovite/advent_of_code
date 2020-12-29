use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<address>\d+)\] = (?P<value>\d+)$").unwrap();
}

fn apply_mask(mask: &Vec<char>, val: &Vec<char>) -> Vec<char> {
    let mut new_mask = Vec::new();
    for (&m, &v) in mask.iter().zip(val.iter()) {
        match (m, v) {
            ('X', _) => new_mask.push('X'),
            ('1', _) => new_mask.push('1'),
            ('0', _) => new_mask.push(v),
            _ => unreachable!(),
        }
    }
    new_mask
}

// could maybe use bools instead
fn get_overlap(mask1: &Vec<char>, mask2: &Vec<char>) -> (Vec<u32>, bool) {
    let mut overlaps = Vec::new();
    for (&m1, &m2) in mask1.iter().zip(mask2.iter()) {
        match (m1, m2) {
            ('X', 'X') => overlaps.push(2),
            ('X', _) | (_, 'X') | ('1', '1') | ('0', '0') => overlaps.push(1),
            ('0', '1') | ('1', '0') => return (Vec::new(), false),
            _ => unreachable!(),
        }
    }
    (overlaps, true)
}

#[derive(Debug)]
struct Command {
    masked: Vec<char>,
    val: u64,
}

fn parse_commands(input: &str) -> Vec<Command> {
    let mut mask = Vec::new();
    let mut commands = Vec::new();
    let lines = input.trim().lines();

    for l in lines {
        if l.starts_with("mask") {
            // update current mask
            mask = l.split(" ").nth(2).unwrap().chars().collect();
        } else {
            let captures = MEM_RE.captures(l).unwrap();
            let address = format!(
                "{:036b}",
                captures
                    .name("address")
                    .unwrap()
                    .as_str()
                    .parse::<u64>()
                    .unwrap()
            )
            .chars()
            .collect();
            let val = captures.name("value").unwrap().as_str().parse().unwrap();
            let masked = apply_mask(&mask, &address);
            commands.push(Command { masked, val });
        }
    }

    commands.reverse();
    commands
}

fn solve(input: &str) -> u64 {
    // XXX: does not work
    // general thought is
    // - parse masks (apply mask to each following line)
    // - reverse the list of masks
    // for each command:
    //   calculate some sort of mask overlap against each of the previous masks
    //   use final overlap to figure out how many Xs are still left
    //   sum += value * num_X_left
    let commands: Vec<Command> = parse_commands(input);
    let mut sum = 0;
    for (i, command) in commands.iter().enumerate() {
        let mut master_overlap = vec![false; command.masked.len()];
        dbg!("next");
        for prev in commands[0..i].iter() {
            dbg!("new comparison", &command, &prev);
            let (curr_overlap, valid) = get_overlap(&command.masked, &prev.masked);
            dbg!(&curr_overlap, valid);
            if !valid {
                continue;
            }

            // update master list
            for (j, &v) in curr_overlap.iter().enumerate() {
                if v == 2 {
                    master_overlap[j] = true;
                }
            }
        }
        let num_x = command
            .masked
            .iter()
            .zip(master_overlap.iter())
            .filter(|(&bit, &overlap)| {
                if overlap {
                    return false;
                } else if bit == 'X' {
                    return true;
                }
                return false;
            })
            .count();
        sum += command.val * num_x as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(solve(input), 208);
    }

    #[test]
    fn case2() {
        let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[1] = 1";
        assert_eq!(solve(input), 408);
    }
}

advent_2020::read_main!();
