use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum ArrayState {
    None,
    MadeArray,
    ClosedArray,
}

impl ArrayState {
    fn transition(&mut self) {
        let new_state = match *self {
            ArrayState::None => ArrayState::MadeArray,
            ArrayState::MadeArray => ArrayState::ClosedArray,
            ArrayState::ClosedArray => ArrayState::None,
        };

        *self = new_state;
    }

    fn made_array(&mut self) {
        *self = ArrayState::MadeArray;
    }
}

fn check(pair1: &[u8], pair2: &[u8]) -> Ordering {
    let mut i = 0;
    let mut j = 0;

    let mut left_array_state = ArrayState::None;
    let mut right_array_state = ArrayState::None;

    while i < pair1.len() && j < pair2.len() {
        let left = pair1[i];
        let right = pair2[j];
        let left_isnum = (b'0'..=b':').contains(&left);
        let right_isnum = (b'0'..=b':').contains(&right);

        // dbg!(left as char, right as char);
        // dbg!("LEFT", &left_array_state);
        // dbg!("RIGHT", &right_array_state);

        // If the left list runs out of items first -> pass
        // If the right list runs out of items first -> fail

        if left == right {
            if left == b',' && right_array_state == ArrayState::ClosedArray {
                // dbg!("left close array");
                return Ordering::Greater;
            }
            if right == b',' && left_array_state == ArrayState::ClosedArray {
                // dbg!("right close array");
                return Ordering::Less;
            }

            if left_isnum && right_isnum {
                if left_array_state != ArrayState::None {
                    left_array_state.transition();
                }
                if right_array_state != ArrayState::None {
                    right_array_state.transition();
                }
            }

            i += 1;
            j += 1;
            continue;
        }

        if (left == b'[' || left == b',') && right == b']' {
            return Ordering::Greater;
        }

        if (right == b'[' || right == b',') && left == b']' {
            return Ordering::Less;
        }

        // If both values are integers:
        // left < right -> pass
        // left == right -> continue (already checked)
        // left >  right -> fail
        if left_isnum && right_isnum {
            // dbg!("num comparison", left as char, right as char);
            return left.cmp(&right);
        }

        // If both values are lists:
        // compare each element successively
        // If all elemnts are the same:
        // left.len() == right.len() -> continue
        // left.len() <  right.len() -> pass
        // left.len() >  right.len() -> fail

        // If exactly one value is integer:
        // wrap it in list, then continue
        // left was the number
        if left_isnum {
            if left_array_state != ArrayState::ClosedArray && right == b']' {
                return Ordering::Greater;
            }
            left_array_state.made_array();
            j += 1;
            continue;
        }

        // right was the number
        if right_array_state != ArrayState::ClosedArray && left == b']' {
            return Ordering::Less;
        }
        right_array_state.made_array();
        i += 1;
    }

    // assume they can't be equal if we got here?
    assert!(pair1.len() != pair2.len());
    // dbg!("reached end");
    pair1.len().cmp(&pair2.len())
}

// 6642 too low
// 6958 too high
// 6914 is wrong
// 7170 - too high
fn solve(input: &str) -> usize {
    let mut pairs: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(|chunk| chunk.split_once("\n").unwrap())
        .flat_map(|(a, b)| [a, b])
        .collect();

    pairs.extend(["[[2]]", "[[6]]"]);

    pairs.sort_by(|a, b| check(a.as_bytes(), b.as_bytes()));

    let idx1 = pairs.iter().position(|&x| x == "[[2]]").unwrap() + 1;
    let idx2 = pairs.iter().position(|&x| x == "[[6]]").unwrap() + 1;
    idx1 * idx2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(solve(input), 140);
    }
}

util::read_main!();
