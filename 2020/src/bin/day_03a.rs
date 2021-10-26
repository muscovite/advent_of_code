fn solve(input: &str) -> u32 {
    // XXX: could probably optimize because input dimensions are known:
    // 31w x 323h
    let map_chunk: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let (mut curr_x, mut curr_y) = (3, 1);
    let mut num_trees = 0;
    let map_width = map_chunk[0].len();
    while curr_y < map_chunk.len() {
        if map_chunk[curr_y][curr_x] == '#' {
            num_trees += 1;
        }
        curr_y += 1;
        curr_x = (curr_x + 3) % map_width;
    }
    num_trees
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
        assert_eq!(solve(input), 7);
    }
}

util::read_main!();
