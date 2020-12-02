use itertools::Itertools;

fn get_product_matching_sum(input: &str, length: usize, sum: usize) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .permutations(length)
        .filter(|x| x.iter().sum::<usize>() == sum)
        .next()
        .map(|x| x.iter().product())
        .expect("Could not find numbers")
}

pub fn star_one(input: &str) -> usize {
    get_product_matching_sum(input, 2, 2020)
}

pub fn star_two(input: &str) -> usize {
    get_product_matching_sum(input, 3, 2020)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    #[test]
    fn test_star_one() {
        assert_eq!(star_one("1721\n979\n366\n299\n675\n1456"), 514579);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two("1721\n979\n366\n299\n675\n1456"), 241861950);
    }
}
