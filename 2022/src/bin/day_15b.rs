use std::collections::HashMap;

#[cfg(test)]
const UPPER_BOUND: i32 = 20;

#[cfg(not(test))]
const UPPER_BOUND: i32 = 4000000;

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn solve(input: &str) -> usize {
    let mut sensor_to_dist: HashMap<(i32, i32), u32> = HashMap::new();
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
        sensor_to_dist.insert((sensor_x, sensor_y), dist);
    }

    for ((sensor_x, sensor_y), dist) in sensor_to_dist.iter() {
        let edge = dist + 1;
        // Get all edge points
        let nw = (0..edge).map(|i| (sensor_x - i as i32, (sensor_y - edge as i32) + i as i32));
        let sw = (0..edge).map(|i| (sensor_x - i as i32, (sensor_y + edge as i32) - i as i32));
        let se = (0..edge).map(|i| (sensor_x + i as i32, (sensor_y + edge as i32) - i as i32));
        let ne = (0..edge).map(|i| (sensor_x + i as i32, (sensor_y - edge as i32) + i as i32));

        for (x, y) in ne.chain(nw.chain(sw.chain(se))) {
            if x < 0 || x > UPPER_BOUND || y < 0 || y > UPPER_BOUND {
                continue;
            }
            if sensor_to_dist
                .iter()
                .all(|((cx, cy), dist)| manhattan_distance(*cx, *cy, x, y) > *dist)
            {
                return (x as usize * 4000000) + y as usize;
            }
        }
    }

    unreachable!()
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
        assert_eq!(solve(input), 56000011);
    }
}

util::read_main!();
