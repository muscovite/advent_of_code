use std::collections::HashMap;

use regex::Regex;

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
    LessThan(u32),
    GreaterThan(u32),
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
                Op::GreaterThan(operand.parse::<u32>().unwrap()),
            ),
            _ => {
                let (part, operand) = comparison.split_once("<").unwrap();
                (
                    Field::new(part),
                    Op::LessThan(operand.parse::<u32>().unwrap()),
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

fn run(workflows: &HashMap<&str, Vec<Instruction>>, ratings: Vec<u32>) -> u32 {
    let mut curr = workflows.get("in").unwrap();
    loop {
        for inst in curr {
            match inst {
                Instruction::Compare(c) => {
                    let passed = match c.operator {
                        Op::LessThan(val) => match c.field {
                            Field::X => ratings[0] < val,
                            Field::M => ratings[1] < val,
                            Field::A => ratings[2] < val,
                            Field::S => ratings[3] < val,
                        },
                        Op::GreaterThan(val) => match c.field {
                            Field::X => ratings[0] > val,
                            Field::M => ratings[1] > val,
                            Field::A => ratings[2] > val,
                            Field::S => ratings[3] > val,
                        },
                    };

                    if !passed {
                        continue;
                    }

                    match &c.action {
                        Action::Workflow(name) => {
                            curr = workflows.get(name.as_str()).unwrap();
                            break;
                        }
                        Action::Accept => return ratings.into_iter().sum(),
                        Action::Reject => return 0,
                    }
                }
                Instruction::Action(a) => match a {
                    Action::Workflow(name) => {
                        curr = workflows.get(name.as_str()).unwrap();
                        break;
                    }
                    Action::Accept => return ratings.into_iter().sum(),
                    Action::Reject => return 0,
                },
            }
        }
    }
}

fn solve(input: &str) -> u32 {
    let part_re =
        Regex::new(r"x=(?P<x>[0-9]*),m=(?P<m>[0-9]*),a=(?P<a>[0-9]*),s=(?P<s>[0-9]*)").unwrap();

    let (workflows, parts) = input.split_once("\n\n").unwrap();

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

    // process parts
    parts
        .lines()
        .map(|p| {
            let (_, ratings): (_, [&str; 4]) = part_re.captures(p).unwrap().extract();
            let ratings: Vec<u32> = ratings.iter().map(|r| r.parse::<u32>().unwrap()).collect();
            run(&workflows, ratings)
        })
        .sum()
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
        assert_eq!(solve(input), 19114);
    }
}

util::read_main!();
