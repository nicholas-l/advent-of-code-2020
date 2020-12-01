use itertools::Itertools;

pub fn star_one(input: &str) -> usize {
    input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .permutations(2)
        .filter(|x| x[0] + x[1] == 2020)
        .next()
        .map(|x| x[0] * x[1])
        .expect("Could not find numbers")
}

pub fn star_two(input: &str) -> usize {
  input
  .lines()
  .map(|x| x.parse::<usize>().unwrap())
  .permutations(3)
  .filter(|x| x.iter().sum::<usize>() == 2020)
  .next()
  .map(|x| x.iter().product())
  .expect("Could not find numbers")
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
