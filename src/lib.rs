// #![feature(map_first_last)]
use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

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
// pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;

type DayFn = fn(Box<dyn BufRead>) -> usize;

pub fn get_day(day: usize) -> (DayFn, DayFn, PathBuf) {
    match day {
        1 => {
            use day01::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day01.txt"),
            )
        }
        2 => {
            use day02::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day02.txt"),
            )
        }
        3 => {
            use day03::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day03.txt"),
            )
        }
        4 => {
            use day04::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day04.txt"),
            )
        }
        5 => {
            use day05::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day05.txt"),
            )
        }
        6 => {
            use day06::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day06.txt"),
            )
        }
        7 => {
            use day07::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day07.txt"),
            )
        }
        8 => {
            use day08::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day08.txt"),
            )
        }
        9 => {
            use day09::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day09.txt"),
            )
        }
        // 10 => {
        //     use day10::{star_one, star_two};
        //     (
        //         star_one as DayFn,
        //         star_two as DayFn,
        //         Path::new("data").join("day10.txt"),
        //     )
        // }
        11 => {
            use day11::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day11.txt"),
            )
        }
        12 => {
            use day12::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day12.txt"),
            )
        }
        13 => {
            use day13::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day13.txt"),
            )
        }
        14 => {
            use day14::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day14.txt"),
            )
        }

        15 => {
            use day15::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day15.txt"),
            )
        }
        16 => {
            use day16::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day16.txt"),
            )
        }
        17 => {
            use day17::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day17.txt"),
            )
        }
        18 => {
            use day18::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day18.txt"),
            )
        }
        19 => {
            use day19::{star_one, star_two};
            (
                star_one as DayFn,
                star_two as DayFn,
                Path::new("data").join("day19.txt"),
            )
        }
        
        x => {
            unimplemented!("Have not implemented day {}", x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::BufReader;

    fn get_data(filepath: &PathBuf) -> Box<dyn BufRead> {
        let f = fs::File::open(filepath).unwrap();
        let input = BufReader::new(f);
        Box::new(input)
    }

    #[test]
    fn day01_complete() {
        let (star_one, star_two, filepath) = get_day(1);
        assert_eq!(star_one(get_data(&filepath)), 805731);

        assert_eq!(star_two(get_data(&filepath)), 192684960);
    }

    #[test]
    fn day02_complete() {
        let (star_one, star_two, filename) = get_day(2);

        assert_eq!(star_one(get_data(&filename)), 469);
        assert_eq!(star_two(get_data(&filename)), 267);
    }

    #[test]
    fn day03_complete() {
        let (star_one, star_two, filename) = get_day(3);

        assert_eq!(star_one(get_data(&filename)), 145);
        assert_eq!(star_two(get_data(&filename)), 3424528800);
    }

    #[test]
    fn day04_complete() {
        let (star_one, star_two, filename) = get_day(4);

        assert_eq!(star_one(get_data(&filename)), 230);
        assert_eq!(star_two(get_data(&filename)), 156);
    }

    #[test]
    fn day05_complete() {
        let (star_one, star_two, filename) = get_day(5);

        assert_eq!(star_one(get_data(&filename)), 994);
        assert_eq!(star_two(get_data(&filename)), 741);
    }

    #[test]
    fn day06_complete() {
        let (star_one, star_two, filename) = get_day(6);

        assert_eq!(star_one(get_data(&filename)), 6382);
        assert_eq!(star_two(get_data(&filename)), 3197);
    }

    #[test]
    fn day07_complete() {
        let (star_one, star_two, filename) = get_day(7);

        assert_eq!(star_one(get_data(&filename)), 172);
        assert_eq!(star_two(get_data(&filename)), 39645);
    }

    #[test]
    fn day08_complete() {
        let (star_one, star_two, filename) = get_day(8);

        assert_eq!(star_one(get_data(&filename)), 1816);
        assert_eq!(star_two(get_data(&filename)), 1149);
    }

    #[test]
    fn day09_complete() {
        let (star_one, star_two, filename) = get_day(9);

        assert_eq!(star_one(get_data(&filename)), 18272118);
        assert_eq!(star_two(get_data(&filename)), 2186361);
    }

    #[test]
    fn day10_complete() {
        let (star_one, star_two, filename) = get_day(10);

        assert_eq!(star_one(get_data(&filename)), 2201);
        assert_eq!(star_two(get_data(&filename)), 169255295254528);
    }

    #[test]
    fn day11_complete() {
        let (star_one, star_two, filename) = get_day(11);

        assert_eq!(star_one(get_data(&filename)), 2412);
        assert_eq!(star_two(get_data(&filename)), 2176);
    }
    #[test]
    fn day12_complete() {
        let (star_one, star_two, filename) = get_day(12);

        assert_eq!(star_one(get_data(&filename)), 1457);
        assert_eq!(star_two(get_data(&filename)), 106860);
    }
    #[test]
    fn day13_complete() {
        let (star_one, star_two, filename) = get_day(13);

        assert_eq!(star_one(get_data(&filename)), 4315);
        assert_eq!(star_two(get_data(&filename)), 556100168221141);
    }

    #[test]
    fn day14_complete() {
        let (star_one, star_two, filename) = get_day(14);

        assert_eq!(star_one(get_data(&filename)), 7997531787333);
        assert_eq!(star_two(get_data(&filename)), 3564822193820);
    }

    #[test]
    fn day15_complete() {
        let (star_one, star_two, filename) = get_day(15);

        assert_eq!(star_one(get_data(&filename)), 639);
        assert_eq!(star_two(get_data(&filename)), 266);
    }
    #[test]
    fn day16_complete() {
        let (star_one, star_two, filename) = get_day(16);

        assert_eq!(star_one(get_data(&filename)), 25972);
        assert_eq!(star_two(get_data(&filename)), 622670335901);
    }


    #[test]
    fn day17_complete() {
        let (star_one, star_two, filename) = get_day(17);

        assert_eq!(star_one(get_data(&filename)), 375);
        assert_eq!(star_two(get_data(&filename)), 2192);
    }
    #[test]
    fn day18_complete() {
        let (star_one, star_two, filename) = get_day(18);

        assert_eq!(star_one(get_data(&filename)), 5374004645253);
        assert_eq!(star_two(get_data(&filename)), 88782789402798);
    }

    #[test]
    fn day19_complete() {
        let (star_one, star_two, filename) = get_day(19);

        assert_eq!(star_one(get_data(&filename)), 115);
        assert_eq!(star_two(get_data(&filename)), 237);
    }
}
