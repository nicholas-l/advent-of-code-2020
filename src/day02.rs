use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<min>\d{1,})-(?P<max>\d{1,}) (?P<letter>\w): (?P<password>\w+)").unwrap();
}

fn process_passwords(input: &str) -> impl Iterator<Item = (usize, usize, char, &str)> {
    input.lines().map(|line| {
        let captures = RE
            .captures(line)
            .expect("Bad line that does not match regex.");
        let letter = captures["letter"].as_bytes()[0] as char;
        let password = captures.get(4).unwrap().as_str();
        let max = captures["max"].parse().unwrap();
        let min = captures["min"].parse().unwrap();
        (min, max, letter, password)
    })
}

#[allow(dead_code)]
pub fn star_one(input: &str) -> usize {
    process_passwords(input)
        .filter(|(min, max, letter, password)| {
            let mut found_letters = 0;
            for x in password.chars() {
                if x == *letter {
                    found_letters += 1;
                }
                if found_letters > *max {
                    return false;
                }
            }
            found_letters >= *min
        })
        .count()
}

#[allow(dead_code)]
pub fn star_two(input: &str) -> usize {
    process_passwords(input)
        .filter(|(min, max, letter, password)| {
            let chars: Vec<char> = password.chars().collect();
            if *min > chars.len() || *max > chars.len() {
                panic!(format!("Bad line: {}", password));
            }
            (chars[*min - 1] == *letter) ^ (chars[*max - 1] == *letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}
