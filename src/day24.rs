use std::{collections::HashMap, io::BufRead};

enum Direction {
    // e, se, sw, w, nw, and ne
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_line(line: String) -> Vec<Direction> {
    let mut chars: Vec<char> = line.chars().rev().collect();
    let mut directions = Vec::new();

    while !chars.is_empty() {
        let current = chars.pop().unwrap();
        let current2 = if current == 's' || current == 'n' {
            chars.pop()
        } else {
            None
        };
        let dir = match (current, current2) {
            ('e', None) => Direction::East,
            ('w', None) => Direction::West,
            ('s', Some('e')) => Direction::SouthEast,
            ('s', Some('w')) => Direction::SouthWest,
            ('n', Some('e')) => Direction::NorthEast,
            ('n', Some('w')) => Direction::NorthWest,
            _ => panic!(),
        };
        directions.push(dir);
    }
    directions
}

type Position = (isize, isize);

fn adjacent_positions(current: &Position) -> Vec<Position> {
    vec![
        (current.0, current.1 + 1),
        (current.0 - 1, current.1),
        (current.0 - 1, current.1 - 1),
        (current.0, current.1 - 1),
        (current.0 + 1, current.1),
        (current.0 + 1, current.1 + 1),
    ]
}

fn follow(mut directions: Vec<Direction>) -> Position {
    let mut current = (0, 0);
    while let Some(dir) = directions.pop() {
        current = match dir {
            Direction::East => (current.0, current.1 + 1),
            Direction::SouthEast => (current.0 - 1, current.1),
            Direction::SouthWest => (current.0 - 1, current.1 - 1),
            Direction::West => (current.0, current.1 - 1),
            Direction::NorthWest => (current.0 + 1, current.1),
            Direction::NorthEast => (current.0 + 1, current.1 + 1),
        }
    }
    current
}

#[derive(PartialEq, Clone)]
enum Colour {
    White,
    Black,
}

impl Colour {
    fn flip(&mut self) {
        *self = match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        };
    }
}

fn step(hm: HashMap<Position, Colour>) -> HashMap<Position, Colour> {
    let mut tile_count = HashMap::new();
    for (position, colour) in &hm {
        if colour == &Colour::Black {
            if !tile_count.contains_key(position) {
                tile_count.insert(*position, 0);
            }
            for pos in adjacent_positions(position) {
                *tile_count.entry(pos).or_insert(0) += 1;
            }
        }
    }
    let mut new_tiles = HashMap::new();
    for (position, count) in tile_count {
        match (hm.get(&position).unwrap_or(&Colour::White), count) {
            (Colour::Black, 1..=2) => {
                new_tiles.insert(position, Colour::Black);
            }
            (Colour::White, 2) => {
                new_tiles.insert(position, Colour::Black);
            }
            (Colour::Black, _) => {}
            (Colour::White, _) => {}
        };
    }
    new_tiles
}

pub fn star_one(input: impl BufRead) -> usize {
    let tiles: Vec<Vec<Direction>> = input
        .lines()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
    let mut hm = HashMap::new();
    for directions in tiles {
        let position = follow(directions);
        hm.entry(position).or_insert(Colour::White).flip();
    }

    hm.values()
        .filter(|&colour| colour == &Colour::Black)
        .count()
}

pub fn star_two(input: impl BufRead) -> usize {
    let tiles: Vec<Vec<Direction>> = input
        .lines()
        .filter_map(Result::ok)
        .map(parse_line)
        .collect();
    let mut hm = HashMap::new();
    for directions in tiles {
        let position = follow(directions);
        hm.entry(position).or_insert(Colour::White).flip();
    }
    for _i in 0..100 {
        hm = step(hm);
    }
    hm.values()
        .filter(|&colour| colour == &Colour::Black)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_follow() {
        let input = parse_line("nwwswee".to_string());
        assert_eq!(follow(input), (0, 0));
    }

    #[test]
    fn test_star_one() {
        let input = b"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(star_one(Cursor::new(input)), 10);
    }

    #[test]
    fn test_star_two() {
        let input = b"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
        assert_eq!(star_two(Cursor::new(input)), 2208);
    }
}
