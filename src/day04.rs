use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::str::FromStr;

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(?P<value>\d{1,})(?P<unit>cm|in)").unwrap();
    static ref COLOUR: Regex = Regex::new(r"^#(?P<value>[0-9a-f]{6})$").unwrap();
    static ref EYE_COLOUR: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
}

#[derive(Debug)]
enum Unit {
    Centimeter,
    Inch,
}

impl FromStr for Unit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cm" => Ok(Unit::Centimeter),
            "in" => Ok(Unit::Inch),
            _ => Err(format!("Cannot convert {} to Unit", s)),
        }
    }
}

#[derive(Debug)]
struct Length {
    value: usize,
    unit: Unit,
}

impl Length {
    fn is_valid(&self) -> bool {
        match self.unit {
            Unit::Centimeter => self.value >= 150 && self.value <= 193,
            Unit::Inch => self.value >= 59 && self.value <= 76,
        }
    }
}

impl FromStr for Length {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HEIGHT_REGEX
            .captures(s)
            .ok_or_else(|| "Wrong format for length".to_string())
            .map(|captures| {
                Ok(Length {
                    value: captures
                        .name("value")
                        .ok_or_else(|| "Unable to get value when parsing length".to_string())
                        .and_then(|value| {
                            value.as_str().parse().map_err(|_e| {
                                format!(
                                    "Unable to parse value when parsing length for {}",
                                    value.as_str()
                                )
                            })
                        })?,
                    unit: captures
                        .name("unit")
                        .ok_or_else(|| "Unable to get unit when parsing length".to_string())
                        .and_then(|value| {
                            value.as_str().parse().map_err(|_e| {
                                format!(
                                    "Unable to parse unit when parsing length for {}",
                                    value.as_str()
                                )
                            })
                        })?,
                })
            })?
    }
}
#[derive(Debug)]
enum Colour {
    Rgb(usize, usize, usize),
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(Colour::Rgb(0, 0, 0))
    }
}

#[derive(Debug)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    expiry_year: usize,
    height: Length,
    hair_colour: String,
    eye_colour: String,
    personal_id: String,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year >= 1920
            && self.birth_year <= 2002
            && self.issue_year >= 2010
            && self.issue_year <= 2020
            && self.expiry_year >= 2020
            && self.expiry_year <= 2030
            && self.height.is_valid()
            && COLOUR.is_match(&self.hair_colour)
            && EYE_COLOUR.is_match(&self.eye_colour)
            && self.personal_id.len() == 9
            && self.personal_id.chars().all(char::is_numeric)
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hm: HashMap<&str, &str> = s
            .split_whitespace()
            .map(|section| {
                let mut iter = section.splitn(2, ':');
                let first = iter.next().unwrap();

                (first, iter.next().expect(section))
            })
            .collect();
        Ok(Passport {
            birth_year: hm
                .get("byr")
                .ok_or_else(|| "Unable to get birth year".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse birth year for {}", value))
                })?,
            issue_year: hm
                .get("iyr")
                .ok_or_else(|| "Unable to get issue year".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse issue year for {}", value))
                })?,
            expiry_year: hm
                .get("eyr")
                .ok_or_else(|| "Unable to get expiry year".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse expiry year for {}", value))
                })?,
            height: hm
                .get("hgt")
                .ok_or_else(|| "Unable to get height".to_string())
                .and_then(|value| {
                    value
                        .parse::<Length>()
                        .map_err(|e| format!("Unable to parse height for {}: {}", value, e))
                })?,
            hair_colour: hm
                .get("hcl")
                .ok_or_else(|| "Unable to get hair colour".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse hair colour for {}", value))
                })?,
            eye_colour: hm
                .get("ecl")
                .ok_or_else(|| "Unable to get eye colour".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse eye colour for {}", value))
                })?,
            personal_id: hm
                .get("pid")
                .ok_or_else(|| "Unable to get personal id".to_string())
                .and_then(|value| {
                    value
                        .parse()
                        .map_err(|_e| format!("Unable to parse personal id for {}", value))
                })?,
        })
    }
}

const VALID_SECTIONS: [&str; 7] = [
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
           // "cid", // (Country ID)
];

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    input_str
        .split("\n\n")
        .filter(|&passport| {
            let keys: HashSet<&str> = passport
                .split_whitespace()
                .map(|section| section.split(':').next().unwrap())
                .collect();
            VALID_SECTIONS.iter().all(|s| keys.contains(s))
        })
        .count()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    input_str
        .split("\n\n")
        .filter_map(|passport| passport.parse::<Passport>().ok())
        .filter(|p| p.is_valid())
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
