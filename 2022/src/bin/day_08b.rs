fn solve(input: &str) -> usize {
    let forest: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut max_score = 0;

    let forest_width = forest[0].len();
    let forest_height = forest.len();

    for tree_y in 0..forest_height {
        for tree_x in 0..forest_width {
            let mut visible;
            let mut cur_view = 0;
            let cur_height = forest[tree_x][tree_y];
            // up
            for y in (0..tree_y).rev() {
                cur_view += 1;
                if forest[tree_x][y] >= cur_height {
                    break;
                }
            }
            visible = cur_view;
            cur_view = 0;
            // down
            for y in tree_y + 1..forest_height {
                cur_view += 1;
                if forest[tree_x][y] >= cur_height {
                    break;
                }
            }
            visible *= cur_view;
            cur_view = 0;
            // left
            for x in (0..tree_x).rev() {
                cur_view += 1;
                if forest[x][tree_y] >= cur_height {
                    break;
                }
            }
            visible *= cur_view;
            cur_view = 0;
            // right
            for x in tree_x + 1..forest_width {
                cur_view += 1;
                if forest[x][tree_y] >= cur_height {
                    break;
                }
            }
            visible *= cur_view;

            if visible > max_score {
                max_score = visible;
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"30373
25512
65332
33549
35390";
        assert_eq!(solve(input), 8);
    }
}

util::read_main!();
