enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Command {
    fn new(input: char) -> Command {
        match input {
            'N' => Command::North,
            'S' => Command::South,
            'E' => Command::East,
            'W' => Command::West,
            'L' => Command::Left,
            'R' => Command::Right,
            'F' => Command::Forward,
            _ => panic!("bad command char"),
        }
    }
}

struct Action {
    command: Command,
    value: i32,
}

struct State {
    x: i32,
    y: i32,
    direction: i32, // or enum?
}

impl State {
    fn new(x: i32, y: i32) -> State {
        State {
            x,
            y,
            direction: 90,
        }
    }

    fn advance(&mut self, action: &Action) {
        match action.command {
            Command::North => self.y -= action.value,
            Command::South => self.y += action.value,
            Command::East => self.x += action.value,
            Command::West => self.x -= action.value,
            Command::Left => self.direction = (self.direction + 360 - action.value) % 360,
            Command::Right => self.direction = (self.direction + action.value) % 360,
            Command::Forward => match self.direction {
                0 => self.y -= action.value,
                90 => self.x += action.value,
                180 => self.y += action.value,
                270 => self.x -= action.value,
                _ => panic!("invalid direction"),
            },
        }
    }
}

impl Action {
    fn new(input: &str) -> Action {
        Action {
            command: Command::new(input.chars().next().unwrap()),
            value: input[1..].parse().unwrap(),
        }
    }
}

fn solve(input: &str) -> i32 {
    let actions: Vec<_> = input.trim().lines().map(|l| Action::new(l)).collect();

    let mut state = State::new(0, 0);

    for action in actions {
        state.advance(&action);
    }

    state.x.abs() + state.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"F10
N3
F7
R90
F11";
        assert_eq!(solve(input), 25);
    }
}

advent_2020::read_main!();
