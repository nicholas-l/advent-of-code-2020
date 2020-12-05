use std::io::BufRead;

fn get_path<'a>(
    map: &'a Vec<Vec<char>>,
    delta_y: usize,
    delta_x: usize,
) -> impl Iterator<Item = char> + 'a {
    map.iter()
        .enumerate()
        .filter(move |y| y.0 % delta_y == 0)
        .map(move |(y, row)| row[(y / delta_y * delta_x) % row.len()])
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    (1..map[0].len())
        .map(|movement| get_path(&map, 1, movement).filter(|&x| x == '#').count())
        .max()
        .unwrap()
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let movements = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    movements
        .into_iter()
        .map(|movement| {
            get_path(&map, movement.0, movement.1)
                .filter(|&x| x == '#')
                .count()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8; 131] = b"..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 7);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 336);
    }
}
