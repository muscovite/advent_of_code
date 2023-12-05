use std::ops::Range;

fn overlaps(
    src_range: &Range<usize>,
    seed: &Range<usize>,
    dst: usize,
) -> Option<(Option<Range<usize>>, Option<Vec<Range<usize>>>)> {
    // src range contains seed
    // [......{*******}..]
    if src_range.start <= seed.start && src_range.end >= seed.end {
        let diff = seed.start - src_range.start;
        return Some((
            //overlap
            Some(Range {
                start: dst + diff,
                end: dst + diff + (seed.end - seed.start),
            }),
            //rest
            None,
        ));
    }
    // seed contains src range
    // {-----[****]-}
    else if seed.start <= src_range.start && seed.end >= src_range.end {
        //         93 94 95 96 97
        //         56 57 58 59 60
        // 91 92                  98 99
        let mut rest = vec![];

        if seed.start < src_range.start {
            // leading - values unchanged
            rest.push(Range {
                start: seed.start,
                end: src_range.start,
            });
        }

        if seed.end > src_range.end {
            // trailing - values unchanged
            rest.push(Range {
                start: src_range.end + 1,
                end: seed.end,
            });
        }

        return Some((
            //overlap
            Some(Range {
                start: dst,
                end: dst + (src_range.end - src_range.start + 1),
            }),
            //rest
            Some(rest),
        ));
    }
    // src range starts lower
    // [....{****]----}
    else if src_range.start <= seed.start
        && src_range.end > seed.start
        && seed.end > src_range.end
    {
        // overlap - map to dst
        let diff = seed.start - src_range.start;
        return Some((
            //overlap
            Some(Range {
                start: dst + diff,
                end: dst + diff + (src_range.end - seed.start) + 1,
            }),
            //rest
            Some(vec![Range {
                start: src_range.end + 1,
                end: seed.end,
            }]),
        ));
    }
    // seed range starts lower
    // {-----[******}....]
    else if seed.start <= src_range.start
        && seed.end > src_range.start
        && src_range.end > seed.end
    {
        return Some((
            //overlap
            Some(Range {
                start: dst,
                end: dst + (seed.end - src_range.start + 1),
            }),
            //rest
            Some(vec![Range {
                start: seed.start,
                end: src_range.start,
            }]),
        ));
    }

    None
}

fn solve(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(":").unwrap();
    let seeds: Vec<usize> = seeds
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect();
    let mut seeds: Vec<Range<usize>> = seeds
        .chunks(2)
        .map(|pair| {
            let src = pair[0];
            let len = pair[1];
            Range {
                start: src,
                end: src + len,
            }
        })
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

                (
                    Range {
                        start: src,
                        end: src + count,
                    },
                    dst,
                )
            })
            .collect();

        // Find matching ranges for seeds
        // seed range: {.........}
        // src range : [.........]
        // **** => value remapped
        // ---- => value unchanged
        // .... => doesn't contribute
        let mut next_seeds: Vec<Range<usize>> = Vec::with_capacity(seeds.len());
        while let Some(seed) = seeds.pop() {
            match ranges
                .iter()
                .find_map(|(src_range, dst)| overlaps(src_range, &seed, *dst))
            {
                // some range matches
                Some((overlap, rest)) => {
                    match overlap {
                        Some(r) => next_seeds.push(r),
                        None => (),
                    }
                    match rest {
                        Some(r) => seeds.extend(r),
                        None => (),
                    }
                }
                // no matching ranges
                None => next_seeds.push(seed),
            }
        }

        seeds = next_seeds;
    }

    seeds
        .into_iter()
        .min_by_key(|seed| seed.start)
        .unwrap()
        .start as usize
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
        assert_eq!(solve(input), 46);
    }
}

util::read_main!();
