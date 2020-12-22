use std::collections::VecDeque;

fn solve(input: &str) -> u64 {
    let mut input = input.split("\n\n");

    // player 1
    let mut deck1: VecDeque<u64> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    // player 2
    let mut deck2: VecDeque<u64> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|l| l.parse().unwrap())
        .collect();

    // play game
    while deck1.len() > 0 && deck2.len() > 0 {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    let winner = match deck1.len() {
        0 => deck2,
        _ => deck1
    };

    winner.iter().rev().enumerate().map(|(i, v)| (i + 1) as u64 * v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(solve(input), 306);
    }
}

advent_2020::read_main!();
