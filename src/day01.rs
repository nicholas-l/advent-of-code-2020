use itertools::Itertools;
use std::io::BufRead;

fn get_product_matching_sum(input: impl BufRead, length: usize, sum: usize) -> usize {
    input
        .lines()
        .map(|x| x.unwrap().parse::<usize>().unwrap())
        .permutations(length)
        .find(|x| x.iter().sum::<usize>() == sum)
        .map(|x| x.iter().product())
        .expect("Could not find numbers")
}

pub fn star_one(input: impl BufRead) -> usize {
    get_product_matching_sum(input, 2, 2020)
}

pub fn star_two(input: impl BufRead) -> usize {
    get_product_matching_sum(input, 3, 2020)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        assert_eq!(
            star_one(Cursor::new(b"1721\n979\n366\n299\n675\n1456")),
            514579
        );
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(Cursor::new(b"1721\n979\n366\n299\n675\n1456")),
            241861950
        );
    }
}
