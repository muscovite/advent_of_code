// Return true when stable
fn make_move(seats: &Vec<Vec<char>>) -> (Vec<Vec<char>>, bool) {
    let mut unchanged = 0;
    let row_max = seats.len() - 1;
    let col_max = seats[0].len() - 1;
    let mut stable = false;
    let mut next_state: Vec<Vec<char>> = Vec::new();

    for (i, row) in seats.iter().enumerate() {
        next_state.push(Vec::new());
        for (j, seat) in row.iter().enumerate() {
            // floors remain unchanged
            if *seat == '.' {
                unchanged += 1;
                next_state[i].push('.');
                continue;
            }
            let mut adjacencies: Vec<(usize, usize)> = Vec::new();
            // top-left
            if i > 0 && j > 0 {
                adjacencies.push((i - 1, j - 1));
            }
            // top-middle
            if i > 0 {
                adjacencies.push((i - 1, j));
            }
            // top-right
            if i > 0 && j < col_max {
                adjacencies.push((i - 1, j + 1));
            }
            // left
            if j > 0 {
                adjacencies.push((i, j - 1));
            }
            // right
            if j < col_max {
                adjacencies.push((i, j + 1));
            }
            // bottom-left
            if i < row_max && j > 0 {
                adjacencies.push((i + 1, j - 1));
            }
            // bottom-middle
            if i < row_max {
                adjacencies.push((i + 1, j));
            }
            // bottom-right
            if i < row_max && j < col_max {
                adjacencies.push((i + 1, j + 1));
            }

            let num_occupied = adjacencies
                .iter()
                .filter(|(i, j)| seats[*i][*j] == '#')
                .count();
            let next = match *seat {
                'L' => {
                    if num_occupied == 0 {
                        '#'
                    } else {
                        'L'
                    }
                }
                '#' => {
                    if num_occupied >= 4 {
                        'L'
                    } else {
                        '#'
                    }
                }
                _ => *seat,
            };
            if next == *seat {
                unchanged += 1;
            }
            next_state[i].push(next);
        }
    }

    if unchanged == (row_max + 1) * (col_max + 1) {
        stable = true;
    }
    (next_state, stable)
}

fn solve(input: &str) -> usize {
    let mut seats: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();
    loop {
        let (new_seats, stable) = make_move(&seats);
        if stable {
            return seats
                .iter()
                .map(|row| row.iter().filter(|&&s| s == '#').count())
                .sum();
        }
        seats = new_seats;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve(input), 37);
    }
}

advent_2020::read_main!();
