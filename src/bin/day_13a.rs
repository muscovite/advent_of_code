fn solve(input: &str) -> u32 {
    let mut input_iter = input.trim().lines();
    let target_time: u32 = input_iter.next().unwrap().parse().unwrap();
    let (bus, diff): (u32, u32) = input_iter
        .next()
        .unwrap()
        .split(',')
        .filter_map(|bus| {
            if bus == "x" {
                return None;
            }
            Some(bus.parse::<u32>().unwrap())
        })
        .map(|bus| {
            // NB: integer division
            let closest = (target_time / bus) * bus;
            if closest >= target_time {
                return (bus, closest - target_time);
            }
            return (bus, (closest + bus) - target_time);
        })
        .min_by(|(_, diff1), (_, diff2)| diff1.cmp(diff2))
        .unwrap();
    diff * bus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"939
7,13,x,x,59,x,31,19";
        assert_eq!(solve(input), 295);
    }
}

advent_2020::read_main!();
