use regex::Regex;
use std::io::BufRead;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<min>\d{1,})-(?P<max>\d{1,}) (?P<letter>\w): (?P<password>\w+)").unwrap();
}

fn process_passwords(input: impl BufRead) -> impl Iterator<Item = (usize, usize, char, String)> {
    input.lines().map(|line| {
        let line = line.unwrap();
        let captures = RE
            .captures(&line)
            .expect("Bad line that does not match regex.");
        let letter = captures["letter"].as_bytes()[0] as char;
        let password = captures.get(4).unwrap().as_str();
        let max = captures["max"].parse().unwrap();
        let min = captures["min"].parse().unwrap();
        (min, max, letter, password.to_string())
    })
}

pub fn star_one(input: impl BufRead) -> usize {
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

pub fn star_two(input: impl BufRead) -> usize {
    process_passwords(input)
        .filter(|(min, max, letter, password)| {
            let chars: Vec<char> = password.chars().collect();
            if *min > chars.len() || *max > chars.len() {
                panic!("Bad line: {}", password);
            }
            (chars[*min - 1] == *letter) ^ (chars[*max - 1] == *letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")),
            2
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc")),
            1
        );
    }
}
