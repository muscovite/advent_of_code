fn solve(input: &str) -> usize {
    let num_lines = input.trim().lines().count();
    let mut rev_iter = input.trim().lines().rev();
    let operators = rev_iter.next().unwrap().split_whitespace();
    let mut numbers: Vec<_> = rev_iter
        .rev()
        .take(num_lines - 1)
        .map(|l| l.chars())
        .collect();

    operators
        .map(|op| {
            let mut subtotal = 0;
            loop {
                let next_chars: Vec<_> = numbers
                    .iter_mut()
                    .flat_map(|iter| match iter.next() {
                        Some(' ') | None => None,
                        Some(c) => Some(c),
                    })
                    .collect();

                if next_chars.len() == 0 {
                    return subtotal;
                }

                let n: usize = next_chars
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| (c as u8 - b'0') as usize * 10_usize.pow(i as u32))
                    .sum();

                match op {
                    "+" => subtotal += n,
                    "*" => {
                        if subtotal == 0 {
                            subtotal = 1;
                        }
                        subtotal *= n
                    }
                    _ => unreachable!(),
                };
            }
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
        assert_eq!(solve(input), 3263827);
    }
}

util::read_main!();
