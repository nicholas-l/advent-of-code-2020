use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher},
    io::BufRead,
};

fn step(v: &mut Vec<usize>, current: usize, print: bool) -> usize {
    let current_value = v[current];
    let mut d1 = v[current] - 1;
    let len = v.len();

    let to_remove1 = v[(current + 1) % v.len()];
    let to_remove2 = v[(current + 2) % v.len()];
    let to_remove3 = v[(current + 3) % v.len()];

    let c1 = v.remove(v.iter().position(|x| x == &to_remove1).unwrap());
    let c2 = v.remove(v.iter().position(|x| x == &to_remove2).unwrap());
    let c3 = v.remove(v.iter().position(|x| x == &to_remove3).unwrap());
    let min = *v.iter().min().unwrap();

    if print {
        println!("pick up: {} {} {}", c1, c2, c3);
    }
    // Find destination cup
    let d = loop {
        if d1 < min {
            break v.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
        } else if let Some(p) = v.iter().position(|x| x == &d1) {
            break p;
        } else {
            d1 -= 1;
        }
    };
    if print {
        println!("{:?} - {}", v, d);
    }
    v.rotate_right(len - 3 - d - 1);
    if print {
        println!("{:?}", v);
    }
    // x x x 4 5 6 7 8
    //
    if print {
        println!("destination: {} ({})", v[d], d);
    }
    // Insert new cups
    v.push(c1);
    v.push(c2);
    v.push(c3);

    v.rotate_left(len - d - 1);

    v.iter().position(|x| x == &current_value).unwrap()
}


#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let mut v: Vec<usize> = input
        .bytes()
        .map(|x| (x.unwrap() - b'0') as usize)
        .collect();
    let number_of_cups = v.len();

    let max = *v.iter().max().unwrap();
    let min = *v.iter().min().unwrap();
    let mut current_index = 0;
    for round in 0..100 {
        println!("-- move {} --", round + 1);
        println!(
            "cups: {}",
            v.iter()
                .enumerate()
                .map(|(i, x)| if i == current_index {
                    format!("({}) ", x)
                } else {
                    format!("{} ", x)
                })
                .collect::<String>()
        );
        current_index = step(&mut v, current_index, true);
        // Get new current position
        current_index = (current_index + 1) % v.len();
    }
    println!("-- final --");
    println!(
        "cups:  {}",
        v.iter()
            .enumerate()
            .map(|(i, x)| if i == current_index {
                format!("({}) ", x)
            } else {
                format!("{} ", x)
            })
            .collect::<String>()
    );
    let mut c = v.splitn(2, |x| x == &1);
    let part2 = c.next().unwrap().to_vec();
    // part2.reverse();
    println!("{:?}", part2);
    let part1 = c.next().unwrap();
    format!(
        "{}{}",
        part1.iter().map(|x| x.to_string()).collect::<String>(),
        part2.iter().map(|x| x.to_string()).collect::<String>(),
    )
    .parse::<usize>()
    .unwrap()
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let mut cups: Vec<usize> = input
        .bytes()
        .map(|x| (x.unwrap() - b'0') as usize)
        .collect();
    let max = *cups.iter().max().unwrap() + 1;
    let upper = max + 1000000 - cups.len();
    cups.extend(max..upper);
    let mut current_index = 0;
    let number_of_cups = cups.len();
    let mut cache: HashMap<u64, Vec<usize>> = HashMap::new();
    for round in 0..10000000 {
        let s = cache.hasher();
        let mut hasher = s.build_hasher();
        current_index.hash(&mut hasher);
        cups.hash(&mut hasher);
        let hash = hasher.finish();
        match cache.entry(hash) {
            std::collections::hash_map::Entry::Occupied(e) => {
                cups = e.get().clone();
            }
            std::collections::hash_map::Entry::Vacant(o) => {
                step(&mut cups, current_index, false);
                o.insert(cups.clone());
            }
        }
        // Get new current position
        current_index = (current_index + 1) % number_of_cups;
    }
    println!("-- final --");
    println!(
        "cups:  {}",
        cups.iter()
            .enumerate()
            .map(|(i, x)| if i == current_index {
                format!("({}) ", x)
            } else {
                format!("{} ", x)
            })
            .collect::<String>()
    );
    let mut c = cups.splitn(2, |x| x == &1);
    let part2 = c.next().unwrap().to_vec();
    // part2.reverse();
    println!("{:?}", part2);
    let part1 = c.next().unwrap();
    format!(
        "{}{}",
        part1.iter().map(|x| x.to_string()).collect::<String>(),
        part2.iter().map(|x| x.to_string()).collect::<String>(),
    )
    .parse::<usize>()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_step() {
        let mut v = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut current = 0;

        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![7, 2, 5, 8, 9, 1, 3, 4, 6]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 5, 8, 4, 6, 7, 9, 1]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![9, 2, 5, 8, 4, 1, 3, 6, 7]);
    }

    #[test]
    fn test_star_one() {
        let input = b"389125467";
        assert_eq!(star_one(Cursor::new(input)), 67384529);
    }

    #[test]
    fn test_star_two() {
        let input = b"389125467";
        assert_eq!(star_two(Cursor::new(input)), 149245887792);
    }
}
