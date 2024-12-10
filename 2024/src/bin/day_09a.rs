use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::{self, Itertools};
use std::iter::repeat_n;

// even index == file, odd index == empty space
fn solve(input: &str) -> usize {
    let empty_spaces = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .flat_map(|(i, chunk_size)| repeat_n(i % 2 == 0, chunk_size as usize))
        .enumerate()
        .filter_map(|(i, is_file)| {
            if !is_file {
                return Some(i);
            }
            None
        });

    let file_spaces: Vec<(usize, usize)> = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .flat_map(|(i, chunk_size)| repeat_n((i % 2 == 0, i / 2), chunk_size as usize))
        .enumerate()
        .filter_map(|(i, (is_file, file_num))| {
            if is_file {
                return Some((i, file_num));
            }
            None
        })
        .collect();
    let file_spaces = file_spaces.into_iter().rev();

    empty_spaces
        .zip_longest(file_spaces)
        .flat_map(|next| match next {
            Both(i, (file_idx, file_num)) => {
                if i < file_idx {
                    return Some(i * file_num);
                }
                Some(file_idx * file_num)
            }
            Left(_) => None,
            Right((file_idx, file_num)) => Some(file_idx * file_num),
        })
        .sum()
}

/*
12345

0..111....22222
02.111....2222.
022111....222..
0221112...22...
02211122..2....
022111222......

2333133121414131402

00...111...2...333.44.5555.6666.777.888899
009..111...2...333.44.5555.6666.777.88889.
0099.111...2...333.44.5555.6666.777.8888..
00998111...2...333.44.5555.6666.777.888...
009981118..2...333.44.5555.6666.777.88....
0099811188.2...333.44.5555.6666.777.8.....
009981118882...333.44.5555.6666.777.......
0099811188827..333.44.5555.6666.77........
00998111888277.333.44.5555.6666.7.........
009981118882777333.44.5555.6666...........
009981118882777333644.5555.666............
00998111888277733364465555.66.............
0099811188827773336446555566..............

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"2333133121414131402";
        assert_eq!(solve(input), 1928);
    }

    #[test]
    fn case2() {
        let input = r"12345";
        assert_eq!(solve(input), 60);
    }
}

util::read_main!();
