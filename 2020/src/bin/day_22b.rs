use std::collections::{HashSet, VecDeque};

// return winning deck?
fn play_game(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) -> u32 {
    let mut prev_rounds = HashSet::new();
    // play game
    while deck1.len() > 0 && deck2.len() > 0 {
        // If there was a previous round in this game that had exactly the same
        // cards in the same order in the same players' decks, player 1 wins game
        let curr_decks = (deck1.clone(), deck2.clone());
        if prev_rounds.contains(&curr_decks) {
            // game ends instantly
            return 1;
        }
        prev_rounds.insert(curr_decks);

        // draw top cards
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        // if both players have at least as many cards
        // remaining in their deck as the value of the card they just drew,
        // the winner of the round is determined by playing a new game of
        // Recursive Combat (see below).
        if deck1.len() >= card1 && deck2.len() >= card2 {
            let deck1_slices = deck1.as_slices();
            let deck2_slices = deck2.as_slices();

            let winner = play_game(
                &mut deck1_slices
                    .0
                    .iter()
                    .chain(deck1_slices.1.iter())
                    .take(card1)
                    .copied()
                    .collect(),
                &mut deck2_slices
                    .0
                    .iter()
                    .chain(deck2_slices.1.iter())
                    .take(card2)
                    .copied()
                    .collect(),
            );
            if winner == 1 {
                deck1.push_back(card1);
                deck1.push_back(card2);
            } else {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
        // Otherwise, at least one player must not have enough cards left in
        // their deck to recurse; the winner of the round is the player with
        // the higher-value card.
        } else if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    // game winners
    if deck1.len() > 0 {
        1
    } else {
        2
    }
}

fn solve(input: &str) -> usize {
    let mut decks = input
        .split("\n\n")
        .map(|deck| deck.lines().skip(1).map(|l| l.parse().unwrap()).collect());

    let mut deck1 = decks.next().unwrap();
    let mut deck2 = decks.next().unwrap();

    let winning_deck = match play_game(&mut deck1, &mut deck2) {
        1 => deck1,
        2 => deck2,
        _ => unreachable!(),
    };
    winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as usize * v)
        .sum()
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
        assert_eq!(solve(input), 291);
    }

    #[test]
    fn case2() {
        let input = r"Player 1:
43
19

Player 2:
2
29
14";
        assert_eq!(solve(input), 105);
    }
}

advent_2020::read_main!();
