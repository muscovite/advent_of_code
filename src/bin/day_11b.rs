// Return first seat seen in this direction
fn first_seat(seats: &Vec<Vec<char>>, mut i: usize, mut j: usize, dx: i32, dy: i32) -> char {
    let i_max = (seats.len() - 1) as i32;
    let j_max = (seats[0].len() - 1) as i32;
    loop {
        let new_i = i as i32 + dx;
        let new_j = j as i32 + dy;

        if new_i < 0 || new_i > i_max || new_j < 0 || new_j > j_max {
            return '.';
        }

        i = new_i as usize;
        j = new_j as usize;

        if seats[i][j] != '.' {
            return seats[i][j];
        }
    }
}

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
            let mut adjacencies: Vec<(i32, i32)> = Vec::new();
            // top-left
            if i > 0 && j > 0 {
                adjacencies.push((-1, -1));
            }
            // top-middle
            if i > 0 {
                adjacencies.push((-1, 0));
            }
            // top-right
            if i > 0 && j < col_max {
                adjacencies.push((-1, 1));
            }
            // left
            if j > 0 {
                adjacencies.push((0, -1));
            }
            // right
            if j < col_max {
                adjacencies.push((0, 1));
            }
            // bottom-left
            if i < row_max && j > 0 {
                adjacencies.push((1, -1));
            }
            // bottom-middle
            if i < row_max {
                adjacencies.push((1, 0));
            }
            // bottom-right
            if i < row_max && j < col_max {
                adjacencies.push((1, 1));
            }

            let num_occupied = adjacencies
                .iter()
                .filter(|(dx, dy)| first_seat(seats, i, j, *dx, *dy) == '#')
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
                    if num_occupied >= 5 {
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
        assert_eq!(solve(input), 26);
    }
}

advent_2020::read_main!();
