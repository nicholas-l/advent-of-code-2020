#[macro_use]
extern crate lazy_static;

pub mod day01;
pub mod day02;
pub mod day03;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::BufReader;
    #[test]
    fn day01_complete() {
        use crate::day01::{star_one, star_two};

        let f = fs::File::open("day01.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 805731);
        let f = fs::File::open("day01.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 192684960);
    }

    #[test]
    fn day02_complete() {
        use crate::day02::{star_one, star_two};

        let f = fs::File::open("day02.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 469);
        let f = fs::File::open("day02.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 267);
    }
}
