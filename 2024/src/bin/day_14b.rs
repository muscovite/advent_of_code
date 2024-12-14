use std::collections::HashSet;

use std::{thread, time::Duration};

use itertools::Itertools;

fn do_move(
    starts: &Vec<((isize, isize), (isize, isize))>,
    steps: isize,
    width: isize,
    height: isize,
) {
    let new_positions: HashSet<_> = starts
        .iter()
        .map(|((x_pos, y_pos), (v_x, v_y))| {
            (
                (x_pos + (v_x * steps)).rem_euclid(width),
                (y_pos + (v_y * steps)).rem_euclid(height),
            )
        })
        .collect();

    // check for, say, 10 robots in a row
    if new_positions
        .iter()
        .into_group_map_by(|(x, _)| x)
        .into_iter()
        .filter(|(_, vals)| vals.len() >= 10)
        .find(|(_, vals)| {
            let min = vals.iter().min().unwrap();
            let max = vals.iter().max().unwrap();
            max.1 - min.1 + 1 == vals.len() as isize
        })
        == None
    {
        return;
    }
    dbg!(steps);

    for x in 0..width {
        for y in 0..height {
            if new_positions.contains(&(x, y)) {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    thread::sleep(Duration::from_secs(1));
}

fn solve(input: &str) -> usize {
    let starts: Vec<_> = input
        .lines()
        .map(|l| {
            let (robot, velocity) = l.split_once(" ").unwrap();

            let (robot_x, robot_y) = robot.strip_prefix("p=").unwrap().split_once(",").unwrap();
            let (robot_x, robot_y) = (
                robot_x.parse::<isize>().unwrap(),
                robot_y.parse::<isize>().unwrap(),
            );

            let (v_x, v_y) = velocity
                .strip_prefix("v=")
                .unwrap()
                .split_once(",")
                .unwrap();
            let (v_x, v_y) = (v_x.parse::<isize>().unwrap(), v_y.parse::<isize>().unwrap());
            ((robot_x, robot_y), (v_x, v_y))
        })
        .collect();

    for steps in 1.. {
        do_move(&starts, steps as isize, 101, 103);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
}

util::read_main!();
