use std::collections::HashMap;
use std::collections::HashSet;

const BOARD_LEN: u32 = 5;

type Coordinate = (u32, u32);
struct BoardState {
    nums_left: HashMap<u32, Coordinate>,
    coords_seen: HashSet<Coordinate>,
    won_at: Option<u32>,
}

impl BoardState {
    fn new(input: &str) -> BoardState {
        let mut nums_left = HashMap::new();
        let coords_seen = HashSet::new();
        let nums = input.split_whitespace().map(|s| s.parse().unwrap());

        let mut x = 0;
        let mut y = 0;
        for n in nums {
            nums_left.insert(n, (x, y));

            x = (x + 1) % BOARD_LEN;
            if x == 0 {
                y += 1;
            }
        }

        BoardState {
            nums_left,
            coords_seen,
            won_at: None,
        }
    }

    fn has_won(&self, coord: Coordinate) -> bool {
        (0..BOARD_LEN).all(|i| self.coords_seen.contains(&(coord.0, i)))
            || (0..BOARD_LEN).all(|i| self.coords_seen.contains(&(i, coord.1)))
    }

    fn make_move(&mut self, chosen_num: u32) -> bool {
        match self.nums_left.remove(&chosen_num) {
            Some(coord) => {
                self.coords_seen.insert(coord);
                if self.has_won(coord) {
                    self.won_at = Some(chosen_num);
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn final_score(&self) -> u32 {
        let chosen_num = self.won_at.expect("final_score called without winning");
        chosen_num * self.nums_left.keys().sum::<u32>()
    }
}

fn solve(input: &str) -> u32 {
    let mut input = input.trim().split("\n\n");
    let chosen_numbers = input.next().unwrap().split(',').map(|s| s.parse().unwrap());

    let mut boards: Vec<BoardState> = input.map(|b| BoardState::new(b)).collect();

    let mut last_won_board = 0;
    for chosen_num in chosen_numbers {
        boards.iter_mut().enumerate().for_each(|(i, b)| {
            if b.won_at.is_some() {
                return;
            }

            if b.make_move(chosen_num) {
                last_won_board = i
            }
        })
    }

    boards[last_won_board].final_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";
        assert_eq!(solve(input), 1924);
    }

    #[test]
    fn case2() {
        let input = r"22,13,17,11,0,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19";
        assert_eq!(solve(input), 0);
    }
}

util::read_main!();
