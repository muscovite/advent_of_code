use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const TARGET_VAL: u32 = 2020;

fn solve(expenses: &Vec<u32>) -> u32 {
    let (mut left, mut right) = (0, expenses.len() - 1);

    loop {
        // condense boundaries
        while expenses[left] + expenses[left + 1] + expenses[right] > TARGET_VAL {
            right -= 1;
        }
        let search_val = TARGET_VAL - expenses[left] - expenses[right];
        match expenses.binary_search(&search_val) {
            Ok(_) => break expenses[left] * expenses[right] * search_val,
            Err(_) => {
                left += 1;
            }
        };
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
        let test_input = vec![299, 366, 675, 979, 1456, 1721];
        let solution = solve(&test_input);
        assert_eq!(solution, 241861950);
    }
}
