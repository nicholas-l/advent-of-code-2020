use std::collections::HashSet;
use std::io::BufRead;

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    input_str
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<char>>()
        })
        .map(|questions| questions.len())
        .sum()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    input_str
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|person| person.chars().collect::<HashSet<char>>())
                // Wish there was fold_first on stable but this does the same job.
                .fold(None, |acc: Option<HashSet<char>>, person| match acc {
                    Some(acc) => Some(acc.intersection(&person).copied().collect()),
                    None => Some(person),
                })
        })
        .map(|x| x.unwrap().len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(
                "abc

a
b
c

ab
ac

a
a
a
a

b"
            )),
            11
        );
    }

    #[test]
    #[ignore]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new("INPUT")), 336);
    }
}
