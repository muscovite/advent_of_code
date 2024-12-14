fn get_safety_factor(input: &str, width: isize, height: isize, steps: isize) -> usize {
    let final_positions: Vec<_> = input
        .lines()
        .map(|l| {
            let (robot, velocity) = l.split_once(" ").unwrap();

            let (robot_x, robot_y) = robot.strip_prefix("p=").unwrap().split_once(",").unwrap();
            let (robot_x, robot_y) = (
                robot_x.parse::<isize>().unwrap(),
                robot_y.parse::<isize>().unwrap(),
            );

            let (v_x, v_y) = velocity
                .strip_prefix("v=")
                .unwrap()
                .split_once(",")
                .unwrap();
            let (v_x, v_y) = (v_x.parse::<isize>().unwrap(), v_y.parse::<isize>().unwrap());

            (
                (robot_x + (v_x * steps)).rem_euclid(width),
                (robot_y + (v_y * steps)).rem_euclid(height),
            )
        })
        .collect();

    [
        // top left
        (0..width / 2, 0..height / 2),
        // top right
        (1 + width / 2..width, 0..height / 2),
        // bottom left
        (0..width / 2, 1 + height / 2..height),
        // bottom right
        (1 + width / 2..width, 1 + height / 2..height),
    ]
    .into_iter()
    .map(|(x_bounds, y_bounds)| {
        final_positions
            .iter()
            .filter(|(x, y)| x_bounds.contains(x) && y_bounds.contains(y))
            .count()
    })
    .product()
}

fn solve(input: &str) -> usize {
    get_safety_factor(input, 101, 103, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let input = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!(get_safety_factor(input, 11, 7, 100), 12);
    }
}

util::read_main!();
