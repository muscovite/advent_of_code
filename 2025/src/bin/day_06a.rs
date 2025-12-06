fn solve(input: &str) -> usize {
    let num_lines = input.trim().lines().count();
    let input_iter = input.trim().lines().map(|l| l.split_whitespace());
    let operators = input_iter.clone().rev().next().unwrap();
    let mut numbers: Vec<_> = input_iter
        .take(num_lines - 1)
        .map(|l| l.map(|n| n.parse::<usize>().unwrap()))
        .collect();

    operators
        .map(|op| {
            let next_nums = numbers.iter_mut().map(|iter| iter.next().unwrap());
            let subtotal: usize = match op {
                "+" => next_nums.sum(),
                "*" => next_nums.product(),
                _ => unreachable!(),
            };
            subtotal
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(input), 4277556);
    }
}

util::read_main!();
