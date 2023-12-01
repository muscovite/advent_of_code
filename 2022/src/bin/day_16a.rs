use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

const TICKS: u32 = 30;

#[derive(Eq, PartialEq, Debug)]
struct Valve<'a> {
    name: &'a str,
    cost: u32,
}

impl<'a> Ord for Valve<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<'a> PartialOrd for Valve<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str) -> u32 {
    let valves: HashMap<&str, (u32, Vec<&str>)> = input
        .trim()
        .lines()
        .map(|l| {
            let mut l = l.strip_prefix("Valve ").unwrap().split(" ");

            let name = l.next().unwrap();

            let mut l = l.skip(2);

            let flow_rate = l
                .next()
                .unwrap()
                .strip_prefix("rate=")
                .unwrap()
                .strip_suffix(";")
                .unwrap()
                .parse()
                .unwrap();

            let l = l.skip(4);
            let neighbors = l
                .map(|segment| match segment.strip_suffix(",") {
                    Some(s) => s,
                    None => segment,
                })
                .collect();
            (name, (flow_rate, neighbors))
        })
        .collect();

    let mut priority_queue: BinaryHeap<Valve> = BinaryHeap::new();
    let (cost, _) = valves.get("AA").unwrap();

    priority_queue.push(Valve {
        name: "AA",
        cost: *cost as u32,
    });

    let mut total_pressure = 0;
    let mut pressure_per_tick = 0;
    let mut opened: HashSet<&str> = HashSet::new();
    let mut i = 0;

    while i < TICKS {
        dbg!(total_pressure, pressure_per_tick, i);

        let curr = priority_queue.pop().unwrap();
        let (flow_rate, neighbors) = valves.get(curr.name).unwrap();
        dbg!(&curr, flow_rate);

        if i == TICKS - 1 {
            return total_pressure + pressure_per_tick;
        }

        if !opened.contains(curr.name) && *flow_rate != 0 {
            i += 2;
            total_pressure += 2 * pressure_per_tick;
            pressure_per_tick += flow_rate;
        } else {
            i += 1;
            total_pressure += pressure_per_tick;
            opened.insert(curr.name);
        }

        priority_queue.extend(neighbors.iter().map(|&name| {
            if opened.contains(name) {
                return Valve { name, cost: 0 };
            }
            let (flow_rate, _) = valves.get(name).unwrap();
            Valve {
                name,
                cost: flow_rate * (TICKS - i),
            }
        }));
    }

    total_pressure
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(solve(input), 1651);
    }
}

util::read_main!();
