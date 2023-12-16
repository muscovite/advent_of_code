use std::collections::HashSet;

#[derive(Debug)]
enum Tile {
    UpMirror,
    DownMirror,
    VSplitter,
    HSplitter,
    Ground,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '/' => Tile::UpMirror,
            '\\' => Tile::DownMirror,
            '|' => Tile::VSplitter,
            '-' => Tile::HSplitter,
            '.' => Tile::Ground,
            _ => panic!("unknown tile"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect();
    let mut visited: HashSet<((usize, usize), Dir)> = HashSet::new();
    let mut beams: Vec<((usize, usize), Dir)> = vec![((0, 0), Dir::Right)];

    while let Some(((x, y), dir)) = beams.pop() {
        let mut x = x;
        let mut y = y;
        let mut dir = dir;

        loop {
            if !visited.insert(((x, y), dir)) {
                break;
            }

            let new_dir = match grid[y][x] {
                Tile::UpMirror => match dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                },
                Tile::DownMirror => match dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                },
                Tile::VSplitter => match dir {
                    Dir::Up | Dir::Down => dir,
                    Dir::Left | Dir::Right => {
                        beams.push(((x, y), Dir::Up));
                        Dir::Down
                    }
                },
                Tile::HSplitter => match dir {
                    Dir::Left | Dir::Right => dir,
                    Dir::Up | Dir::Down => {
                        beams.push(((x, y), Dir::Left));
                        Dir::Right
                    }
                },
                Tile::Ground => dir,
            };

            let new_x = match new_dir {
                Dir::Left => {
                    let Some(new_x) = x.checked_sub(1) else { break };
                    new_x
                }
                Dir::Right => x + 1,
                _ => x,
            };
            let new_y = match new_dir {
                Dir::Up => {
                    let Some(new_y) = y.checked_sub(1) else { break };
                    new_y
                }
                Dir::Down => y + 1,
                _ => y,
            };

            if new_x == grid[0].len() || new_y == grid.len() {
                break;
            }

            x = new_x;
            y = new_y;
            dir = new_dir;
        }
    }

    visited
        .into_iter()
        .map(|(coord, _)| coord)
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(solve(input), 46);
    }
}

util::read_main!();
