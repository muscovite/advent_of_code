use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Space {
    Rock,
    Robot,
    Empty,
    Wall,
}

impl Space {
    fn new(c: char) -> Space {
        match c {
            '@' => Space::Robot,
            '.' => Space::Empty,
            'O' => Space::Rock,
            '#' => Space::Wall,
            _ => unreachable!("unexpected space type"),
        }
    }
}

fn solve(input: &str) -> isize {
    let (grid, steps) = input.split_once("\n\n").unwrap();

    let max_x = grid.lines().next().unwrap().len();
    let max_y = grid.lines().count();

    let mut robot = None;
    let mut grid: HashMap<(isize, isize), Space> = grid
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), Space::new(c)))
        })
        .inspect(|(coord, space)| {
            if *space == Space::Robot {
                robot = Some(*coord);
            }
        })
        .collect();

    let mut robot = robot.unwrap();
    let steps = steps.trim().chars().filter(|c| *c != '\n');

    for step in steps {
        let (dx, dy) = match step {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => unreachable!("invalid step"),
        };

        let next_coord = (robot.0 + dx, robot.1 + dy);
        match grid.get(&next_coord).unwrap() {
            Space::Wall => (),
            Space::Empty | Space::Robot => robot = next_coord,
            Space::Rock => {
                let mut space_after_rock = (next_coord.0 + dx, next_coord.1 + dy);
                while Some(&Space::Rock) == grid.get(&space_after_rock) {
                    space_after_rock = (space_after_rock.0 + dx, space_after_rock.1 + dy);
                }

                if let Some(Space::Empty) = grid.get(&space_after_rock) {
                    *grid.get_mut(&space_after_rock).unwrap() = Space::Rock;
                    *grid.get_mut(&next_coord).unwrap() = Space::Empty;
                    *grid.get_mut(&(robot.0, robot.1)).unwrap() = Space::Empty;
                    robot = next_coord;
                };
            }
        }
    }

    // for x in 0..max_x as usize {
    //     for y in 0..max_y as usize {
    //         match grid.get(&(x as isize, y as isize)).unwrap() {
    //             &Space::Empty => print!("."),
    //             &Space::Robot => print!("@"),
    //             &Space::Rock => print!("O"),
    //             &Space::Wall => print!("#"),
    //         }
    //     }
    //     println!();
    // }

    grid.iter()
        .filter_map(|((x, y), v)| {
            if *v == Space::Rock {
                return Some(x + 100 * y);
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(solve(input), 2028);
    }

    #[test]
    fn case2() {
        let input = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve(input), 10092);
    }
}

util::read_main!();
