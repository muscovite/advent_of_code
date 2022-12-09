use std::collections::HashSet;

fn follow(head: &(i32, i32), tail: &mut (i32, i32)) {
    if *head == *tail {
        return;
    }

    if tail.1 - head.1 > 1 {
        tail.1 = tail.1 - 1;
        if tail.0 != head.0 {
            tail.0 = head.0;
        }
    } else if head.1 - tail.1 > 1 {
        tail.1 = tail.1 + 1;
        if tail.0 != head.0 {
            tail.0 = head.0;
        }
    } else if tail.0 - head.0 > 1 {
        tail.0 = tail.0 - 1;
        if tail.1 != head.1 {
            tail.1 = head.1;
        }
    } else if head.0 - tail.0 > 1 {
        tail.0 = tail.0 + 1;
        if tail.1 != head.1 {
            tail.1 = head.1;
        }
    }
}

fn take_step(head: &mut (i32, i32), tail: &mut (i32, i32), dir: &str) {
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
    follow(head, tail);
}

fn solve(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    for (direction, step) in input.trim().lines().map(|l| l.split_once(" ").unwrap()) {
        let steps = step.parse::<usize>().unwrap();
        for _ in 0..steps {
            take_step(&mut head, &mut tail, direction);
            visited.insert(tail);
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
        assert_eq!(solve(input), 13);
    }
}

util::read_main!();
