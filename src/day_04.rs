use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!("assets/day_04_input.txt");

fn passport_from_str(passport_str: &str) -> HashMap<&str, &str> {
    passport_str
        .trim()
        .split_whitespace()
        .filter_map(|key_value_pair| {
            let tokens: Vec<&str> = key_value_pair.split(':').collect();
            match tokens.as_slice() {
                [key, value] => Some((*key, *value)),
                _ => None,
            }
        })
        .collect()
}

fn has_required_fields(
    passport_fields: HashMap<&str, &str>,
    required_fields: &HashSet<&'static str>,
) -> bool {
    required_fields
        .iter()
        .all(|field| passport_fields.get(field).is_some())
}

fn count_valid_p1(passports_str: &'static str) -> usize {
    let required_fields = hashset! { "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" };
    let passports = passports_str.split("\n\n");
    passports
        .filter(|passport| has_required_fields(passport_from_str(passport), &required_fields))
        .count()
}

fn validate_fields(passport_fields: HashMap<&str, &str>) -> bool {
    vec![
        passport_fields.get("byr").map(|value| {
            let value: i32 = value.parse().unwrap();
            value >= 1920 && value <= 2002
        }),
        passport_fields.get("iyr").map(|value| {
            let value: i32 = value.parse().unwrap();
            value >= 2010 && value <= 2020
        }),
        passport_fields.get("eyr").map(|value| {
            let value: i32 = value.parse().unwrap();
            value >= 2020 && value <= 2030
        }),
        passport_fields.get("hgt").map(|value| {
            if value.len() <= 2 {
                return false;
            }
            let (num, unit) = value.split_at(value.len() - 2);
            match unit {
                "cm" => {
                    let height: i32 = num.parse().unwrap_or(-1);
                    height >= 150 && height <= 193
                }
                "in" => {
                    let height: i32 = num.parse().unwrap_or(-1);
                    height >= 59 && height <= 76
                }
                _ => false,
            }
        }),
        passport_fields
            .get("hcl")
            .map(|value| value.len() == 7 && i64::from_str_radix(&value[1..], 16).is_ok()),
        passport_fields.get("ecl").map(|&value| {
            value == "amb"
                || value == "blu"
                || value == "brn"
                || value == "gry"
                || value == "grn"
                || value == "hzl"
                || value == "oth"
        }),
        passport_fields
            .get("pid")
            .map(|value| value.len() == 9 && value.parse::<i32>().is_ok()),
    ]
    .iter()
    .all(|&is_valid| is_valid == Some(true))
}

fn count_valid_p2(passports_str: &'static str) -> usize {
    let passports = passports_str.split("\n\n");
    passports
        .filter(|passport| validate_fields(passport_from_str(passport)))
        .count()
}

pub fn p1() -> usize {
    count_valid_p1(INPUT)
}

pub fn p2() -> usize {
    count_valid_p2(INPUT)
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_P1: &'static str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    static EXAMPLE_P2_INVALID: &'static str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    static EXAMPLE_P2_VALID: &'static str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn p1_example() {
        assert_eq!(2, count_valid_p1(EXAMPLE_P1));
    }

    #[test]
    fn p1_correct_answer() {
        assert_eq!(182, count_valid_p1(INPUT));
    }

    #[test]
    fn p2_example() {
        assert_eq!(0, count_valid_p2(EXAMPLE_P2_INVALID));
        assert_eq!(4, count_valid_p2(EXAMPLE_P2_VALID));
    }

    // #[test]
    // fn p2_correct_answer() {}
}
