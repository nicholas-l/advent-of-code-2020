use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Seat(usize, usize);

impl Seat {
    fn get_id(&self) -> usize {
        self.0 * 8 + self.1
    }
}

impl FromStr for Seat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = s.chars().take(7).fold((0, 127), |(low, high), c| {
            let mid = low + (high - low) / 2;
            match c {
                'F' => (low, mid),
                'B' => (mid + 1, high),
                _ => unreachable!(),
            }
        });
        let column = s.chars().skip(7).fold((0, 7), |(low, high), c| {
            let mid = low + (high - low) / 2;
            match c {
                'L' => (low, mid),
                'R' => (mid + 1, high),
                _ => unreachable!(),
            }
        });
        Ok(Seat(row.0, column.0))
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    input
        .lines()
        .filter_map(|s| s.unwrap().parse::<Seat>().ok())
        .map(|seat| seat.get_id())
        .max()
        .unwrap()
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut sorted_seats: Vec<usize> = input
        .lines()
        .map(|s| s.unwrap().parse::<Seat>().unwrap())
        .map(|seat| seat.get_id())
        .collect();
    sorted_seats.sort_unstable();

    sorted_seats
        .windows(2)
        .find(|window| window[0] + 1 != window[1])
        .unwrap()[0]
        + 1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, Seat};
    use std::io::Cursor;

    #[test]
    fn test_parsing_seat() {
        assert_eq!("FBFBBFFRLR".parse::<Seat>(), Ok(Seat(44, 5)));
        assert_eq!("BFFFBBFRRR".parse::<Seat>(), Ok(Seat(70, 7)));
        assert_eq!("FFFBBBFRRR".parse::<Seat>(), Ok(Seat(14, 7)));
        assert_eq!("BBFFBBFRLL".parse::<Seat>(), Ok(Seat(102, 4)));
    }

    #[test]
    #[ignore]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new("INPUT")), 7);
    }

    #[test]
    #[ignore]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new("INPUT")), 336);
    }
}
