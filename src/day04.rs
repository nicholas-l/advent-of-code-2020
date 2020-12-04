use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::io::BufRead;
use std::str::FromStr;

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(?P<value>\d{1,})(?P<unit>cm|in)").unwrap();
    static ref COLOUR: Regex = Regex::new(r"^#(?P<value>[0-9a-f]{6})$").unwrap();
    static ref EYE_COLOUR: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
}

fn is_seperator(c: char) -> bool {
    c == ' ' || c == '\n'
}

// struct Passport {
//     birth_year: BirthYear,
// }

// impl FromStr for Passport {
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match key {
//             "byr" => {
//                 value.len() == 4
//                     && value
//                         .parse::<usize>()
//                         .map(|x| x >= 1920 && x <= 2002)
//                         .unwrap_or(false)
//             }
//             "iyr" => {
//                 value.len() == 4
//                     && value
//                         .parse::<usize>()
//                         .map(|x| x >= 2010 && x <= 2020)
//                         .unwrap_or(false)
//             }
//             "eyr" => {
//                 value.len() == 4
//                     && value
//                         .parse::<usize>()
//                         .map(|x| x >= 2020 && x <= 2030)
//                         .unwrap_or(false)
//             }
//             "hgt" => HEIGHT_REGEX
//                 .captures(&value)
//                 .map(|captures| {
//                     let value = captures
//                         .name("value")
//                         .expect("Unable to get value")
//                         .as_str()
//                         .parse::<usize>()
//                         .expect("Unable to parse value");
//                     let unit = captures
//                         .name("unit")
//                         .expect("Unable to parse unit")
//                         .as_str();

//                     match unit {
//                         "cm" => value >= 150 && value <= 193,
//                         "in" => value >= 59 && value <= 76,
//                         _ => {
//                             let message = format!("{} {}", value, unit);
//                             panic!(message)
//                         }
//                     }
//                 })
//                 .unwrap_or(false),
//             "hcl" => {
//                 // # followed by exactly six characters 0-9 or a-f.
//                 COLOUR.is_match(value)
//             }
//             "ecl" => {
//                 // exactly one of: amb blu brn gry grn hzl oth.
//                 EYE_COLOUR.is_match(value)
//             }
//             "pid" => {
//                 // a nine-digit number, including leading zeroes
//                 value.len() == 9 && value.chars().all(char::is_numeric)
//             }
//             "cid" => true,
//             _ => panic!("Invalid key"),
//         };
//     }
// }

const valid_sections: [&str; 7] = [
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
           // "cid", // (Country ID)
];

fn is_valid(passport: &str) -> bool {
    let keys: HashSet<&str> = passport
        .split(is_seperator)
        .map(|section| section.split(':').next().unwrap())
        .collect();
    valid_sections.iter().all(|s| keys.contains(s))
}

#[allow(dead_code, unused_variables)]
pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str);
    input_str
        .split("\n\n")
        .filter(|&passport| {
            let keys: HashSet<&str> = passport
                .split(is_seperator)
                .map(|section| section.split(':').next().unwrap())
                .collect();
            valid_sections.iter().all(|s| keys.contains(s))
        })
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str);
    let is_valid = |(&key, value): (&&str, &&str)| match key {
        "byr" => {
            value.len() == 4
                && value
                    .parse::<usize>()
                    .map(|x| x >= 1920 && x <= 2002)
                    .unwrap_or(false)
        }
        "iyr" => {
            value.len() == 4
                && value
                    .parse::<usize>()
                    .map(|x| x >= 2010 && x <= 2020)
                    .unwrap_or(false)
        }
        "eyr" => {
            value.len() == 4
                && value
                    .parse::<usize>()
                    .map(|x| x >= 2020 && x <= 2030)
                    .unwrap_or(false)
        }
        "hgt" => HEIGHT_REGEX
            .captures(&value)
            .map(|captures| {
                let value = captures
                    .name("value")
                    .expect("Unable to get value")
                    .as_str()
                    .parse::<usize>()
                    .expect("Unable to parse value");
                let unit = captures
                    .name("unit")
                    .expect("Unable to parse unit")
                    .as_str();

                match unit {
                    "cm" => value >= 150 && value <= 193,
                    "in" => value >= 59 && value <= 76,
                    _ => {
                        let message = format!("{} {}", value, unit);
                        panic!(message)
                    }
                }
            })
            .unwrap_or(false),
        "hcl" => {
            // # followed by exactly six characters 0-9 or a-f.
            COLOUR.is_match(value)
        }
        "ecl" => {
            // exactly one of: amb blu brn gry grn hzl oth.
            EYE_COLOUR.is_match(value)
        }
        "pid" => {
            // a nine-digit number, including leading zeroes
            value.len() == 9 && value.chars().all(char::is_numeric)
        }
        "cid" => true,
        _ => panic!("Invalid key"),
    };
    input_str
        .split("\n\n")
        .filter(|&passport| {
            let keys: HashMap<&str, &str> = passport
                .split(is_seperator)
                .inspect(|x| println!("{}", x))
                .map(|section| {
                    let mut iter = section.splitn(2, ':');
                    let first = iter.next().unwrap();

                    (first, iter.next().expect(section))
                })
                .collect();
            valid_sections.iter().all(|s| keys.contains_key(s))
                && keys
                    .iter()
                    .inspect(|x| println!("isValid: {:?}", x))
                    .all(is_valid)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 282] = b"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 2);
    }

    #[test]
    fn test_star_two_invalid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(star_two(Cursor::new(input)), 0);
    }

    #[test]
    fn test_star_two_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(star_two(Cursor::new(input)), 4);
    }
}
