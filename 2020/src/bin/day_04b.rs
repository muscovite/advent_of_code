use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref HGT_RE: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>cm|in)$").unwrap();
}

lazy_static! {
    static ref HCL_RE: Regex = Regex::new(r"^#([a-f]|\d){6}$").unwrap();
}

lazy_static! {
    static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
}

lazy_static! {
    static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

fn solve(input: &str) -> usize {
    fn byr_validate(val: &str) -> bool {
        if val.len() != 4 {
            return false;
        }
        let year: u32 = val.parse().unwrap();
        year >= 1920 && year <= 2002
    }
    fn iyr_validate(val: &str) -> bool {
        if val.len() != 4 {
            return false;
        }
        let year: u32 = val.parse().unwrap();
        year >= 2010 && year <= 2020
    }
    fn eyr_validate(val: &str) -> bool {
        if val.len() != 4 {
            return false;
        }
        let year: u32 = val.parse().unwrap();
        year >= 2020 && year <= 2030
    }
    fn hgt_validate(val: &str) -> bool {
        if !HGT_RE.is_match(val) {
            return false;
        }
        let captures = HGT_RE.captures(val).unwrap();
        let num: u32 = captures.name("num").unwrap().as_str().parse().unwrap();
        match captures.name("unit").unwrap().as_str() {
            "cm" => return num >= 150 && num <= 193,
            "in" => return num >= 59 && num <= 76,
            _ => return false,
        }
    }
    fn hcl_validate(val: &str) -> bool {
        HCL_RE.is_match(val)
    }
    fn ecl_validate(val: &str) -> bool {
        ECL_RE.is_match(val)
    }
    fn pid_validate(val: &str) -> bool {
        PID_RE.is_match(val)
    }
    fn cid_validate(_: &str) -> bool {
        true
    }

    // could probably use something lighter than a hash set
    let validators: [(&str, fn(&str) -> bool); 8] = [
        ("byr", byr_validate),
        ("iyr", iyr_validate),
        ("eyr", eyr_validate),
        ("hgt", hgt_validate),
        ("hcl", hcl_validate),
        ("ecl", ecl_validate),
        ("pid", pid_validate),
        ("cid", cid_validate), // optional
    ];

    input
        .trim()
        .split("\n\n")
        .filter(|l| {
            let tokens: Vec<_> = l.split_whitespace().collect();
            if tokens.len() < 7 {
                return false;
            }
            let mut validator_map = validators
                .iter()
                .map(|&(key, func)| (key, func))
                .collect::<HashMap<_, _>>();

            for token in tokens {
                let mut parts = token.split(":");
                let key = parts.next().unwrap();
                let val = parts.next().unwrap();
                match validator_map.remove(key) {
                    Some(validator) => {
                        if !validator(val) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
            validator_map.len() == 0
                || (validator_map.len() == 1 && validator_map.contains_key("cid"))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_invalid() {
        let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn all_valid() {
        let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve(input), 4);
    }
}

util::read_main!();
