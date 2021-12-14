use std::collections::HashMap;

fn solve(input: &str) -> u64 {
    let (template, rules) = input.trim().split_once("\n\n").unwrap();

    let last_char = template.chars().last().unwrap();
    let mut template: HashMap<(char, char), u64> =
        template
            .as_bytes()
            .windows(2)
            .fold(HashMap::new(), |mut map, window| {
                *map.entry((window[0] as char, window[1] as char))
                    .or_default() += 1;
                map
            });
    let rules: HashMap<_, _> = rules
        .lines()
        .map(|l| {
            let elems: Vec<char> = l.chars().collect();
            ((elems[0], elems[1]), elems[6])
        })
        .collect();

    // Part A: for _ in 0..10 {
    for _ in 0..40 {
        template = template
            .into_iter()
            .fold(HashMap::new(), |mut map, (k, v)| {
                if let Some(&new_val) = rules.get(&k) {
                    *map.entry((k.0, new_val)).or_default() += v;
                    *map.entry((new_val, k.1)).or_default() += v;
                } else {
                    *map.entry(k).or_default() += v;
                }
                map
            });
    }

    let mut final_counts =
        template
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<char, u64>, (k, &v)| {
                *map.entry(k.0).or_default() += v;
                map
            });

    *final_counts.entry(last_char).or_default() += 1;
    final_counts.values().max().unwrap() - final_counts.values().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        assert_eq!(solve(input), 2188189693529);
    }
}

util::read_main!();
