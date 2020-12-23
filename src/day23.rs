use std::{collections::LinkedList, io::BufRead, time::Instant};

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

fn step_linked(v: &mut LinkedList<usize>, max: usize) -> usize {
    // let now = Instant::now();

    let current_value = v.pop_front().unwrap();
    let mut cursor = v.cursor_front_mut();
    // dbg!(now.elapsed());
    let mut d1 = current_value - 1;
    cursor.move_next();
    cursor.move_next();
    cursor.move_next();
    // dbg!(now.elapsed());

    let taken_cups = cursor.split_before();
    // dbg!(taken_cups);
    // cursor.insert_before(current_value);

    // dbg!(d1);
    // dbg!(now.elapsed());
    // Find destination cup value
    let d = loop {
        // Assume we dont have anythin less that 1
        if d1 < 1 {
            d1 = max;
        }
        if !taken_cups.contains(&d1) {
            break d1;
        } else {
            d1 -= 1;
        }
    };
    // dbg!(d);
    // dbg!(now.elapsed());

    while let Some(x) = cursor.current() {
        if x == &d {
            break;
        } else {
            cursor.move_next();
        }
    }
    // dbg!(now.elapsed());

    cursor.splice_after(taken_cups);
    // dbg!(&other);

    // dbg!(now.elapsed());
    // let val = v.pop_front().unwrap();
    // v.push_back(val);
    v.push_back(current_value);

    1
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let mut v: LinkedList<usize> = input
        .bytes()
        .map(|x| (x.unwrap() - b'0') as usize)
        .collect();
    let number_of_cups = v.len();

    let max = *v.iter().max().unwrap();
    let min = *v.iter().min().unwrap();
    let current_index = 0;
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
        step_linked(&mut v, max);
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
    let part2 = v.split_off(v.iter().position(|x| x == &1).unwrap());
    println!("{:?}", part2);
    let part1 = v;
    format!(
        "{}{}",
        part1.iter().map(|x| x.to_string()).collect::<String>(),
        part2
            .iter()
            .skip(1)
            .map(|x| x.to_string())
            .collect::<String>(),
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
    let upper = 1_000_000;
    cups.extend(max..=upper);
    let number_of_cups = cups.len();
    assert!(cups.contains(&1_000_000));

    let mut cups: LinkedList<usize> = cups.into_iter().collect();
    let number_of_rounds = 10_000_000;

    let now = Instant::now();
    let print_every = 10_000;
    for round in 0..number_of_rounds {
        step_linked(&mut cups, 1_000_000);
        if round % print_every == 0 {
            println!(
                "{} : {}",
                now.elapsed().as_micros() / print_every * (number_of_rounds - round * print_every),
                round as f64 * 100f64 / number_of_rounds as f64
            );
        }
    }
    // println!("-- final --");
    // println!(
    //     "cups:  {}",
    //     cups.iter()
    //         .enumerate()
    //         .map(|(i, x)| if i == current_index {
    //             format!("({}) ", x)
    //         } else {
    //             format!("{} ", x)
    //         })
    //         .collect::<String>()
    // );
    println!("Done, now to find 1");
    while let Some(x) = cups.pop_front() {
        if x == 1 {
            break;
        }
    }
    let cup1 = cups.pop_front().unwrap();
    let cup2 = cups.pop_front().unwrap();
    cup1 * cup2
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
    fn test_step_linked() {
        let mut v: LinkedList<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into_iter().collect();

        step_linked(&mut v, 9);
        let expected = vec![2, 8, 9, 1, 5, 4, 6, 7, 3].into_iter().collect();
        assert_eq!(v, expected);

        step_linked(&mut v, 9);
        let mut expected2 = vec![5, 4, 6, 7, 8, 9, 1, 3, 2].into_iter().collect();
        assert_eq!(v, expected2);

        step_linked(&mut expected2, 9);
        let mut expected3 = vec![8, 9, 1, 3, 4, 6, 7, 2, 5].into_iter().collect();
        assert_eq!(expected2, expected3);

        step_linked(&mut expected3, 9);
        let mut expected4 = vec![4, 6, 7, 9, 1, 3, 2, 5, 8].into_iter().collect();
        assert_eq!(expected3, expected4);

        step_linked(&mut expected4, 9);
        assert_eq!(
            expected4,
            vec![1, 3, 6, 7, 9, 2, 5, 8, 4].into_iter().collect()
        );
    }

    #[test]
    fn test_star_two() {
        let input = b"389125467";
        assert_eq!(star_two(Cursor::new(input)), 149245887792);
    }
}
