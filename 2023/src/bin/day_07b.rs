use std::collections::HashSet;

// Every hand is exactly one type. From strongest to weakest, they are:

// 7 Five of a kind, where all five cards have the same label: AAAAA
// 6 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
// 5 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
// 4 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
// 3 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
// 2 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
// 1 High card, where all cards' labels are distinct: 23456

fn card_to_val(c: char) -> u64 {
    match c.to_digit(10) {
        Some(n) => n as u64,
        None => match c {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("unknown card"),
        },
    }
}

fn hand_type(hand: &Vec<u64>) -> u64 {
    let card_types: HashSet<u64> = hand
        .iter()
        .filter_map(|&e| match e {
            1 => None,
            _ => Some(e),
        })
        .collect();
    let num_jokers = hand.iter().filter(|&&c| c == 1).count();
    match card_types.len() {
        // High card
        5 => 1,
        // One pair
        4 => 2,
        3 => {
            for card_type in card_types {
                // Three of a kind
                if hand.iter().filter(|&card| *card == card_type).count() + num_jokers == 3 {
                    return 4;
                }
            }
            // Two pair
            3
        }
        2 => {
            for card_type in card_types {
                // Four of a kind
                if hand.iter().filter(|&card| *card == card_type).count() + num_jokers == 4 {
                    return 6;
                }
            }
            // Full house
            5
        }
        // Five of a kind
        1 | 0 => 7,
        _ => panic!("not a game??"),
    }
}

fn solve(input: &str) -> usize {
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(" ").unwrap();

            let hand: Vec<u64> = hand.chars().map(|c| card_to_val(c)).collect();
            let hand_type = hand_type(&hand);

            (hand_type, hand, bid.parse::<usize>().unwrap())
        })
        .collect();

    hands.sort_by_key(|(hand_type, hand, _)| (*hand_type, hand.clone()));
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, _, bid))| (idx + 1) * bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(solve(input), 5905);
    }
}

util::read_main!();
