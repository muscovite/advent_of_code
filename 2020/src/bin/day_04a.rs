use std::collections::HashSet;

fn solve(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .filter(|l| {
            let tokens: Vec<_> = l.split_whitespace().collect();
            if tokens.len() < 7 {
                return false;
            }

            // could probably use something lighter than a hash set
            let mut keys = HashSet::new();
            keys.insert("byr");
            keys.insert("iyr");
            keys.insert("eyr");
            keys.insert("hgt");
            keys.insert("hcl");
            keys.insert("ecl");
            keys.insert("pid");
            keys.insert("cid"); // optional

            for mut token in tokens {
                token = token.split(":").next().unwrap();
                if !keys.remove(token) {
                    return false;
                }
            }
            keys.len() == 0 || (keys.len() == 1 && keys.contains("cid"))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(solve(input), 2);
    }
}

util::read_main!();
