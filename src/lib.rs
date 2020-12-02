#[macro_use]
extern crate lazy_static;


mod day01;
mod day02;

#[cfg(test)]
mod tests {
    use std::fs;
    #[test]
    fn day01_complete() {
        use crate::day01::{star_one, star_two};

        let input = fs::read_to_string("day01.txt").unwrap();
        assert_eq!(star_one(&input), 805731);
        assert_eq!(star_two(&input), 192684960);
    }

    #[test]
    fn day02_complete() {
        use crate::day02::{star_one, star_two};

        let input = fs::read_to_string("day02.txt").unwrap();
        assert_eq!(star_one(&input), 469);
        assert_eq!(star_two(&input), 267);
    }
}
