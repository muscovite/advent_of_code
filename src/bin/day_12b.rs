#[derive(Debug)]
enum Command {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

impl Command {
    fn new(input: &str) -> Command {
        let command = input.chars().next().unwrap();
        let value = input[1..].parse().unwrap();

        match command {
            'N' => Command::North(value),
            'S' => Command::South(value),
            'E' => Command::East(value),
            'W' => Command::West(value),
            'L' => Command::Left(value),
            'R' => Command::Right(value),
            'F' => Command::Forward(value),
            _ => panic!("bad command char"),
        }
    }
}
#[derive(Debug)]
struct State {
    ship_x: i32,
    ship_y: i32,
    wx: i32,
    wy: i32,
}

impl State {
    fn new(ship_x: i32, ship_y: i32, wx: i32, wy: i32) -> State {
        State {
            ship_x,
            ship_y,
            wx,
            wy,
        }
    }

    fn advance(&mut self, command: &Command) {
        match command {
            Command::North(v) => self.wy -= v,
            Command::South(v) => self.wy += v,
            Command::East(v) => self.wx += v,
            Command::West(v) => self.wx -= v,
            Command::Left(v) => {
                let (old_x, old_y) = (self.wx, self.wy);
                match v {
                    270 => {
                        self.wx = old_y * -1;
                        self.wy = old_x;
                    }
                    180 => {
                        self.wx = old_x * -1;
                        self.wy = old_y * -1;
                    }
                    90 => {
                        self.wx = old_y;
                        self.wy = old_x * -1;
                    }
                    _ => panic!("left turn invalid"),
                }
            }
            Command::Right(v) => {
                let (old_x, old_y) = (self.wx, self.wy);
                match v {
                    90 => {
                        self.wx = old_y * -1;
                        self.wy = old_x;
                    }
                    180 => {
                        self.wx = old_x * -1;
                        self.wy = old_y * -1;
                    }
                    270 => {
                        self.wx = old_y;
                        self.wy = old_x * -1;
                    }
                    _ => panic!("right turn invalid"),
                }
            }
            Command::Forward(v) => {
                let (old_x, old_y) = (self.wx, self.wy);
                // move towards waypoint N times
                self.ship_x += old_x * v;
                self.ship_y += old_y * v;
            }
        }
    }
}

fn solve(input: &str) -> i32 {
    let actions: Vec<_> = input.trim().lines().map(|l| Command::new(l)).collect();

    let mut state = State::new(0, 0, 10, -1);

    for action in actions {
        state.advance(&action);
    }

    state.ship_x.abs() + state.ship_y.abs()
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
        assert_eq!(solve(input), 286);
    }
}

advent_2020::read_main!();
