use std::collections::VecDeque;
// The pipes are arranged in a two-dimensional grid of tiles:
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your
// sketch doesn't show what shape the pipe has.

#[derive(Debug)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("invalid tile"),
        }
    }
}

type Map = Vec<Vec<Tile>>;
type Step = ((usize, usize), (usize, usize), usize);

fn neighbors(coord: (usize, usize), prev: (usize, usize), count: usize, map: &Map) -> Vec<Step> {
    let mut neighbors: Vec<Step> = Vec::with_capacity(4);
    let curr = &map[coord.0][coord.1];

    // west: neighbor must connect east
    if let Some(col) = coord.1.checked_sub(1) {
        match curr {
            Tile::EastWest | Tile::NorthWest | Tile::SouthWest | Tile::Start => {
                if (coord.0, col) != prev {
                    match map[coord.0][col] {
                        Tile::EastWest | Tile::NorthEast | Tile::SouthEast | Tile::Start => {
                            neighbors.push(((coord.0, col), coord, count + 1))
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }

    // east: neighbor must connect west
    let col = coord.1 + 1;
    if col < map[0].len() && (coord.0, col) != prev {
        match curr {
            Tile::EastWest | Tile::NorthEast | Tile::SouthEast | Tile::Start => {
                match map[coord.0][col] {
                    Tile::EastWest | Tile::NorthWest | Tile::SouthWest | Tile::Start => {
                        neighbors.push(((coord.0, col), coord, count + 1))
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // north: neighbor must connect south
    if let Some(row) = coord.0.checked_sub(1) {
        if (row, coord.1) != prev {
            match curr {
                Tile::NorthEast | Tile::NorthSouth | Tile::NorthWest | Tile::Start => {
                    match map[row][coord.1] {
                        Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth | Tile::Start => {
                            neighbors.push(((row, coord.1), coord, count + 1))
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    // south: neighbor must connect north
    let row = coord.0 + 1;
    if row < map.len() && (row, coord.1) != prev {
        match curr {
            Tile::SouthEast | Tile::SouthWest | Tile::NorthSouth | Tile::Start => {
                match map[row][coord.1] {
                    Tile::NorthEast | Tile::NorthWest | Tile::NorthSouth | Tile::Start => {
                        neighbors.push(((row, coord.1), coord, count + 1))
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    neighbors
}

// General solution
// - Find pipe tiles
// - Find x/y boundaries
// - Find all chunks using flood fill (fill up to x/y boundary or pipe tile)
// - Pick a direction and count how many pipe tiles you cross (inc if pipe is at edge of map)

fn solve(input: &str) -> usize {
    let mut start: Option<(usize, usize)> = None;
    let map: Map = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(col, c)| {
                    let tile = Tile::new(c);
                    if matches!(tile, Tile::Start) {
                        start = Some((row, col));
                    }
                    tile
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();
    let mut frontier: VecDeque<Step> = VecDeque::new();
    frontier.extend(neighbors(start, start, 0, &map));

    while let Some((coord, prev, count)) = frontier.pop_front() {
        if let Tile::Start = map[coord.0][coord.1] {
            return count / 2;
        }
        frontier.extend(neighbors(coord, prev, count, &map));
    }

    panic!("should not reach")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn case2() {
        let input = r"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn case3() {
        let input = r".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve(input), 8);
    }

    #[test]
    fn case4() {
        let input = r"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(solve(input), 10);
    }
}

util::read_main!();
