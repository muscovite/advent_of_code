use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::iter::FromIterator;

lazy_static! {
    static ref NOTE_RE: Regex = Regex::new(
        r"^(?P<field>[ \w]+): (?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)$"
    )
    .unwrap();
}

fn solve(input: &str) -> usize {
    let mut fields = std::iter::repeat(vec![]).take(1000).collect::<Vec<_>>();
    let mut input = input.trim().split("\n\n");

    // notes
    let notes = input.next().unwrap();
    for note in notes.lines() {
        let caps = NOTE_RE.captures(note).unwrap();
        let field_name = caps.name("field").unwrap().as_str();

        for (l, u) in [("lower1", "upper1"), ("lower2", "upper2")].iter() {
            let lower: usize = caps.name(l).unwrap().as_str().parse().unwrap();
            let upper: usize = caps.name(u).unwrap().as_str().parse().unwrap();
            for i in lower..=upper {
                fields[i].push(field_name);
            }
        }
    }

    // your ticket
    let my_ticket: Vec<usize> = input
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut field_pos: Vec<HashSet<&str>> = vec![HashSet::new(); my_ticket.len()];

    // other tickets
    let mut others = input.next().unwrap().lines();
    // skip "nearby tickets:"
    others.next();
    // filter out invalid tickets
    let valid = others.filter(|o| {
        for num in o.split(',') {
            let n: usize = num.parse().unwrap();
            if fields[n].len() == 0 {
                return false;
            }
        }
        true
    });

    // for each position in ticket, possible fields are intersection of
    // field names?
    for ticket in valid {
        for (i, value) in ticket.split(",").enumerate() {
            let options = &fields[value.parse::<usize>().unwrap()];
            if field_pos[i].len() == 0 {
                field_pos[i].extend(options);
            } else {
                field_pos[i] = &field_pos[i] & &HashSet::from_iter(options.iter().cloned());
            }
        }
    }

    // order possibilities from lowest to highest
    // there should be only one set with one possibility, which can then be
    // removed from each successive set to leave just one possibility, etc
    let mut field_pos = field_pos.iter().enumerate().collect::<Vec<(_, _)>>();
    field_pos.sort_by_key(|(_, o)| o.len());

    let mut used = HashSet::new();
    field_pos
        .iter()
        .filter_map(|&(i, o)| {
            let diff = o - &used;
            // there should only be one!
            let name = diff.iter().next().unwrap();
            used = &used | o;
            if name.starts_with("departure") {
                return Some(my_ticket[i]);
            }
            None
        })
        .product()
}

advent_2020::read_main!();
