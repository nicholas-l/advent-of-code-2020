use std::io::BufRead;

fn get_loop_size(pkey: usize, subject_number: usize) -> usize {
    let mut value = 1;
    let mut i = 0;
    while value != pkey {
        value *= subject_number;
        value %= 20201227;

        i += 1;
    }
    i
}

fn create_encryption_key(subject_number: usize, loops: usize) -> usize {
    let mut value = 1;
    for _ in 0..loops {
        value *= subject_number;
        value %= 20201227;
    }
    value
}

pub fn star_one(input: impl BufRead) -> usize {
    let pkeys: Vec<usize> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let loops: Vec<usize> = pkeys.iter().map(|pkey| get_loop_size(*pkey, 7)).collect();
    create_encryption_key(pkeys[1], loops[0])
}

pub fn star_two(_input: impl BufRead) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_loop_size() {
        assert_eq!(get_loop_size(5764801, 7), 8);
        assert_eq!(get_loop_size(17807724, 7), 11);
    }

    #[test]
    fn test_star_one() {
        let input = b"5764801
17807724";
        assert_eq!(star_one(Cursor::new(input)), 14897079);
    }

    #[test]
    fn test_star_two() {
        let input = b"";
        assert_eq!(star_two(Cursor::new(input)), 1);
    }
}
