use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize, usize) -> usize>,
    num_inspected: usize,
    divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("num_inspected", &self.num_inspected)
            .field("divisor", &self.divisor)
            .field("true_monkey", &self.true_monkey)
            .field("false_monkey", &self.false_monkey)
            .finish()
    }
}

impl Monkey {
    fn new(chunk: &str) -> Monkey {
        let mut lines = chunk.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect::<VecDeque<usize>>();

        let (operator, operand) = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split_once(" ")
            .unwrap();

        // So... seems like this works because
        // if a + b = c, then a (mod N) + b (mod N) = c (mod N)
        let op_fn: Box<dyn Fn(usize, usize) -> usize> = match (operator, operand.parse::<usize>()) {
            ("+", Ok(num)) => Box::new(move |x, mega_divisor| (x + num) % mega_divisor),
            ("+", Err(_)) => Box::new(move |x, mega_divisor| (x + x) % mega_divisor),
            ("*", Ok(num)) => Box::new(move |x, mega_divisor| (x * num) % mega_divisor),
            ("*", Err(_)) => Box::new(move |x, mega_divisor| (x * x) % mega_divisor),
            _ => unreachable!(),
        };

        let divisor = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();

        let true_monkey = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        let false_monkey = lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();

        Monkey {
            items: items,
            num_inspected: 0,
            operation: op_fn,
            divisor: divisor,
            true_monkey: true_monkey,
            false_monkey: false_monkey,
        }
    }

    fn inspect(&mut self, mega_divisor: usize) -> Option<(usize, usize)> {
        if self.items.len() == 0 {
            return None;
        }

        self.num_inspected += 1;
        let item = self.items.pop_front().unwrap();
        let item = (self.operation)(item, mega_divisor);
        if item % self.divisor == 0 {
            return Some((self.true_monkey, item));
        }
        Some((self.false_monkey, item))
    }
}

fn solve(input: &str) -> usize {
    let mut monkeys: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(|chunk| Monkey::new(chunk))
        .collect();

    let mega_divisor: usize = monkeys.iter().map(|monkey| monkey.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some((pos, item)) = monkeys[i].inspect(mega_divisor) {
                monkeys[pos].items.push_back(item);
            }
        }
    }

    let mut inspections: Vec<_> = monkeys.iter().map(|monkey| monkey.num_inspected).collect();
    inspections.sort();
    inspections.iter().rev().take(2).product::<usize>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(solve(input), 2713310158);
    }
}

util::read_main!();
