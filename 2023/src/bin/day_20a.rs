use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Type {
    // %
    FlipFlop(bool),
    // &
    Con,
    Broadcast,
}

fn solve(input: &str) -> usize {
    let mut map: HashMap<&str, (Type, Vec<&str>)> = input
        .lines()
        .map(|l| {
            let (module, destinations) = l.split_once(" -> ").unwrap();
            let destinations = destinations.split(", ").collect();
            if module == "broadcaster" {
                return (module, (Type::Broadcast, destinations));
            }

            match module.strip_prefix("%") {
                Some(name) => (name, (Type::FlipFlop(false), destinations)),
                _ => match module.strip_prefix("&") {
                    Some(name) => (name, (Type::Con, destinations)),
                    _ => unreachable!(),
                },
            }
        })
        .collect();

    let mut cons_map: HashMap<&str, HashMap<&str, bool>> = map
        .iter()
        .filter_map(|(name, (t, _))| match t {
            Type::Con => Some((*name, HashMap::new())),
            _ => None,
        })
        .collect();

    for (name, (_, destinations)) in map.iter() {
        for dest in destinations {
            if let Some(inputs) = cons_map.get_mut(dest) {
                inputs.insert(name, false);
            }
        }
    }

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let (_, start_dests) = map.remove("broadcaster").unwrap();
    for _ in 0..1000 {
        let mut frontier =
            VecDeque::from_iter(start_dests.iter().map(|dest| ("broadcaster", false, *dest)));
        low_pulses += 1;
        while let Some((source, pulse, dest)) = frontier.pop_front() {
            if pulse {
                high_pulses += 1
            } else {
                low_pulses += 1
            }

            let Some((curr_type, destinations)) = map.get_mut(dest) else {
                continue;
            };

            match curr_type {
                Type::FlipFlop(state) => {
                    if pulse {
                        continue;
                    }

                    *state = !*state;

                    frontier.extend(
                        destinations
                            .into_iter()
                            .map(|new_dest| (dest, *state, *new_dest)),
                    );
                }
                Type::Con => {
                    // if it remembers high pulses for all inputs,low pulse
                    // otherwise, it sends a high pulse
                    let dest_states = cons_map.get_mut(&dest).unwrap();
                    let state = dest_states.get_mut(&source).unwrap();
                    *state = pulse;

                    let new_pulse = !dest_states.iter().all(|(_, s)| *s);

                    frontier.extend(
                        destinations
                            .into_iter()
                            .map(|new_dest| (dest, new_pulse, *new_dest)),
                    )
                }
                _ => (),
            }
        }
    }

    low_pulses * high_pulses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(solve(input), 11687500);
    }

    #[test]
    fn case2() {
        let input = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(solve(input), 32000000);
    }
}

util::read_main!();
