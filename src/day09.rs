use itertools::Itertools;
use std::io::BufRead;

fn xmas(numbers: &[usize], length: usize) -> usize {
    *numbers
        .iter()
        .skip(length)
        .enumerate()
        .find(|(index, &number)| {
            for p in numbers[*index..(index + length)].iter().permutations(2) {
                if p[0] + p[1] == number {
                    return false;
                }
            }
            true
        })
        .unwrap()
        .1
}

fn xmas2(numbers: &[usize], value: usize) -> Option<Vec<usize>> {
    for k in 2..numbers.len() {
        for p in numbers.windows(k) {
            if p.iter().sum::<usize>() == value {
                return Some(p.to_vec());
            }
        }
    }
    None
}

pub fn star_one(input: impl BufRead) -> usize {
    let numbers: Vec<usize> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect();
    xmas(&numbers, 25)
}

pub fn star_two(input: impl BufRead) -> usize {
    let numbers: Vec<usize> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect();
    let values = xmas2(&numbers, xmas(&numbers, 25)).unwrap();
    match (values.iter().min(), values.iter().max()) {
        (Some(x), Some(y)) => x + y,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::{xmas, xmas2};

    #[test]
    fn test_xmas() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(xmas(&numbers, 5), 127);
    }

    #[test]
    fn test_star_two() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(xmas2(&numbers, 127), Some(vec![15, 25, 47, 40]));
    }
}
