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
        // NB: "inner corners" check doesn't overcount because the checks consistently
        // use left/right emptiness, so any up/down emptiness isn't counted
        let mut corners = 0;
        while let Some((x, y)) = frontier.pop() {
            area += 1;

            let (top, bottom, left, right, top_left, top_right, bottom_left, bottom_right) = (
                (x, y - 1),
                (x, y + 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
            );

            // top left is corner
            if ref_plots.get(&left) != Some(&c)
                && ref_plots.get(&top_left) != Some(&c)
                && ref_plots.get(&top) != Some(&c)
            {
                corners += 1;
            }

            // top right is corner
            if ref_plots.get(&top) != Some(&c)
                && ref_plots.get(&top_right) != Some(&c)
                && ref_plots.get(&right) != Some(&c)
            {
                corners += 1;
            }

            // bottom left is corner
            if ref_plots.get(&left) != Some(&c)
                && ref_plots.get(&bottom_left) != Some(&c)
                && ref_plots.get(&bottom) != Some(&c)
            {
                corners += 1;
            }

            // bottom right is corner
            if ref_plots.get(&right) != Some(&c)
                && ref_plots.get(&bottom_right) != Some(&c)
                && ref_plots.get(&bottom) != Some(&c)
            {
                corners += 1;
            }

            // top left is inner corner
            if ref_plots.get(&left) != Some(&c) && ref_plots.get(&top_left) == Some(&c) {
                corners += 1;
            }

            // top right is inner corner
            if ref_plots.get(&right) != Some(&c) && ref_plots.get(&top_right) == Some(&c) {
                corners += 1;
            }

            // bottom left is inner corner
            if ref_plots.get(&left) != Some(&c) && ref_plots.get(&bottom_left) == Some(&c) {
                corners += 1;
            }

            // bottom right is inner corner
            if ref_plots.get(&right) != Some(&c) && ref_plots.get(&bottom_right) == Some(&c) {
                corners += 1;
            }

            // get next
            for new_pos in [top, bottom, left, right].into_iter() {
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
        price += area * (corners);
    }

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case0() {
        let input = r"AA
AB";
        assert_eq!(solve(input), 22);
    }

    #[test]
    fn case_surrounded() {
        let input = r"AAA
ABA
AAA";
        assert_eq!(solve(input), 68);
    }

    #[test]
    fn case1() {
        let input = r"AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(input), 80);
    }

    #[test]
    fn case2() {
        let input = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        assert_eq!(solve(input), 236);
    }

    #[test]
    fn case3() {
        let input = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!(solve(input), 368);
    }

    #[test]
    fn case4() {
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
        assert_eq!(solve(input), 1206);
    }

    #[test]
    fn case5() {
        let input = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(solve(input), 436);
    }
}

util::read_main!();
