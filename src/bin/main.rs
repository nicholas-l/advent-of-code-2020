use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use advent_of_code_2020::*;

type DayFn = fn(Box<dyn BufRead>) -> usize;

fn main() {
    let days: Vec<String> = env::args().skip(1).collect();
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
            // "3" => {
            //   use day03::{star_one, star_two};
            //   (star_one, star_two, "day01.txt")

            // },
            _ => {
                unimplemented!();
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
