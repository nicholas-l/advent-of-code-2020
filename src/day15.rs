use std::collections::HashMap;
use std::io::BufRead;
use std::collections::hash_map::Entry;

fn run(input: impl BufRead, index: usize) -> usize {
    let starting: Vec<usize> = input
        .split(b',')
        .filter_map(Result::ok)
        .map(|x| String::from_utf8(x).unwrap().parse::<usize>().unwrap())
        .collect();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut last_spoken = starting[2];
    for (i, x) in starting.iter().enumerate() {
        map.insert(*x, vec![i]);
    }

    let mut first_time_spoken = true;
    for i in starting.len()..index {
        // If we have just spoken it the first time in the previous turn
        let value = if first_time_spoken {
            0
        } else {
            let x = map.get(&last_spoken).unwrap();
            x[1] - x[0]
        };

        // Insert this new value into the map;
        match map.entry(value) {
            Entry::Occupied(mut e) => {
                // We have spoken this before.
                first_time_spoken = false;
                let v = e.get_mut();
                v.push(i);
                if v.len() > 2 {
                    v.remove(0);
                }
            }
            Entry::Vacant(o) => {
                // First time spoken
                first_time_spoken = true;
                o.insert(vec![i]);
            }
        }
        last_spoken = value;
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
