use std::io::BufRead;

fn get_path<'a, 'b>(map: &'a Vec<Vec<char>>, movement: &'b (usize, usize)) -> Vec<char> {
    map.iter()
        .enumerate()
        .filter(|(y, _)| y % movement.0 == 0)
        .map(|(y, row)| row[(y / movement.0 * movement.1) % row.len()])
        .collect()
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    (1..map[0].len())
        .map(|movement| {
            let movement = (1, movement);
            get_path(&map, &movement)
                .into_iter()
                .filter(|&x| x == '#')
                .count()
        })
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
            get_path(&map, &movement)
                .into_iter()
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
