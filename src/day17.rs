use std::{collections::HashMap, fmt::Debug};
use std::{convert::TryFrom, io::BufRead};

#[derive(Copy, Clone, PartialEq)]
enum Cube {
    Active,
    Inactive,
}

impl TryFrom<char> for Cube {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Cube::Active),
            '.' => Ok(Cube::Inactive),
            _ => Err(format!("Could not match {} to SeatStatus", c)),
        }
    }
}

impl Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cube::Active => '#',
            Cube::Inactive => '.',
        };
        f.write_fmt(format_args!("{}", c))
    }
}

// https://stackoverflow.com/questions/18732974/c-dynamic-number-of-nested-for-loops-without-recursion
fn iterative_nested_loop(depth: usize, max: Vec<usize>, mut action: impl FnMut(&Vec<usize>)) {
    // Initialize the slots to hold the current iteration value for each depth
    let mut slots = vec![0; depth];

    let mut index = 0;
    loop {
        action(&slots);
        println!("{:?}", slots);
        // Increment
        slots[0] += 1;

        // Carry
        while slots[index] == max[index] {
            // Overflow, we're done
            if index == depth - 1 {
                return;
            }

            slots[index] = 0;
            index += 1;
            slots[index] += 1;
        }

        index = 0;
    }
}

// #[derive(Clone)]
// struct Map {
//     data: HashMap<Vec<usize>, Cube>,
//     dimensions: Vec<usize>,
// }

// impl Map {
//     fn pad(&mut self, amount: usize) {
//         let data = HashMap::new();
//         for (pos, cube) in data {
//             let new_pos = pos.into_iter().map(|i| i + 1).collect();
//             data.insert(new_pos, cube);
//         }
//         self.dimensions = self.dimensions.into_iter().map(|i| i + 2).collect();
//         self.data = data;
//     }
// }

type State = Vec<Vec<Vec<Cube>>>;

fn count_active(state: &State, i: usize, j: usize, k: usize) -> usize {
    let dirs = (-1..=1)
        .flat_map(|i| {
            (-1..=1)
                .flat_map(|j| {
                    (-1..=1)
                        .map(|k| (i, j, k))
                        .collect::<Vec<(isize, isize, isize)>>()
                })
                .collect::<Vec<(isize, isize, isize)>>()
        })
        .filter(|x| !(x.0 == 0 && x.1 == 0 && x.2 == 0));
    let mut count = 0;
    for dir in dirs {
        let c_i = i as isize + dir.0;
        let c_j = j as isize + dir.1;
        let c_k = k as isize + dir.2;
        if c_i >= 0
            && c_j >= 0
            && c_k >= 0
            && c_i < state.len() as isize
            && c_j < state[c_i as usize].len() as isize
            && c_k < state[c_i as usize][c_j as usize].len() as isize
            && state[c_i as usize][c_j as usize][c_k as usize] == Cube::Active
        {
            count += 1;
        }
    }
    count
}

fn step(state: State) -> State {
    let depth = state.len() + 2;
    let number_of_rows = state[0].len() + 2;
    let number_of_columns = state[0][0].len() + 2;
    // let mut new_state = state.pad(1);
    let mut new_state = vec![vec![vec![Cube::Inactive; number_of_columns]; number_of_rows]; depth];
    let mut old_state = vec![vec![vec![Cube::Inactive; number_of_columns]; number_of_rows]; depth];
    for i in 0..(depth - 2) {
        for j in 0..(number_of_rows - 2) {
            for k in 0..(number_of_columns - 2) {
                old_state[i + 1][j + 1][k + 1] = state[i][j][k]
            }
        }
    }
    let maxes = vec![depth, number_of_rows, number_of_columns];
    iterative_nested_loop(maxes.len(), maxes, |v| {
        let i = v[0];
        let j = v[1];
        let k = v[2];
        let current = old_state[i][j][k];
        new_state[i][j][k] = match (current, count_active(&old_state, i, j, k)) {
            (Cube::Active, 2..=3) => Cube::Active,
            (Cube::Inactive, 3) => Cube::Active,
            _ => Cube::Inactive,
        }
    });
    new_state
}

fn count_active2(
    state: &Vec<Vec<Vec<Vec<Cube>>>>,
    i: usize,
    j: usize,
    k: usize,
    w: usize,
) -> usize {
    // Calculate all the relative positions for the neighbours.
    let dirs = (-1..=1)
        .flat_map(|i| {
            (-1..=1)
                .flat_map(|j| {
                    (-1..=1)
                        .flat_map(|k| {
                            (-1..=1)
                                .map(|w| (i, j, k, w))
                                .collect::<Vec<(isize, isize, isize, isize)>>()
                        })
                        .collect::<Vec<(isize, isize, isize, isize)>>()
                })
                .collect::<Vec<(isize, isize, isize, isize)>>()
        })
        .filter(|x| !(x.0 == 0 && x.1 == 0 && x.2 == 0 && x.3 == 0));
    let mut count = 0;
    for dir in dirs {
        let c_i = i as isize + dir.0;
        let c_j = j as isize + dir.1;
        let c_k = k as isize + dir.2;
        let c_w = w as isize + dir.3;
        if c_i >= 0
            && c_j >= 0
            && c_k >= 0
            && c_w >= 0
            && c_i < state.len() as isize
            && c_j < state[c_i as usize].len() as isize
            && c_k < state[c_i as usize][c_j as usize].len() as isize
            && c_w < state[c_i as usize][c_j as usize][c_k as usize].len() as isize
            && state[c_i as usize][c_j as usize][c_k as usize][c_w as usize] == Cube::Active
        {
            count += 1;
        }
    }
    count
}

