use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use advent_of_code_2020::*;

type DayFn = fn(Box<dyn BufRead>) -> usize;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let days = if args.len() == 1 && args[0] == "all" {
        (1..=7).map(|x| x.to_string()).collect()
    } else {
        args
    };
    for day in days {
        let (star_one, star_two, filename) = match day.as_str() {
            "1" => {
                use day01::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day01.txt")
            }
            "2" => {
                use day02::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day02.txt")
            }
            "3" => {
                use day03::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day03.txt")
            }
            "4" => {
                use day04::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day04.txt")
            }
            "5" => {
                use day05::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day05.txt")
            }
            "6" => {
                use day06::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day06.txt")
            }
            "7" => {
                use day07::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day07.txt")
            }
            "8" => {
                use day08::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day08.txt")
            }
            "9" => {
                use day09::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day09.txt")
            }
            "10" => {
                use day10::{star_one, star_two};
                (star_one as DayFn, star_two as DayFn, "day10.txt")
            }
            x => {
                unimplemented!("Have not implemented day {}", x);
            }
        };

        println!("Day {}:", day);
        let f = fs::File::open(filename).unwrap();
        let input = BufReader::new(f);
        println!("Star One: {}", star_one(Box::new(input)));
        let f = fs::File::open(filename).unwrap();
        let input = BufReader::new(f);
        println!("Star Two: {}", star_two(Box::new(input)));
    }
}
