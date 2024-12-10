use itertools::{self, Itertools};
use std::collections::HashSet;
use std::iter::repeat_n;

// even index == file, odd index == empty space
fn solve(input: &str) -> usize {
    let mut empty_spaces = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .flat_map(|(i, chunk_size)| repeat_n(i % 2 == 0, chunk_size as usize))
        .enumerate()
        .chunk_by(|(_, is_file)| *is_file)
        .into_iter()
        .filter_map(|(is_file, chunk)| {
            if is_file {
                return None;
            }
            let mut chunk = chunk;
            let (min, _) = chunk.next().unwrap();
            if let Some((max, _)) = chunk.last() {
                return Some(min..=max);
            }
            Some(min..=min)
        })
        .collect::<HashSet<_>>();

    let files = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .flat_map(|(i, chunk_size)| repeat_n((i % 2 == 0, i / 2), chunk_size as usize))
        .enumerate()
        .filter_map(|(file_idx, (is_file, file_num))| {
            if is_file {
                return Some((file_idx, file_num));
            }
            None
        })
        .chunk_by(|(_, file_num)| *file_num);
    let files = files.into_iter().map(|(_, chunk)| {
        let mut chunk = chunk;
        let (min, file_num) = chunk.next().unwrap();
        if let Some((max, _)) = chunk.last() {
            return (min..=max, file_num);
        }
        (min..=min, file_num)
    });

    let mut checksum: usize = 0;
    for (file_bounds, file_num) in files.collect::<Vec<_>>().into_iter().rev() {
        let space = empty_spaces
            .iter()
            .filter(|r| {
                (r.end() - r.start()) >= (file_bounds.end() - file_bounds.start())
                    && *r.end() < *file_bounds.start()
            })
            .min_by_key(|r| r.start())
            .cloned();
        match space {
            Some(space) => {
                empty_spaces.remove(&space);

                checksum += space
                    .clone()
                    .zip(file_bounds.clone())
                    .map(|(i, _)| i * file_num)
                    .sum::<usize>();

                let file_len = file_bounds.end() - file_bounds.start() + 1;
                if space.end() - space.start() != file_bounds.end() - file_bounds.start() {
                    empty_spaces.insert(space.start() + file_len..=*space.end());
                }
            }
            None => {
                checksum += file_bounds
                    .map(|file_idx| file_idx * file_num)
                    .sum::<usize>();
            }
        }
    }
    checksum
}

/*
12345

0..111....22222

2333133121414131402

00...111...2...333.44.5555.6666.777.888899
0099.111...2...333.44.5555.6666.777.8888..
0099.1117772...333.44.5555.6666.....8888..
0099.111777244.333....5555.6666.....8888..
00992111777.44.333....5555.6666.....8888..

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"2333133121414131402";
        assert_eq!(solve(input), 2858);
    }

    #[test]
    fn case2() {
        let input = r"12345";
        assert_eq!(solve(input), 132);
    }
}

util::read_main!();
