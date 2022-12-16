use std::collections::HashSet;

#[cfg(test)]
const ROW_Y: i32 = 10;

#[cfg(not(test))]
const ROW_Y: i32 = 2000000;

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn solve(input: &str) -> usize {
    let mut impossible_spots: HashSet<i32> = HashSet::new();
    for l in input.trim().lines() {
        let mut line = l.strip_prefix("Sensor at x=").unwrap().split(",");
        let sensor_x: i32 = line.next().unwrap().parse().unwrap();
        let (sensor_y_unparsed, beacon_x_unparsed) = line
            .next()
            .unwrap()
            .strip_prefix(" y=")
            .unwrap()
            .split_once(": closest beacon is at x=")
            .unwrap();
        let sensor_y: i32 = sensor_y_unparsed.parse().unwrap();
        let beacon_x: i32 = beacon_x_unparsed.parse().unwrap();
        let beacon_y: i32 = line
            .next()
            .unwrap()
            .strip_prefix(" y=")
            .unwrap()
            .parse()
            .unwrap();

        let dist = manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y);
        let y_bounds = sensor_y - dist as i32..sensor_y + dist as i32;
        if !y_bounds.contains(&ROW_Y) {
            continue;
        }

        impossible_spots.extend(
            (sensor_x - dist as i32..sensor_x + dist as i32)
                .filter(|x| manhattan_distance(sensor_x, sensor_y, *x, ROW_Y) <= dist),
        );
        if beacon_y == ROW_Y {
            impossible_spots.remove(&(beacon_x));
        }
    }
    impossible_spots.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(solve(input), 26);
    }
}

util::read_main!();
