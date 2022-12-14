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
                    occupied.extend((y1.min(y2)..=y1.max(y2)).map(|y| (x1, y)));
                } else {
                    // horizontal line
                    occupied.extend((x1.min(x2)..=x1.max(x2)).map(|x| (x, y1)));
                }
            }
        }

        // find lowest map y
        let (_, lowest_rock) = *occupied.iter().max_by_key(|(_, y)| y).unwrap();
        Cave {
            occupied,
            floor_depth: lowest_rock + 2,
        }
    }

    // Return whether the source is now occupied
    fn drop_sand(&mut self) -> bool {
        // origin = 500, 0
        let mut sand_x = 500;
        let mut sand_y = 0;

        if self.occupied.contains(&(500, 0)) {
            return true;
        }

        loop {
            // right above the floor
            if sand_y == self.floor_depth - 1 {
                self.occupied.insert((sand_x, sand_y));
                return false;
            }

            let can_move = [sand_x, sand_x - 1, sand_x + 1]
                .into_iter()
                .find(|x| !self.occupied.contains(&(*x, sand_y + 1)));

            match can_move {
                Some(x) => {
                    if sand_y >= self.floor_depth {
                        return false;
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
        assert_eq!(solve(input), 93);
    }
}

util::read_main!();
