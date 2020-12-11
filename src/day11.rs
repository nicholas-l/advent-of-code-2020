use std::fmt::Debug;
use std::{convert::TryFrom, fmt::Display, io::BufRead};

type SeatMap = Vec<Vec<SeatStatus>>;

#[derive(Copy, Clone, PartialEq)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for SeatStatus {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(SeatStatus::Floor),
            'L' => Ok(SeatStatus::Empty),
            '#' => Ok(SeatStatus::Occupied),
            _ => Err(format!("Could not match {} to SeatStatus", c)),
        }
    }
}

impl Display for SeatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SeatStatus::Floor => '.',
            SeatStatus::Empty => 'L',
            SeatStatus::Occupied => '#',
        };
        f.write_fmt(format_args!("{}", c))
    }
}

impl Debug for SeatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            SeatStatus::Floor => '.',
            SeatStatus::Empty => 'L',
            SeatStatus::Occupied => '#',
        };
        f.write_fmt(format_args!("{}", c))
    }
}

fn get_new_state(map: &Vec<Vec<SeatStatus>>, y: usize, x: usize) -> SeatStatus {
    let mut count = 0;
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for (dy, dx) in dirs.iter() {
        // FIXME: has overflow
        let cy = (y as isize + dy) as usize;
        let cx = ((x as isize) + dx) as usize;
        if map
            .get(cy)
            .and_then(|row| row.get(cx))
            .map(|x| match x {
                SeatStatus::Occupied => true,
                _ => false,
            })
            .unwrap_or(false)
        {
            count += 1;
        }
    }

    match (map[y][x], count) {
        (SeatStatus::Empty, 0) => SeatStatus::Occupied,
        (SeatStatus::Occupied, 4..=12) => SeatStatus::Empty,
        _ => map[y][x],
    }
}

fn get_new_state2(map: &Vec<Vec<SeatStatus>>, y: usize, x: usize) -> SeatStatus {
    let count = count_occupied(map, y, x);

    match (map[y][x], count) {
        (SeatStatus::Empty, 0) => SeatStatus::Occupied,
        (SeatStatus::Occupied, 5..=12) => SeatStatus::Empty,
        _ => map[y][x],
    }
}

fn step<T>(map: &SeatMap, get_new_state: T) -> SeatMap  where T: Fn(&SeatMap, usize, usize) -> SeatStatus {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _x)| get_new_state(map, y, x))
                .collect()
        })
        .collect()
}

fn count_occupied(map: &SeatMap, y: usize, x: usize) -> usize {
    let dirs: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let rows = map.len() as isize;
    let cols = map[y].len() as isize;
    dirs.iter()
        .filter(|(dy, dx)| {
            let mut cy = y as isize + dy;
            let mut cx = x as isize + dx;
            while cy >= 0 && cy < rows && cx >= 0 && cx < cols {
                if map[cy as usize][cx as usize] == SeatStatus::Occupied {
                    return true;
                } else if map[cy as usize][cx as usize] == SeatStatus::Empty {
                    return false;
                }
                cy += dy;
                cx += dx;
            }
            false
        })
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let mut map: SeatMap = input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(SeatStatus::try_from)
                .filter_map(Result::ok)
                .collect()
        })
        .collect();
    loop {
        let map2 = step(&map, get_new_state);
        if map == map2 {
            break map
                .iter()
                .map(|row| row.iter().filter(|&&x| x == SeatStatus::Occupied).count())
                .sum();
        }
        map = map2;
    }
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let mut map: SeatMap = input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(SeatStatus::try_from)
                .filter_map(Result::ok)
                .collect()
        })
        .collect();
    loop {
        let map2 = step(&map, get_new_state2);
        if map == map2 {
            break map
                .iter()
                .map(|row| row.iter().filter(|&&x| x == SeatStatus::Occupied).count())
                .sum();
        }
        map = map2;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        count_occupied, get_new_state, get_new_state2, star_one, star_two, step, SeatMap,
        SeatStatus, TryFrom,
    };
    use std::io::Cursor;

    fn get_map(input: &str) -> SeatMap {
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(SeatStatus::try_from)
                    .filter_map(Result::ok)
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_new_state() {
        let expected_map = get_map(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );

        assert_eq!(get_new_state(&expected_map, 0, 2), SeatStatus::Empty);
    }

    #[test]
    fn test_step() {
        let map = get_map(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );

        let expected_map = get_map(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );

        assert_eq!(get_new_state(&expected_map, 0, 2), SeatStatus::Empty);
        assert_eq!(step(&map, get_new_state), expected_map);

        let expected_map2 = get_map(
            "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        );
        assert_eq!(step(&expected_map, get_new_state), expected_map2);
    }

    #[test]
    fn test_star_one() {
        let input = b"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(star_one(Cursor::new(input)), 37);
    }

    #[test]
    fn test_count_occupied() {
        let map = get_map(
            ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
        );
        assert_eq!(count_occupied(&map, 4, 3), 8)
    }

    #[test]
    fn test_step2() {
        let map = get_map(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );
        let map2 = get_map(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );
        let map3 = get_map(
            "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        );

        assert_eq!(step(&map, get_new_state2), map2);
        assert_eq!(step(&step(&map, get_new_state2), get_new_state2), map3);
    }

    #[test]
    fn test_get_new_state2() {
        let map = get_map(
            "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        );

        assert_eq!(get_new_state2(&map, 1, 9), SeatStatus::Empty);
    }

    #[test]
    fn test_star_two() {
        let input = b"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(star_two(Cursor::new(input)), 26);
    }
}