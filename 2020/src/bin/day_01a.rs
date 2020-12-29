use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const TARGET_VAL: u32 = 2020;

fn solve(expenses: &Vec<u32>) -> u32 {
    let (mut left, mut right) = (0, expenses.len() - 1);

    loop {
        if left == right || right >= expenses.len() {
            panic!("Did not find value ):")
        }
        let sum = expenses[left] + expenses[right];
        if sum == TARGET_VAL {
            break expenses[left] * expenses[right];
        } else if sum < TARGET_VAL {
            left += 1;
        } else if sum > TARGET_VAL {
            left = left.saturating_sub(1);
            right -= 1;
        }
    }
}

fn main() {
    let f = File::open("inputs/day_01").unwrap();
    let f = BufReader::new(f);

    let mut expenses: Vec<u32> = f
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .filter(|&n| n < 2020)
        .collect();
    expenses.sort();

    println!("{}", solve(&expenses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let test_input = vec![1, 4, 5, 10, 100, 2016, 2017, 2018];
        let solution = solve(&test_input);
        assert_eq!(solution, 2016 * 4);
    }
}
