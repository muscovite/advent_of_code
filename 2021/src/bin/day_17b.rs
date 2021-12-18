// this is gross and ugly, but coding math isn't fun
fn solve(input: &str) -> i32 {
    let (x_str, y_str) = input.trim().strip_prefix("target area: ").unwrap().split_once(", ").unwrap();
    let (x1, x2)= x_str.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (y1, y2)= y_str.strip_prefix("y=").unwrap().split_once("..").unwrap();

    let x1: i32 = x1.parse().unwrap();
    let x2: i32 = x2.parse().unwrap();
    let y1: i32 = y1.parse().unwrap();
    let y2: i32 = y2.parse().unwrap();

    let (x_min, x_max) = if x1 < x2 {
        (x1, x2)
    } else {
        (x2, x1)
    };

    let (y_min, y_max) = if y1 < y2 {
        (y1, y2)
    } else {
        (y2, y1)
    };

    let x_range = x_min..=x_max;
    let y_range = y_min..=y_max;


    let in_bounds = |x: i32, y: i32| -> bool {
        let mut x_vel = x;
        let mut y_vel = y;
        let mut x_tot= x;
        let mut y_tot = y;

        while y_tot >= y_min {
            if x_range.contains(&x_tot) && y_range.contains(&y_tot) {
                return true;
            }

            if x_vel != 0 {
                x_vel -=1;
            }
            y_vel -= 1;
            x_tot += x_vel;
            y_tot += y_vel;
        }
        return false;
    };

    let x_bound = if x_max % 2 == 0 {
        x_max/2
    } else {
        (x_max+1)/2
    };
    let y_bound = if y_min % 2 == 0 {
        (y_min/2)+1
    } else {
        (y_min+1)/2
    };

    // 0 .. x_bound
    // y_bound .. y_min * -1

    let count = (1..=x_bound).flat_map(move |x| {
        (y_bound..=y_min.abs()).filter(move |&y| in_bounds(x, y))
    }).count();

    ((x_max - x_min + 1) * ((y_min - y_max).abs() + 1)) + count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"target area: x=20..30, y=-10..-5";
        assert_eq!(solve(input), 112);
    }
}

util::read_main!();
