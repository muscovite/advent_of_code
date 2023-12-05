use std::ops::Range;

fn solve(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(":").unwrap();
    let mut seeds: Vec<usize> = seeds
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    for map in maps.split("\n\n") {
        // strip first line
        // make ranges
        let ranges: Vec<(Range<usize>, usize)> = map
            .lines()
            .skip(1)
            .map(|l| {
                let mut l = l.splitn(3, " ");
                let dst: usize = l.next().unwrap().parse().unwrap();
                let src: usize = l.next().unwrap().parse().unwrap();
                let count: usize = l.next().unwrap().parse().unwrap();

                (src..src + count, dst)
            })
            .collect();

        // apply ranges to seeds
        for idx in 0..seeds.len() {
            let seed = seeds[idx];
            for (src_range, dst) in ranges.iter() {
                if src_range.contains(&seed) {
                    seeds[idx] = *dst + (seed - src_range.start);
                    break;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(solve(input), 35);
    }
}

util::read_main!();
