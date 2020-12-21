use std::collections::HashMap;

#[derive(Debug)]
struct Piece{
    edges: Vec<String>, // including mirrored edges
    id: String
}

impl Piece {
    fn new(top: String, bottom: String, left: String, right: String, id: String) -> Piece {
        let mut edges = Vec::new();
        edges.push(top.clone());
        edges.push(top.chars().rev().collect());
        edges.push(bottom.clone());
        edges.push(bottom.chars().rev().collect());
        edges.push(left.clone());
        edges.push(left.chars().rev().collect());
        edges.push(right.clone());
        edges.push(right.chars().rev().collect());

        Piece{
            edges,
            id
        }
    }
}

fn solve(input: &str) -> u64 {
    // working assumption: only one possible pattern per edge
    // tiles are all 10x10

    let mut edge_to_id: HashMap<String, Vec<&str>> = HashMap::new();
    let mut pieces= Vec::new();

    for chunk in input.trim().split("\n\n") {
        let mut chunk = chunk.lines();
        // id
        let id = &chunk.next().unwrap()[5..9];

        let chunk: Vec<&str> = chunk.collect();

        // add all edges and mirror images to map
        // top
        let top = chunk[0].to_owned();
        edge_to_id.entry(top.clone()).or_insert(vec![]).push(id);
        edge_to_id.entry(top.chars().rev().collect()).or_insert(vec![]).push(id);
        // bottom
        let bottom = chunk[chunk.len()-1].to_owned();
        edge_to_id.entry(bottom.clone()).or_insert(vec![]).push(id);
        edge_to_id.entry(bottom.chars().rev().collect()).or_insert(vec![]).push(id);
        // left
        let left: String = chunk.iter().map(|&l| l[0..1].chars().next().unwrap()).collect();
        edge_to_id.entry(left.clone()).or_insert(vec![]).push(id);
        edge_to_id.entry(left.chars().rev().collect()).or_insert(vec![]).push(id);
        // right
        let right: String = chunk.iter().map(|&l| l[9..10].chars().next().unwrap()).collect();
        edge_to_id.entry(right.clone()).or_insert(vec![]).push(id);
        edge_to_id.entry(right.chars().rev().collect()).or_insert(vec![]).push(id);

        pieces.push(Piece::new(top, bottom, left, right, id.to_string()));
    };

    // keep track of everything with only two matching edges??
    let mut corner_pieces = Vec::new();
    for piece in pieces {
        let num_corners = piece.edges.iter().filter(|&edge| edge_to_id.get(edge).unwrap().len() == 1).count();
        if num_corners == 4 {
            corner_pieces.push(piece.id.clone());
        }
    }

    assert!(corner_pieces.len() == 4);
    corner_pieces.iter().map(|id| id.parse::<u64>().unwrap()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
        assert_eq!(solve(input), 20899048083289);
    }
}

advent_2020::read_main!();
