use std::collections::HashSet;

// Return whether tail moved
fn follow(head: &(i32, i32), tail: &mut (i32, i32)) -> bool {
    if *head == *tail {
        return false;
    }

    if tail.1 - head.1 > 1 {
        // move up
        tail.1 = tail.1 - 1;
        if tail.0 < head.0 {
            tail.0 += 1;
        } else if tail.0 > head.0 {
            tail.0 -= 1;
        }
    } else if head.1 - tail.1 > 1 {
        // move down
        tail.1 = tail.1 + 1;
        if tail.0 < head.0 {
            tail.0 += 1;
        } else if tail.0 > head.0 {
            tail.0 -= 1;
        }
    } else if tail.0 - head.0 > 1 {
        // move left
        tail.0 = tail.0 - 1;
        if tail.1 < head.1 {
            tail.1 += 1;
        } else if tail.1 > head.1 {
            tail.1 -= 1;
        }
    } else if head.0 - tail.0 > 1 {
        // move right
        tail.0 = tail.0 + 1;
        if tail.1 < head.1 {
            tail.1 += 1;
        } else if tail.1 > head.1 {
            tail.1 -= 1;
        }
    } else {
        return false;
    }
    return true;
}

fn take_step(head: &mut (i32, i32), dir: &str) {
    match dir {
        "U" => {
            head.1 = head.1 - 1;
        }
        "D" => {
            head.1 = head.1 + 1;
        }
        "L" => {
            head.0 = head.0 - 1;
        }
        "R" => {
            head.0 = head.0 + 1;
        }
        _ => unreachable!(),
    }
}

fn solve(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = [(0, 0); 10];

    for (direction, step) in input.trim().lines().map(|l| l.split_once(" ").unwrap()) {
        let steps = step.parse::<usize>().unwrap();
        for _ in 0..steps {
            take_step(&mut rope[0], direction);
            for i in 0..9 {
                let (head_part, tail_part) = rope.split_at_mut(i + 1);
                if !follow(&head_part[i], &mut tail_part[0]) {
                    break;
                }
            }
            visited.insert(rope[9]);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn case2() {
        let input = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(solve(input), 36);
    }
}

util::read_main!();
