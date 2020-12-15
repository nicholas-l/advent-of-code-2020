use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io::BufRead;

fn run(input: impl BufRead, index: usize) -> usize {
    let starting: Vec<usize> = input
        .split(b',')
        .filter_map(Result::ok)
        .map(|x| String::from_utf8(x).unwrap().parse::<usize>().unwrap())
        .collect();
    let mut map: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
    // let mut first: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last_spoken = starting[2];
    for (i, x) in starting.iter().enumerate() {
        map.insert(*x, (i, None));
    }

    let start = starting.len();

    for i in start..index {
        // If we have just spoken it the first time in the previous turn
        let x = map.get(&last_spoken).unwrap();
        last_spoken = if let Some(later) = x.1 {
            later - x.0
        } else {
            0
        };

        // Insert this new value into the map;
        match map.entry(last_spoken) {
            Entry::Occupied(mut e) => {
                // We have spoken this before.
                let v = e.get_mut();
                if let Some(later) = v.1 {
                    v.0 = later;
                    v.1.replace(i);
                } else {
                    v.1 = Some(i);
                }
            }
            Entry::Vacant(o) => {
                // First time spoken
                o.insert((i, None));
            }
        }
    }
    last_spoken
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    run(input, 2020)
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    run(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        {
            let input = b"0,3,6";
            assert_eq!(star_one(Cursor::new(input)), 436);
        }
        {
            let input = b"1,3,2";
            assert_eq!(star_one(Cursor::new(input)), 1);
        }
    }

    #[test]
    fn test_star_two() {
        let input = b"0,3,6";
        assert_eq!(star_two(Cursor::new(input)), 175594);
    }
}
