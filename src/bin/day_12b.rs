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
    ship: (i32, i32),     // position relative to starting location
    waypoint: (i32, i32), // positiion relative to ship
}

impl State {
    fn new(ship: (i32, i32), waypoint: (i32, i32)) -> State {
        State { ship, waypoint }
    }

    fn advance(&mut self, command: &Command) {
        match command {
            Command::North(v) => self.waypoint = (self.waypoint.0, self.waypoint.1 - v),
            Command::South(v) => self.waypoint = (self.waypoint.0, self.waypoint.1 + v),
            Command::East(v) => self.waypoint = (self.waypoint.0 + v, self.waypoint.1),
            Command::West(v) => self.waypoint = (self.waypoint.0 - v, self.waypoint.1),
            Command::Left(v) => {
                let (old_x, old_y) = self.waypoint;
                self.waypoint = match v {
                    270 => (old_y * -1, old_x),
                    180 => (old_x * -1, old_y * -1),
                    90 => (old_y, old_x * -1),
                    _ => unreachable!(),
                };
            }
            Command::Right(v) => {
                let (old_x, old_y) = self.waypoint;
                self.waypoint = match v {
                    90 => (old_y * -1, old_x),
                    180 => (old_x * -1, old_y * -1),
                    270 => (old_y, old_x * -1),
                    _ => unreachable!(),
                }
            }
            Command::Forward(v) => {
                // move towards waypoint N times
                let (wx, wy) = self.waypoint;
                self.ship = (self.ship.0 + (wx * v), self.ship.1 + (wy * v));
            }
        }
    }
}

fn solve(input: &str) -> i32 {
    let actions: Vec<_> = input.trim().lines().map(|l| Command::new(l)).collect();

    let mut state = State::new((0, 0), (10, -1));

    for action in actions {
        state.advance(&action);
    }

    state.ship.0.abs() + state.ship.1.abs()
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
