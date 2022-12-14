use std::collections::HashSet;

struct Cave {
    occupied: HashSet<(usize, usize)>,
    floor_depth: usize,
}

impl Cave {
    fn new(input: &str) -> Cave {
        let mut occupied: HashSet<(usize, usize)> = HashSet::new();
        for line in input.trim().lines() {
            let segments: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect();
            for pair in segments.windows(2) {
                let (x1, y1) = pair[0];
                let (x2, y2) = pair[1];
                if x1 == x2 {
                    // vertical line
                    if y1 > y2 {
                        occupied.extend((y2..=y1).map(|y| (x1, y)));
                    } else {
                        occupied.extend((y1..=y2).map(|y| (x1, y)));
                    }
                } else {
                    // horizontal line
                    if x1 > x2 {
                        occupied.extend((x2..=x1).map(|x| (x, y1)));
                    } else {
                        occupied.extend((x1..=x2).map(|x| (x, y1)));
                    }
                }
            }
        }

        // find lowest map y
        let (_, floor_depth) = *occupied.iter().max_by_key(|(_, y)| y).unwrap();
        Cave {
            occupied,
            floor_depth,
        }
    }

    // Return whether we reached lower bound, i.e we are now freefalling
    fn drop_sand(&mut self) -> bool {
        // origin = 500, 0
        let mut sand_x = 500;
        let mut sand_y = 0;

        loop {
            let mut can_move: Option<usize> = None;
            for &x in [sand_x, sand_x - 1, sand_x + 1].iter() {
                if !self.occupied.contains(&(x, sand_y + 1)) {
                    can_move = Some(x);
                    break;
                }
            }

            match can_move {
                Some(x) => {
                    if sand_y >= self.floor_depth {
                        return true;
                    }
                    sand_x = x;
                    sand_y += 1;
                }
                None => {
                    // all spots occupied, halted
                    self.occupied.insert((sand_x, sand_y));
                    return false;
                }
            }
        }
    }
}

fn solve(input: &str) -> usize {
    let mut cave = Cave::new(input);

    for i in 0.. {
        if cave.drop_sand() {
            return i;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(solve(input), 24);
    }
}

util::read_main!();
