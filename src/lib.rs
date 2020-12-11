#![feature(map_first_last)]

#[macro_use]
extern crate lazy_static;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;

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

    #[test]
    fn day03_complete() {
        use crate::day03::{star_one, star_two};

        let f = fs::File::open("day03.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 145);

        let f = fs::File::open("day03.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 3424528800);
    }

    #[test]
    fn day04_complete() {
        use crate::day04::{star_one, star_two};

        let f = fs::File::open("day04.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 230);

        let f = fs::File::open("day04.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 156);
    }

    #[test]
    fn day05_complete() {
        use crate::day05::{star_one, star_two};

        let f = fs::File::open("day05.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 994);

        let f = fs::File::open("day05.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 741);
    }

    #[test]
    fn day06_complete() {
        use crate::day06::{star_one, star_two};

        let f = fs::File::open("day06.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 6382);

        let f = fs::File::open("day06.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 3197);
    }

    #[test]
    fn day07_complete() {
        use crate::day07::{star_one, star_two};

        let f = fs::File::open("day07.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 172);

        let f = fs::File::open("day07.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 39645);
    }

    #[test]
    fn day08_complete() {
        use crate::day08::{star_one, star_two};

        let f = fs::File::open("day08.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 1816);

        let f = fs::File::open("day08.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 1149);
    }

    #[test]
    fn day09_complete() {
        use crate::day09::{star_one, star_two};

        let f = fs::File::open("day09.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 18272118);

        let f = fs::File::open("day09.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 2186361);
    }

    #[test]
    fn day10_complete() {
        use crate::day10::{star_one, star_two};

        let f = fs::File::open("day10.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 2201);

        let f = fs::File::open("day10.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 169255295254528);
    }

    #[test]
    fn day11_complete() {
        use crate::day11::{star_one, star_two};

        let f = fs::File::open("day11.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_one(input), 2412);

        let f = fs::File::open("day11.txt").unwrap();
        let input = BufReader::new(f);
        assert_eq!(star_two(input), 2176);
    }
}
