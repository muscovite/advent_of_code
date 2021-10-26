fn num_trees(map_chunk: &Vec<Vec<char>>, delta_x: usize, delta_y: usize) -> usize {
    let (mut curr_x, mut curr_y) = (delta_x, delta_y);
    let mut num_trees = 0;
    let map_width = map_chunk[0].len();
    while curr_y < map_chunk.len() {
        if map_chunk[curr_y][curr_x] == '#' {
            num_trees += 1;
        }
        curr_y += delta_y;
        curr_x = (curr_x + delta_x) % map_width;
    }
    num_trees
}

fn solve(input: &str) -> usize {
    // XXX: could probably optimize because input dimensions are known:
    // 31w x 323h
    let map_chunk: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let cases: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    cases
        .iter()
        .map(|(x_delta, y_delta)| num_trees(&map_chunk, *x_delta, *y_delta))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(solve(input), 336);
    }
}

util::read_main!();
