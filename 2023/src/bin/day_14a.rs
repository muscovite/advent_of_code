#[derive(PartialEq, Debug)]
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

fn tilt_north(platform: &mut Vec<Vec<Tile>>) {
    // ignore first line, tilting won't change it
    for y in 1..platform.len() {
        for x in 0..platform[0].len() {
            match platform[y][x] {
                Tile::Round => {
                    // find northmost valid unoccupied space
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
                _ => (),
            }
        }
    }
}

fn solve(input: &str) -> usize {
    let mut platform: Vec<Vec<Tile>> = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::new(c)).collect())
        .collect();
    tilt_north(&mut platform);

    platform
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.into_iter().filter(|t| *t == Tile::Round).count() * (idx + 1))
        .sum()
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
        assert_eq!(solve(input), 136);
    }
}

util::read_main!();
