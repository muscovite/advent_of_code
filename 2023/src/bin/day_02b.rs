#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Hand(u32, Color);

fn solve(input: &str) -> u32 {
    input
        .trim()
        .lines()
        // Get sets
        .map(|l| l.split(":").last().unwrap().split("; ").collect::<Vec<_>>())
        // Parse each set
        .enumerate()
        .map(|(idx, sets)| {
            // Parse each color within each set
            let draws: Vec<_> = sets
                .iter()
                .flat_map(|draw| {
                    draw.split(", ").map(|elem| {
                        let (count, color) = elem.trim().split_once(" ").unwrap();
                        let count = count.to_owned().parse::<u32>().unwrap();
                        let color = match color {
                            "blue" => Color::Blue,
                            "green" => Color::Green,
                            "red" => Color::Red,
                            _ => panic!("unknown color"),
                        };
                        Hand(count, color)
                    })
                })
                .collect();

            draws
                .iter()
                .filter(|&hand| hand.1 == Color::Blue)
                .max_by_key(|hand| hand.0)
                .unwrap()
                .0
                * draws
                    .iter()
                    .filter(|&hand| hand.1 == Color::Green)
                    .max_by_key(|hand| hand.0)
                    .unwrap()
                    .0
                * draws
                    .iter()
                    .filter(|&hand| hand.1 == Color::Red)
                    .max_by_key(|hand| hand.0)
                    .unwrap()
                    .0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), 2286);
    }
}

util::read_main!();
