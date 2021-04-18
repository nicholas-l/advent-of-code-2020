use std::io::BufRead;
use std::{fmt::Debug, str::FromStr};

#[derive(Copy, Clone, PartialEq)]
enum Action {
    North(usize),   // Action N means to move north by the given value.
    South(usize),   // Action S means to move south by the given value.
    East(usize),    // Action E means to move east by the given value.
    West(usize),    // Action W means to move west by the given value.
    Left(usize),    // Action L means to turn left the given number of degrees.
    Right(usize),   // Action R means to turn right the given number of degrees.
    Forward(usize), // Action F means to move forward by the given value in the direction the ship is currently facing.
}

impl FromStr for Action {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let instruction = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<usize>().unwrap();
        match instruction {
            'N' => Ok(Action::North(value)),
            'S' => Ok(Action::South(value)),
            'E' => Ok(Action::East(value)),
            'W' => Ok(Action::West(value)),
            'L' => Ok(Action::Left(value)),
            'R' => Ok(Action::Right(value)),
            'F' => Ok(Action::Forward(value)),
            _ => Err(format!("Unknown instruction: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    direction: isize,
}

impl Ship {
    fn step(&mut self, action: Action) {
        match action {
            Action::North(y) => self.y += y as isize,
            Action::South(y) => self.y -= y as isize,
            Action::East(x) => self.x += x as isize, // Action E means to move east by the given value.
            Action::West(x) => self.x -= x as isize, // Action W means to move west by the given value.
            Action::Left(dir) => {
                // Action L means to turn left the given number of degrees.
                self.direction = self.direction - dir as isize;
                if self.direction < 0 {
                    self.direction = 360 + self.direction
                }
            }
            Action::Right(dir) => {
                // Action R means to turn right the given number of degrees.
                self.direction += dir as isize;
                if self.direction >= 360 {
                    self.direction = self.direction - 360
                }
            }
            Action::Forward(value) => match self.direction {
                0 => self.y += value as isize,
                90 => self.x += value as isize,
                180 => self.y -= value as isize,
                270 => self.x -= value as isize,
                _ => panic!("bad direction {}", self.direction),
            },
        };
    }

    fn step2(&mut self, action: Action, waypoint: (isize, isize)) -> (isize, isize) {
        match action {
            Action::North(y) => (waypoint.0 + y as isize, waypoint.1),
            Action::South(y) => (waypoint.0 - y as isize, waypoint.1),
            Action::East(x) => (waypoint.0, waypoint.1 + x as isize), // Action E means to move east by the given value.
            Action::West(x) => (waypoint.0, waypoint.1 - x as isize), // Action W means to move west by the given value.
            Action::Left(dir) => {
                // Action L means to turn left the given number of degrees.
                let turns = dir / 90;
                let mut y = waypoint.0;
                let mut x = waypoint.1;
                for _ in 0..turns {
                    let tmp = (x, -y);
                    y = tmp.0;
                    x = tmp.1;
                }
                (y, x)
            }
            Action::Right(dir) => {
                // Action R means to turn right the given number of degrees.
                let turns = dir / 90;
                let mut y = waypoint.0;
                let mut x = waypoint.1;
                for _ in 0..turns {
                    let tmp = (-x, y);
                    y = tmp.0;
                    x = tmp.1;
                }
                (y, x)
            }
            Action::Forward(value) => {
                let x = waypoint.1;
                let y = waypoint.0;
                self.x = self.x + x * value as isize;
                self.y = self.y + y * value as isize;
                waypoint
            }
        }
    }

    fn manhatten_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: 90,
    };
    for action in input
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<Action>().unwrap())
    {
        ship.step(action);
    }
    ship.manhatten_distance()
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut ship = Ship {
        x: 0,
        y: 0,
        direction: 90,
    };
    let mut waypoint = (1, 10);
    for action in input
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.parse::<Action>().unwrap())
    {
        waypoint = ship.step2(action, waypoint);
    }
    ship.manhatten_distance()
}

#[cfg(test)]
mod tests {

    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"F10
N3
F7
R90
F11";
        assert_eq!(star_one(Cursor::new(input)), 25);
    }

    #[test]
    fn test_star_two() {
        let input = b"F10
N3
F7
R90
F11";
        assert_eq!(star_two(Cursor::new(input)), 286);
    }
}
