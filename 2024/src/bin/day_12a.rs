use std::collections::HashMap;

fn pop(map: &mut HashMap<(isize, isize), char>) -> Option<((isize, isize), char)> {
    let cloned_pos = map.keys().next()?.clone();
    return map.remove_entry(&cloned_pos);
}

fn solve(input: &str) -> usize {
    let mut plots: HashMap<(isize, isize), char> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars()
                .enumerate()
                .map(move |(i, c)| ((i as isize, j as isize), c))
        })
        .collect();

    let ref_plots = plots.clone();

    let mut price = 0;
    while let Some((pos, c)) = pop(&mut plots) {
        let mut frontier = vec![pos];
        let mut area = 0;
        let mut perimeter = 0;
        while let Some((x, y)) = frontier.pop() {
            area += 1;

            // find neighbors
            for new_pos in [
                // up
                (x, y - 1),
                // down
                (x, y + 1),
                // left
                (x - 1, y),
                // right
                (x + 1, y),
            ]
            .into_iter()
            {
                // perimeter
                if ref_plots.get(&new_pos) != Some(&c) {
                    perimeter += 1;
                }

                // get next
                match plots.get(&new_pos) {
                    Some(new_c) => {
                        if *new_c == c {
                            frontier.push(new_pos);
                            plots.remove(&new_pos);
                        }
                    }
                    None => {}
                }
            }
        }
        price += area * perimeter;
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let input = r"AA";
        assert_eq!(solve(input), 12);
    }

    #[test]
    fn case1() {
        let input = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(input), 140);
    }

    #[test]
    fn case2() {
        let input = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(input), 772);
    }

    #[test]
    fn case3() {
        let input = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(solve(input), 1930);
    }
}

util::read_main!();
