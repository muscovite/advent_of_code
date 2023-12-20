use std::{collections::HashMap, ops::Range};

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Action {
    Workflow(String),
    Accept,
    Reject,
}

impl Action {
    fn new(inst: &str) -> Action {
        match inst {
            "A" => Action::Accept,
            "R" => Action::Reject,
            name => Action::Workflow(name.to_owned()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Op {
    LessThan(u64),
    GreaterThan(u64),
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn new(field: &str) -> Field {
        match field {
            "x" => Field::X,
            "m" => Field::M,
            "a" => Field::A,
            "s" => Field::S,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Compare {
    field: Field,
    operator: Op, // > or <
    action: Action,
}

impl Compare {
    fn new(comparison: &str, action: &str) -> Instruction {
        let (field, op) = match comparison.split_once(">") {
            Some((part, operand)) => (
                Field::new(part),
                Op::GreaterThan(operand.parse::<u64>().unwrap()),
            ),
            _ => {
                let (part, operand) = comparison.split_once("<").unwrap();
                (
                    Field::new(part),
                    Op::LessThan(operand.parse::<u64>().unwrap()),
                )
            }
        };

        Instruction::Compare(Compare {
            field: field,
            operator: op,
            action: Action::new(action),
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
enum Instruction {
    Compare(Compare),
    Action(Action),
}

impl Instruction {
    fn new(inst: &str) -> Instruction {
        match inst.split_once(":") {
            Some((comparison, action)) => Compare::new(comparison, action),
            _ => Instruction::Action(Action::new(inst)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Possibility {
    next: String,
    ratings: [Range<u64>; 4],
}

fn run(workflows: &HashMap<&str, Vec<Instruction>>, p: Possibility) -> (Vec<Possibility>, u64) {
    let mut subtotal = 0;
    let mut possibilities = vec![];

    let mut curr_ratings = p.ratings;

    for inst in workflows.get(p.next.as_str()).unwrap() {
        match inst {
            Instruction::Compare(c) => {
                let (true_range, false_range) = match c.operator {
                    Op::LessThan(val) => match c.field {
                        Field::X => (curr_ratings[0].start..val, val..curr_ratings[0].end),
                        Field::M => (curr_ratings[1].start..val, val..curr_ratings[1].end),
                        Field::A => (curr_ratings[2].start..val, val..curr_ratings[2].end),
                        Field::S => (curr_ratings[3].start..val, val..curr_ratings[3].end),
                    },
                    Op::GreaterThan(val) => match c.field {
                        Field::X => (val + 1..curr_ratings[0].end, curr_ratings[0].start..val + 1),
                        Field::M => (val + 1..curr_ratings[1].end, curr_ratings[1].start..val + 1),
                        Field::A => (val + 1..curr_ratings[2].end, curr_ratings[2].start..val + 1),
                        Field::S => (val + 1..curr_ratings[3].end, curr_ratings[3].start..val + 1),
                    },
                };

                if true_range.end > true_range.start {
                    let new_ratings = match c.field {
                        Field::X => [
                            true_range,
                            curr_ratings[1].clone(),
                            curr_ratings[2].clone(),
                            curr_ratings[3].clone(),
                        ],
                        Field::M => [
                            curr_ratings[0].clone(),
                            true_range,
                            curr_ratings[2].clone(),
                            curr_ratings[3].clone(),
                        ],
                        Field::A => [
                            curr_ratings[0].clone(),
                            curr_ratings[1].clone(),
                            true_range,
                            curr_ratings[3].clone(),
                        ],
                        Field::S => [
                            curr_ratings[0].clone(),
                            curr_ratings[1].clone(),
                            curr_ratings[2].clone(),
                            true_range,
                        ],
                    };
                    match &c.action {
                        Action::Workflow(name) => {
                            possibilities.push(Possibility {
                                next: name.clone(),
                                ratings: new_ratings,
                            });
                        }
                        Action::Accept => {
                            subtotal +=
                                new_ratings.iter().map(|r| r.end - r.start).product::<u64>();
                        }
                        Action::Reject => (),
                    }
                }

                if false_range.end > false_range.start {
                    match c.field {
                        Field::X => curr_ratings[0] = false_range,
                        Field::M => curr_ratings[1] = false_range,
                        Field::A => curr_ratings[2] = false_range,
                        Field::S => curr_ratings[3] = false_range,
                    };
                } else {
                    break;
                }
            }
            Instruction::Action(a) => match a {
                Action::Workflow(name) => {
                    possibilities.push(Possibility {
                        next: name.clone(),
                        ratings: curr_ratings.clone(),
                    });
                }
                Action::Accept => {
                    subtotal += curr_ratings
                        .iter()
                        .map(|r| r.end - r.start)
                        .product::<u64>()
                }
                Action::Reject => (),
            },
        }
    }
    (possibilities, subtotal)
}

fn solve(input: &str) -> u64 {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<&str, Vec<Instruction>> = workflows
        .lines()
        .map(|l| {
            let l = l.strip_suffix("}").unwrap();
            let (name, instructions) = l.split_once("{").unwrap();

            let instructions = instructions
                .split(",")
                .map(|inst| Instruction::new(inst))
                .collect();

            (name, instructions)
        })
        .collect();

    let mut total = 0;
    let mut possibilities: Vec<Possibility> = vec![Possibility {
        next: "in".to_owned(),
        ratings: [1..4001, 1..4001, 1..4001, 1..4001],
    }];
    while let Some(possibility) = possibilities.pop() {
        let (p, subtotal) = run(&workflows, possibility);
        possibilities.extend(p);
        total += subtotal;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(solve(input), 167409079868000);
    }
}

util::read_main!();
