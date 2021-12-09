use std::collections::{HashMap, HashSet};

// Cheat sheet
//     0:      1:      2:      3:      4:
//     aaaa    ....    aaaa    aaaa    ....
//    b    c  .    c  .    c  .    c  b    c
//    b    c  .    c  .    c  .    c  b    c
//     ....    ....    dddd    dddd    dddd
//    e    f  .    f  e    .  .    f  .    f
//    e    f  .    f  e    .  .    f  .    f
//     gggg    ....    gggg    gggg    ....

//      5:      6:      7:      8:      9:
//     aaaa    aaaa    aaaa    aaaa    aaaa
//    b    .  b    .  .    c  b    c  b    c
//    b    .  b    .  .    c  b    c  b    c
//     dddd    dddd    ....    dddd    dddd
//    .    f  e    f  .    f  e    f  .    f
//    .    f  e    f  .    f  e    f  .    f
//     gggg    gggg    ....    gggg    gggg

// in order of discovery

// 1 -> len == 2
// 4 -> len == 4
// 7 -> len == 3
// 8 -> len == 7

// Top = 7 - 1
// 5 -> Contains (4 - 1) + Top and len == 5
// BottomLeft = 8 - 5 - 1
// 6 -> 5 + BottomLeft
// 9 -> 8 - Bottomleft
// TopRight = 8 - 6
// BottomRight = 1 - TopRight
// 2 -> contains Top, Top Right, and BottomLeft, and len == 5
// 3 = (2 - BottomLeft) + BottomRight
// TopLeft = 4 - 3
// 0 = (3 - 4) + TopLeft + TopRight + BottomLeft + BottomRight

fn get_output(line: &str) -> u32 {
    let mut mapping: Vec<HashSet<char>> = vec![HashSet::new(); 10];

    let (input, output) = line.split_once("|").unwrap();

    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let output: Vec<&str> = output.trim().split_whitespace().collect();

    // Easy ones first
    input.iter().for_each(|s| {
        let num = match s.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            _ => return,
        };

        mapping[num] = s.chars().collect();
    });

    let top = &mapping[7] - &mapping[1];

    mapping[5] = input
        .iter()
        .find_map(|s| {
            let char_set: HashSet<_> = s.chars().collect();
            if (char_set.is_superset(&(&(&mapping[4] - &mapping[1]) | &top))) && char_set.len() == 5
            {
                Some(char_set)
            } else {
                None
            }
        })
        .unwrap();

    let bottom_left = &(&mapping[8] - &mapping[5]) - &mapping[1];
    mapping[6] = &mapping[5] | &bottom_left;
    mapping[9] = &mapping[8] - &bottom_left;
    let top_right = &mapping[8] - &mapping[6];
    let bottom_right = &mapping[1] - &top_right;

    mapping[2] = input
        .iter()
        .find_map(|s| {
            let char_set: HashSet<_> = s.chars().collect();
            if (char_set.is_superset(&(&(&top | &top_right) | &bottom_left))) && char_set.len() == 5
            {
                Some(char_set)
            } else {
                None
            }
        })
        .unwrap();

    mapping[3] = &(&mapping[2] - &bottom_left) | &bottom_right;
    let top_left = &mapping[4] - &mapping[3];
    mapping[0] = &(&(&(&(&mapping[3] - &mapping[4]) | &top_left) | &top_right) | &bottom_left)
        | &bottom_right;

    // Reverse the mapping
    let solved_to_num_map: HashMap<Vec<char>, u32> = mapping
        .into_iter()
        .enumerate()
        .map(|(idx, char_set)| {
            let mut s: Vec<char> = char_set.into_iter().collect();
            s.sort();
            (s, idx as u32)
        })
        .collect();

    output
        .iter()
        .rev()
        .enumerate()
        .map(|(i, s)| {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();
            solved_to_num_map.get(&key).unwrap() * 10_u32.pow(i as u32)
        })
        .sum()
}

fn solve(input: &str) -> u32 {
    input.trim().lines().map(get_output).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input =
            r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(solve(input), 5353);
    }

    #[test]
    fn case2() {
        let input = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(solve(input), 61229);
    }
}

util::read_main!();
