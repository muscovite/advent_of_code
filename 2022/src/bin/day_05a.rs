use regex::Regex;

fn solve(input: &str) -> String {
    let re = Regex::new(r"\[(?P<crate>[A-Z])\]").unwrap();
    let re_instr =
        Regex::new(r"move (?P<num>[0-9]+) from (?P<src>[0-9]+) to (?P<dst>[0-9]+)").unwrap();

    let (stacks, instructions) = input.trim_end().split_once("\n\n").unwrap();

    // Build stacks
    let mut stacks_iter = stacks.lines().rev();
    let num_line = stacks_iter.next().unwrap();
    let num_stacks: usize = num_line
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![Vec::<String>::new(); num_stacks];

    for line in stacks_iter {
        for caps in re.captures_iter(line) {
            let capture = &caps["crate"];
            let m = caps.get(0).unwrap();
            stacks[m.start() / 4].push(capture.to_owned());
        }
    }
    // Move stacks
    for line in instructions.lines() {
        for caps in re_instr.captures_iter(line) {
            let num: usize = caps["num"].parse().unwrap();
            let src: usize = caps["src"].parse().unwrap();
            let dst: usize = caps["dst"].parse().unwrap();

            let idx = stacks[src - 1].len() - num;
            let moving: Vec<String> = stacks[src - 1].drain(idx..).rev().collect();
            stacks[dst - 1].extend(moving);
        }
    }

    stacks
        .iter()
        .map(|stack| stack[stack.len() - 1].clone())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        assert_eq!(solve(input), "CMZ");
    }
}

util::read_main!();
