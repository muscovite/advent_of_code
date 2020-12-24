const NUM_CUPS: usize = 9;

#[inline]
fn get_idx(curr: usize, offset: usize, len: usize) -> usize {
    (curr + offset) % len
}

fn run(input: &str, iterations: usize) -> String {
    let mut cups: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut curr_idx = 0;
    for _ in 0..iterations {
        let mut search_val = cups[curr_idx];
        let orig_val = search_val;
        // remove the three cups that are immediately clockwise of the current cup
        // XXX: SUPER GROSS
        let (mut next1, mut next2, mut next3) = (0,0,0);
        for (i, cup) in cups.iter().enumerate() {
            if *cup == orig_val {
                next1 = cups.remove(get_idx(i, 1, NUM_CUPS));
                break;
            }
        }

        for (i, cup) in cups.iter().enumerate() {
            if *cup == orig_val {
                next2 = cups.remove(get_idx(i, 1, NUM_CUPS - 1));
                break;
            }
        }

        for (i, cup) in cups.iter().enumerate() {
            if *cup == orig_val {
                next3 = cups.remove(get_idx(i, 1, NUM_CUPS - 2));
                break;
            }
        }

        // destination cup:
        // if current is lowest (excluding picked cups), use index of highest
        // else, use index of current - 1, or current - 2, or... etc
        let mut destination = curr_idx;
        loop {
            if search_val - 1 == 0 {
                // return index of max
                destination = cups
                    .iter()
                    .enumerate()
                    // .filter(|&(idx, _)| idx != next1 && idx != next2 && idx != next3)
                    .max_by_key(|&(_, v)| *v)
                    .unwrap()
                    .0;
                break;
            }
            search_val -=1;
            let candidate = cups
                .iter()
                .enumerate()
                .find(|&(_, v)| *v == search_val);
            match candidate {
                Some((idx, _)) => {
                    destination = idx;
                    break;
                }
                None => {
                   continue;
                }
            }
        }
        // move cups to the right of destination
        let right = cups.split_off(destination + 1);
        cups.push(next1);
        cups.push(next2);
        cups.push(next3);
        cups.extend(right);

        // pick next current cup - cup directly to right of current cup
        for (i, cup) in cups.iter().enumerate() {
            if *cup == orig_val {
                curr_idx = get_idx(i, 1, NUM_CUPS);
                break;
            }
        }
    }

    // 1 will always exist
    let mut idx = 0;
    for (i, cup) in cups.iter().enumerate() {
        if *cup == 1 {
            idx = i;
            break;
        }
    }
    let mut right = cups.split_off(idx);
    if right.len() > 0 {
        right.remove(0);
    } else {
        cups.remove(cups.len() - 1);
    }
    right.extend(cups);
    right.iter().map(|num| num.to_string()).collect::<Vec<_>>().join("")
}

fn solve(input: &str) -> String {
    run(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"389125467";
        assert_eq!(run(input, 100), "67384529");
    }

    #[test]
    fn case2() {
        let input = r"389125467";
        assert_eq!(run(input, 10), "92658374");
    }
}

advent_2020::read_main!();
