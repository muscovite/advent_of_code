const NUM_CUPS: usize = 1000000;

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
    cups.extend(10..=1000000);

    let mut curr_idx = 0;
    for foo in 0..iterations {
        if foo % 1000 == 0 {
            dbg!(foo);
        }

        let mut search_val = cups[curr_idx];
        let orig_val = search_val;
        // remove the three cups that are immediately clockwise of the current cup
        // XXX: SUPER GROSS
        let (mut next1, mut next2, mut next3) = (0, 0, 0);
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
            search_val -= 1;
            let candidate = cups.iter().enumerate().find(|&(_, v)| *v == search_val);
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
    let star1 = cups[get_idx(idx, 1, NUM_CUPS)];
    let star2 = cups[get_idx(idx, 2, NUM_CUPS)];
    (star1 * star2).to_string()
}

fn solve(input: &str) -> String {
    run(input, 10000000)
}

util::read_main!();