fn step2(state: Vec<Vec<Vec<Vec<Cube>>>>) -> Vec<Vec<Vec<Vec<Cube>>>> {
    let depth = state.len() + 2;
    let number_of_rows = state[0].len() + 2;
    let number_of_columns = state[0][0].len() + 2;
    let max_w = state[0][0][0].len() + 2;
    let mut new_state =
        vec![vec![vec![vec![Cube::Inactive; max_w]; number_of_columns]; number_of_rows]; depth];
    let mut old_state =
        vec![vec![vec![vec![Cube::Inactive; max_w]; number_of_columns]; number_of_rows]; depth];
    for i in 0..(depth - 2) {
        for j in 0..(number_of_rows - 2) {
            for k in 0..(number_of_columns - 2) {
                for w in 0..(max_w - 2) {
                    old_state[i + 1][j + 1][k + 1][w + 1] = state[i][j][k][w]
                }
            }
        }
    }
    for i in 0..depth {
        for j in 0..number_of_rows {
            for k in 0..number_of_columns {
                for w in 0..max_w {
                    let current = old_state[i][j][k][w];
                    new_state[i][j][k][w] = match (current, count_active2(&old_state, i, j, k, w)) {
                        (Cube::Active, 2..=3) => Cube::Active,
                        (Cube::Inactive, 3) => Cube::Active,
                        _ => Cube::Inactive,
                    }
                }
            }
        }
    }
    new_state
}

fn parse_state(input: impl BufRead) -> State {
    let numbers: Vec<Vec<Cube>> = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().map(|c| Cube::try_from(c).unwrap()).collect())
        .collect();
    let mut state = Vec::new();
    state.push(numbers);
    state
}

fn print_state(state: &State) {
    for z in state {
        for row in z {
            for x in row {
                print!("{:?}", x);
            }
            println!("");
        }
        println!("");
    }
}

fn print_state2(state: &Vec<Vec<Vec<Vec<Cube>>>>) {
    for w in state {
        for z in w {
            for row in z {
                for x in row {
                    print!("{:?}", x);
                }
                println!("");
            }
            println!("");
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let mut state = parse_state(input);
    println!("{:?}", state);
    for i in 0..6 {
        state = step(state);
    }
    state
        .into_iter()
        .flat_map(|plane| plane.into_iter().flat_map(|row| row))
        .filter(|x| x == &Cube::Active)
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let mut state = vec![parse_state(input)];
    println!("{:?}", state);
    for i in 0..6 {
        state = step2(state);
    }
    state
        .into_iter()
        .flat_map(|cube| {
            cube.into_iter()
                .flat_map(|plane| plane.into_iter().flat_map(|row| row))
        })
        .filter(|x| x == &Cube::Active)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_loop() {
        iterative_nested_loop(2, vec![3, 5], |v| println!("{:?}", v));
    }

    #[test]
    fn test_count_active() {
        let input = b".#.
..#
###";
        let dirs: Vec<(isize, isize, isize)> = (-1..=1)
            .flat_map(|i| {
                (-1..=1)
                    .flat_map(|j| {
                        (-1..=1)
                            .map(|k| (i, j, k))
                            .collect::<Vec<(isize, isize, isize)>>()
                    })
                    .collect::<Vec<(isize, isize, isize)>>()
            })
            .filter(|x| !(x.0 == 0 && x.1 == 0 && x.2 == 0))
            .collect();
        assert_eq!(dirs.len(), 26);
        let state = parse_state(Cursor::new(input));
        // dbg!(&state);

        assert_eq!(count_active(&state, 0, 0, 1), 1);
    }

    #[test]
    fn test_step() {
        let input = b".#.
..#
###";

        let state = parse_state(Cursor::new(input));
        // dbg!(&state);
        let z1 = b"#..
..#
.#.";

        let z2 = "#.#
.##
.#.";
        let z3 = "#..
..#
.#.";
        let expected = vec![
            parse_state(Cursor::new(z1)).pop().unwrap(),
            parse_state(Cursor::new(z2)).pop().unwrap(),
            parse_state(Cursor::new(z3)).pop().unwrap(),
        ];
        let new_state = step(state);

        print_state(&new_state);
        // assert_eq!(new_state, expected);
    }

    #[test]
    fn test_star_one() {
        let input = b".#.
..#
###";
        assert_eq!(star_one(Cursor::new(input)), 112);
    }

    #[test]
    fn test_step2() {
        let input = b".#.
..#
###";

        let state = vec![parse_state(Cursor::new(input))];
        // dbg!(&state);
        let z1 = b"#..
..#
.#.";

        let z2 = "#.#
.##
.#.";
        let z3 = "#..
..#
.#.";
        let expected = vec![
            parse_state(Cursor::new(z1)).pop().unwrap(),
            parse_state(Cursor::new(z2)).pop().unwrap(),
            parse_state(Cursor::new(z3)).pop().unwrap(),
        ];
        let new_state = step2(state);

        print_state2(&new_state);
        // assert_eq!(new_state, expected);
    }

    #[test]
    fn test_count_active2() {
        let dirs = (-1..=1)
            .flat_map(|i| {
                (-1..=1)
                    .flat_map(|j| {
                        (-1..=1)
                            .flat_map(|k| {
                                (-1..=1)
                                    .map(|w| (i, j, k, w))
                                    .collect::<Vec<(isize, isize, isize, isize)>>()
                            })
                            .collect::<Vec<(isize, isize, isize, isize)>>()
                    })
                    .collect::<Vec<(isize, isize, isize, isize)>>()
            })
            .filter(|x| !(x.0 == 0 && x.1 == 0 && x.2 == 0 && x.3 == 0));
        assert_eq!(dirs.count(), 80);
    }

    #[test]
    fn test_star_two() {
        let input = b".#.
..#
###";
        assert_eq!(star_two(Cursor::new(input)), 848);
    }
}
