enum Instruction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn line_to_instruction(line: &str) -> Instruction {
    let (dir, magnitude) = line.split_once(' ').unwrap();
    let magnitude: u32 = magnitude.parse().unwrap();
    match dir {
        "forward" => Instruction::Forward(magnitude),
        "up" => Instruction::Up(magnitude),
        "down" => Instruction::Down(magnitude),
        _ => panic!("invalid direction"),
    }
}

fn solve(input: &str) -> u32 {
    let instructions = input.trim().lines().map(|line| line_to_instruction(line));
    let mut y_total = 0;
    let mut x_total = 0;
    instructions.for_each(|instruction| match instruction {
        Instruction::Forward(x) => x_total += x,
        Instruction::Up(y) => y_total -= y,
        Instruction::Down(y) => y_total += y,
    });
    y_total * x_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(solve(input), 150);
    }
}

util::read_main!();
