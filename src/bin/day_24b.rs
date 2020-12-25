use std::collections::HashSet;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn get_neighbor(&self, x: i64, y: i64) -> Tile {
        Tile {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

const NEIGHBORS: [(i64, i64); 6] = [(1, 2), (-1, 1), (1, -1), (-1, -2), (2, 1), (-2, -1)];

fn next_flip(input: &str) -> Tile {
    let mut x = 0;
    let mut y = 0;

    let mut input = input.chars();
    while let Some(c) = input.next() {
        match c {
            'n' => match input.next() {
                Some('e') => {
                    x += 1;
                    y += 2;
                }
                Some('w') => {
                    x -= 1;
                    y += 1;
                }
                _ => {}
            },
            's' => match input.next() {
                Some('e') => {
                    x += 1;
                    y -= 1;
                }
                Some('w') => {
                    x -= 1;
                    y -= 2;
                }
                _ => {}
            },
            'e' => {
                x += 2;
                y += 1;
            }
            'w' => {
                x -= 2;
                y -= 1;
            }
            _ => {}
        }
    }

    Tile { x, y }
}

fn solve(input: &str) -> usize {
    // Essentially, remap hexagon into three flattened faces of a cube, and
    // you're moving between the middle points of each cube

    // sidenote: TIL that Rust represents HashSets internally as HashMaps with
    // an empty tuple as value, ie zero-sized value
    let mut black_tiles: HashSet<Tile> = HashSet::new(); // black = true
    input.trim().lines().for_each(|input| {
        let next_flip = next_flip(input);
        if black_tiles.contains(&next_flip) {
            black_tiles.remove(&next_flip);
        } else {
            black_tiles.insert(next_flip);
        }
    });

    // now, run the game N times
    (0..100).for_each(|_| {
        let mut new_black_tiles: HashSet<Tile> = HashSet::new(); // (Tile, bool), black = true
        let mut white_tiles = Vec::new();
        for tile in black_tiles.iter() {
            // Any black tile with zero or more than 2 black tiles immediately
            // adjacent to it is flipped to white.
            let mut black_neighbors = 0;
            for &(dx, dy) in NEIGHBORS.iter() {
                let neighbor = tile.get_neighbor(dx, dy);
                if black_tiles.contains(&neighbor) {
                    black_neighbors += 1;
                } else {
                    // accumulate all white tiles to check later
                    white_tiles.push(neighbor);
                }
            }
            if black_neighbors == 1 || black_neighbors == 2 {
                new_black_tiles.insert(tile.clone());
            }
        }

        // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
        for tile in white_tiles {
            if NEIGHBORS
                .iter()
                .filter(|&&(dx, dy)| black_tiles.contains(&tile.get_neighbor(dx, dy)))
                .count()
                == 2
            {
                new_black_tiles.insert(tile);
            }
        }
        black_tiles = new_black_tiles;
    });

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(solve(input), 2208);
    }
}

advent_2020::read_main!();
