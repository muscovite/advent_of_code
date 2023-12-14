use std::collections::HashMap;

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
enum Tile {
    Round,
    Square,
    Floor,
}

impl Tile {
    fn new(c: char) -> Tile {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Square,
            '.' => Tile::Floor,
            _ => panic!("invalid tile"),
        }
    }
}

// Shortened version; could update the rest to match
fn tilt_north(platform: &mut Vec<Vec<Tile>>) {
    let len = platform.len();
    let coords = (1..len).flat_map(|y| (0..len).map(move |x| (x, y)));

    for (x, y) in coords {
        let Tile::Round = platform[y][x] else {
            continue;
        };
        for i in (0..y).rev() {
            match platform[i][x] {
                Tile::Square | Tile::Round => {
                    platform[i + 1][x] = Tile::Round;
                    if i + 1 != y {
                        platform[y][x] = Tile::Floor;
                    }
                    break;
                }
                _ => {
                    // reached edge
                    if i == 0 {
                        platform[0][x] = Tile::Round;
                        platform[y][x] = Tile::Floor;
                    }
                }
            }
        }
    }
}

fn tilt_south(platform: &mut Vec<Vec<Tile>>) {
    for y in (0..platform.len() - 1).rev() {
        for x in 0..platform[0].len() {
            match platform[y][x] {
                Tile::Round => {
                    // find northmost valid unoccupied space
                    for i in y + 1..platform.len() {
                        match platform[i][x] {
                            Tile::Square | Tile::Round => {
                                platform[i - 1][x] = Tile::Round;
                                if i - 1 != y {
                                    platform[y][x] = Tile::Floor;
                                }
                                break;
                            }
                            _ => {
                                // reached edge
                                if i == platform.len() - 1 {
                                    let len = platform.len();
                                    platform[len - 1][x] = Tile::Round;
                                    platform[y][x] = Tile::Floor;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn tilt_west(platform: &mut Vec<Vec<Tile>>) {
    for y in 0..platform.len() {
        for x in 1..platform[0].len() {
            match platform[y][x] {
                Tile::Round => {
                    for i in (0..x).rev() {
                        match platform[y][i] {
                            Tile::Square | Tile::Round => {
                                platform[y][i + 1] = Tile::Round;
                                if i + 1 != x {
                                    platform[y][x] = Tile::Floor;
                                }
                                break;
                            }
                            _ => {
                                // reached edge
                                if i == 0 {
                                    platform[y][0] = Tile::Round;
                                    platform[y][x] = Tile::Floor;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn tilt_east(platform: &mut Vec<Vec<Tile>>) {
    for y in 0..platform.len() {
        for x in (0..platform[0].len() - 1).rev() {
            match platform[y][x] {
                Tile::Round => {
                    for i in x + 1..platform[0].len() {
                        match platform[y][i] {
                            Tile::Square | Tile::Round => {
                                platform[y][i - 1] = Tile::Round;
                                if i - 1 != x {
                                    platform[y][x] = Tile::Floor;
                                }
                                break;
                            }
                            _ => {
                                // reached edge
                                if i == platform[0].len() - 1 {
                                    let len = platform[0].len();
                                    platform[y][len - 1] = Tile::Round;
                                    platform[y][x] = Tile::Floor;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
}

fn get_load(platform: &Vec<Vec<Tile>>) -> usize {
    platform
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|t| **t == Tile::Round).count() * (idx + 1))
        .sum()
}

fn solve(input: &str) -> usize {
    let mut platform: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect();

    for i in 0..1000000000 {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);
        let load = get_load(&platform);
        dbg!(i, load);
        if i == 1000 {
            break;
        }
        // at this point, I just looked at the output for cycles
        // the answer is...
        // let idx = starting index of first cycle
        // let diff = 1000000000 - (cycle_length * floor(1000000000 / cycle_length))
        // calculate offset based on
        //      diff - floor(diff/idx)
        //      idx - floor(idx/diff)
        //   depending which is larger
        // Answer = load @ idx + (offset - 1) (The offset-th load, counting from start index)
    }
    panic!("should no longer reach?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(input), 64);
    }
}

util::read_main!();
