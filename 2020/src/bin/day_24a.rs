use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Tile {
    x: i64,
    y: i64,
}

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

    let mut flipped: HashMap<Tile, bool> = HashMap::new(); // black = true
    input.trim().lines().for_each(|input| {
        flipped
            .entry(next_flip(input))
            .and_modify(|is_black| *is_black = !(*is_black))
            .or_insert(true);
    });

    flipped.iter().filter(|(_, &is_black)| is_black).count()
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
        assert_eq!(solve(input), 10);
    }
}

util::read_main!();
