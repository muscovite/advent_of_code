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

fn check(pair1: &[u8], pair2: &[u8]) -> bool {
    let mut i = 0;
    let mut j = 0;

    let mut left_array_state = ArrayState::None;
    let mut right_array_state = ArrayState::None;

    while i < pair1.len() && j < pair2.len() {
        let left = pair1[i];
        let right = pair2[j];
        let left_isnum = (b'0'..=b':').contains(&left);
        let right_isnum = (b'0'..=b':').contains(&right);

        // If the left list runs out of items first -> pass
        // If the right list runs out of items first -> fail

        if left == right {
            if left == b',' && right_array_state == ArrayState::ClosedArray {
                return false;
            }
            if right == b',' && left_array_state == ArrayState::ClosedArray {
                return true;
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
            return false;
        }

        if (right == b'[' || right == b',') && left == b']' {
            return true;
        }

        // If both values are integers:
        // left < right -> pass
        // left == right -> continue (already checked)
        // left >  right -> fail
        if left_isnum && right_isnum {
            return left < right;
        }

        // If both values are lists:
        // compare each element successively
        // If all elements are the same:
        // left.len() == right.len() -> continue
        // left.len() <  right.len() -> pass
        // left.len() >  right.len() -> fail

        // If exactly one value is integer:
        // wrap it in list, then continue
        // left was the number
        if left_isnum {
            if left_array_state != ArrayState::ClosedArray && right == b']' {
                return false;
            }
            left_array_state.made_array();
            j += 1;
            continue;
        }

        // right was the number
        if right_array_state != ArrayState::ClosedArray && left == b']' {
            return true;
        }
        right_array_state.made_array();
        i += 1;
    }

    // assume they can't be equal if we got here?
    assert!(pair1.len() != pair2.len());
    pair1.len() < pair2.len()
}

// 6642 too low
// 6958 too high
// 6914 is wrong
// 7170 - too high
fn solve(input: &str) -> usize {
    let pairs = input
        .trim()
        .split("\n\n")
        .map(|chunk| chunk.split_once("\n").unwrap());

    pairs
        .enumerate()
        .filter_map(|(idx, (pair1, pair2))| {
            if check(pair1.as_bytes(), pair2.as_bytes()) {
                return Some(idx + 1);
            }
            None
        })
        .sum()
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
        assert_eq!(solve(input), 13);
    }

    #[test]
    fn case2() {
        let input = r"
[1]
[[8]]
";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case3() {
        let input = r"
[9]
[[8]]
";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn left_fail_braces() {
        let input = r"
[9,8]
[[[[9]]]]
";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn right_fail_braces() {
        let input = r"
[7,8]
[[[[7,7]]]]
";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case6() {
        let input = r"
[[[]]]
[[],1]
";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case7() {
        let input = r"
[[],1]
[[[]]]
";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case8() {
        let input = r"
[[[],[[5],[6,9]]],[]]
[[5],[],[[8,[5],2],[2,0,[7],[8,2,1]],[1,[4,9,3],[8,1,3,1,4],[]],6,2]]
";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case9() {
        let input = r"
[[5],[],[[8,[5],2],[2,0,[7],[8,2,1]],[1,[4,9,3],[8,1,3,1,4],[]],6,2]]
[[[],[[5],[6,9]]],[]]
";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn case10() {
        let input = r"
[[],[6,[[8],[7,3,3,6],3,8,[]]],[[0],[],6,[5,:,2,2]],[[0,3,0],2,[[9],2],3],[9,[9],7,2,[6,[],[0,9],1,[0]]]]
[[3,[2,[:,3,0,6],0],1,9],[2,[[6]],[],[[1,1],7,[5,6,4,2],7,8],[0,[5,8,4],[]]]]
";
        assert_eq!(solve(input), 1);
    }
}

util::read_main!();
