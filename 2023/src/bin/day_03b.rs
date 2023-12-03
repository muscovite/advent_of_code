use std::collections::HashSet;

fn solve(input: &str) -> usize {
    let mut nums_and_coords: Vec<(usize, HashSet<(usize, usize)>)> = vec![];
    let mut symbol_coords: Vec<(usize, usize)> = vec![];

    input.lines().enumerate().for_each(|(j, l)| {
        let chars = l.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < l.len() {
            let curr = chars[i];

            if curr == '.' {
                i += 1;
                continue;
            }

            if curr.is_digit(10) {
                // Grab all successive numeric chars
                let mut num_str: Vec<char> = vec![curr];
                let start = i;
                i += 1;
                while i < l.len() && chars[i].is_digit(10) {
                    num_str.push(chars[i]);
                    i += 1;
                }

                // Convert to number
                let num: usize = num_str.iter().collect::<String>().parse::<usize>().unwrap();

                // Populate coord-to-num map
                let mut coords = HashSet::new();
                for idx in start..i {
                    coords.insert((idx, j));
                }
                nums_and_coords.push((num, coords));
                continue;
            } else if curr == '*' {
                // Char is a gear, track location
                symbol_coords.push((i, j));
            }

            i += 1;
        }
    });
    // Loop through symbols, find adjacent numbers

    let mut sum = 0;
    for (x, y) in symbol_coords.into_iter() {
        // seen nums_and_coords indexes
        let mut seen_nums: HashSet<usize> = HashSet::new();
        // Note: this only works because my input happens to not have any
        // gears with x = 0 or y = 0
        let matches: Vec<_> = [
            // top left
            (x - 1, y - 1),
            // top middle
            (x, y - 1),
            // top right
            (x + 1, y - 1),
            // left
            (x - 1, y),
            // right
            (x + 1, y),
            // bottom left
            (x - 1, y + 1),
            // bottom middle
            (x, y + 1),
            // bottom right
            (x + 1, y + 1),
        ]
        .iter()
        .filter_map(|coord| {
            for (idx, (num, coords)) in nums_and_coords.iter().enumerate() {
                if coords
                    .iter()
                    .any(|pot_match| coord == pot_match && !seen_nums.contains(&idx))
                {
                    seen_nums.insert(idx);
                    return Some(*num);
                }
            }
            None
        })
        // we don't need all the matches
        .take(3)
        .collect();

        if matches.len() != 2 {
            continue;
        }
        sum += matches.iter().product::<usize>();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(input), 467835);
    }
}

util::read_main!();
