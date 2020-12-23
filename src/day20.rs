use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fmt::Display,
    hash::Hash,
    io::BufRead,
    str::FromStr,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Tile (?P<id>\d+):").unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Pixel {
    Hash,
    Dot,
}

impl TryFrom<char> for Pixel {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Pixel::Dot),
            '#' => Ok(Pixel::Hash),
            _ => Err(format!("Could not match {} to SeatStatus", c)),
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pixel::Hash => '#',
            Pixel::Dot => '.',
        };
        f.write_fmt(format_args!("{}", c))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    left: Edge,
    right: Edge,
    top: Edge,
    bottom: Edge,
    matrix: Vec<Vec<Pixel>>,
}

impl Tile {
    fn rotate(&self) -> Tile {
        let mut top = self.left.clone();
        top.reverse();
        let mut bottom = self.right.clone();
        bottom.reverse();
        let matrix = matrix_rotate(&self.matrix);
        Tile {
            id: self.id,
            top,
            right: self.top.clone(),
            bottom,
            left: self.bottom.clone(),
            matrix,
        }
    }

    fn flip(&self, axis: usize) -> Tile {
        match axis {
            0 => {
                let mut right = self.right.clone();
                right.reverse();
                let mut left = self.left.clone();
                left.reverse();

                let matrix = matrix_flip(&self.matrix, axis);
                Tile {
                    id: self.id,
                    top: self.bottom.clone(),
                    bottom: self.top.clone(),
                    right,
                    left,
                    matrix,
                }
            }
            1 => {
                let mut top = self.top.clone();
                top.reverse();
                let mut bottom = self.bottom.clone();
                bottom.reverse();
                let matrix = matrix_flip(&self.matrix, axis);
                Tile {
                    id: self.id,
                    top,
                    bottom,
                    right: self.left.clone(),
                    left: self.right.clone(),
                    matrix,
                }
            }
            x => panic!("Could not rotate about axis {}", x),
        }
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let captures = RE
            .captures(&lines.next().unwrap())
            .expect("Bad line that does not match regex.");
        let id = captures["id"].parse::<usize>().unwrap();
        let matrix: Vec<Vec<Pixel>> = lines
            .map(|line| line.chars().map(|c| Pixel::try_from(c).unwrap()).collect())
            .collect();
        // number_of_tiles += 1;
        let top = matrix[0].clone();
        let bottom = matrix.last().unwrap().clone();
        let left = matrix.iter().map(|r| r[0]).collect();
        let right = matrix.iter().map(|r| *r.last().unwrap()).collect();
        Ok(Tile {
            id,
            left,
            right,
            top,
            bottom,
            matrix,
        })
    }
}

fn matrix_flip(matrix: &Matrix, axis: usize) -> Vec<Vec<Pixel>> {
    match axis {
        0 => matrix.iter().rev().cloned().collect(),
        1 => matrix
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect(),
        x => panic!("Could not flip about axis {}", x),
    }
}

fn matrix_rotate(matrix: &Vec<Vec<Pixel>>) -> Vec<Vec<Pixel>> {
    let mut new_matrix = matrix.clone();
    let matrix_height = matrix.len();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            new_matrix[j][i] = matrix[matrix_height - 1 - i][j];
        }
    }
    new_matrix
}

fn matrix_rotations(matrix: Matrix) -> Vec<Matrix> {
    let mut rotations = Vec::with_capacity(4);
    rotations.push(matrix);
    for _ in 0..3 {
        rotations.push(matrix_rotate(rotations.last().unwrap()))
    }
    rotations
}

fn matrix_flips(matrix: Matrix) -> Vec<Matrix> {
    let mut flips = Vec::with_capacity(4);
    let flip_h = matrix_flip(&matrix, 0);
    let flip_v = matrix_flip(&matrix, 1);
    // let flip_hv = flip_h.flip(1);
    flips.push(matrix);
    flips.push(flip_h);
    flips.push(flip_v);
    // flips.push(flip_hv);
    flips
}

fn matrix_transformations(matrix: Vec<Vec<Pixel>>) -> impl Iterator<Item = Matrix> {
    matrix_flips(matrix).into_iter().flat_map(matrix_rotations)
}

type Position = (isize, isize);
type Map<'a> = HashMap<Position, Tile>;
type Edge = Vec<Pixel>;
type Matrix = Vec<Vec<Pixel>>;

fn tile_rotations(tile: Tile) -> Vec<Tile> {
    let mut rotations = Vec::with_capacity(4);
    rotations.push(tile);
    for _ in 0..3 {
        rotations.push(rotations.last().unwrap().rotate())
    }
    rotations
}

fn tile_flips(tile: Tile) -> Vec<Tile> {
    let mut flips = Vec::with_capacity(4);
    let flip_h = tile.flip(0);

    flips.push(tile);
    flips.push(flip_h);
    flips
}

fn transformations(tile: Tile) -> impl Iterator<Item = Tile> {
    tile_flips(tile).into_iter().flat_map(tile_rotations)
}

fn get_neighbour_ids(map: &Map, pos: &Position) -> Vec<usize> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|dir| map.get(&(pos.0 + dir.0, pos.1 + dir.1)))
        .map(|x| x.id)
        .collect()
}

fn is_valid_in_position(map: &Map, pos: &Position, tile: &Tile) -> bool {
    // Left
    map.get(&(pos.0, pos.1 - 1)).map(|left| left.right == tile.left).unwrap_or(true) &&
    // Right
    map.get(&(pos.0, pos.1 + 1)).map(|right| right.left == tile.right).unwrap_or(true) &&

    map.get(&(pos.0 - 1, pos.1)).map(|above| above.bottom == tile.top).unwrap_or(true) &&
    map.get(&(pos.0 + 1, pos.1)).map(|below| below.top == tile.bottom).unwrap_or(true)
}

fn solve<'a>(
    map: &mut Map<'a>,
    tile_to_tile: &'a HashMap<usize, HashSet<usize>>,
    tiles: &'a HashMap<usize, Tile>,
    done: &mut HashSet<usize>,
    width: usize,
    depth: usize,
    extra: usize,
) -> bool {
    if depth == width * width {
        println!("Found!");
        return true;
    } else {
        let position = ((depth / width) as isize, (depth % width) as isize);
        // println!("{}: Checking {}, {:?}", extra, width, depth);
        // let tmp = map.clone();
        let ids: HashSet<_> = get_neighbour_ids(&map, &position)
            .iter()
            .map(|id| &tile_to_tile[id])
            .fold(None, |acc: Option<HashSet<_>>, neighbours| {
                if let Some(acc) = acc {
                    Some(
                        acc.intersection(neighbours)
                            .copied()
                            .collect::<HashSet<usize>>(),
                    )
                } else {
                    Some(neighbours.clone())
                }
            })
            .unwrap();
        ids.into_iter()
            .map(|id| tiles.get(&id).unwrap())
            .flat_map(|tile| transformations(tile.clone()))
            .any(|tile| {
                if !done.contains(&tile.id) && is_valid_in_position(map, &position, &tile) {
                    // Potentially dont copy map instead add and remove.
                    let id = tile.id;
                    done.insert(id);
                    map.insert(position, tile);
                    if solve(map, tile_to_tile, tiles, done, width, depth + 1, extra) {
                        return true;
                    }
                    map.remove(&position);
                    done.remove(&id);
                }
                false
            })
    }
}

fn parse_tiles(input_str: &String) -> impl Iterator<Item = Tile> + '_ {
    input_str
        .split("\n\n")
        .map(|tile_str| tile_str.parse::<Tile>().unwrap())
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut tiles: Vec<Tile> = parse_tiles(&input_str)
        .flat_map(|tile| transformations(tile))
        .collect();
    println!("{}", tiles.len());
    tiles.dedup();
    println!("{}", tiles.len());
    let mut edge_to_tile_id = HashMap::new();
    for t in &tiles {
        edge_to_tile_id
            .entry(&t.left)
            .or_insert_with(Vec::new)
            .push(t);
        edge_to_tile_id
            .entry(&t.right)
            .or_insert_with(Vec::new)
            .push(t);
        edge_to_tile_id
            .entry(&t.top)
            .or_insert_with(Vec::new)
            .push(t);
        edge_to_tile_id
            .entry(&t.bottom)
            .or_insert_with(Vec::new)
            .push(t);
    }

    let mut tile_tile: HashMap<usize, HashSet<usize>> = HashMap::new();

    for tile in &tiles {
        for x in &edge_to_tile_id[&tile.left] {
            if x.id != tile.id {
                tile_tile
                    .entry(tile.id)
                    .or_insert_with(HashSet::new)
                    .insert(x.id);
            }
        }
    }

    for tile in &tile_tile {
        println!("{:?}", tile);
    }

    return tile_tile
        .iter()
        .filter(|(_key, value)| value.len() == 2)
        .map(|(key, _value)| *key)
        .product();
}

fn convert_to_matrix(map: &Map, width: usize) -> Vec<Vec<Pixel>> {
    let top_left_tile = &map.get(&(0, 0)).unwrap();
    let tile_height = top_left_tile.matrix.len();
    let tile_width = top_left_tile.matrix[0].len();
    let mut vec = Vec::new();
    for i in 0..(width * tile_height) {
        if i % tile_height != 0 && i % tile_height != tile_height - 1 {
            let mut row = Vec::with_capacity(width);
            for j in 0..(width * tile_width) {
                if j % tile_width != 0 && j % tile_width != tile_width - 1 {
                    row.push(get_position(map, tile_width, &(i as isize, j as isize)));
                }
            }
            vec.push(row);
        }
    }
    vec
}

fn search(matrix: &Matrix) -> usize {
    let sea_monster = vec![
        "                  # ".chars().collect::<Vec<char>>(),
        "#    ##    ##    ###".chars().collect::<Vec<char>>(),
        " #  #  #  #  #  #   ".chars().collect::<Vec<char>>(),
    ];
    let mut count = 0;
    for i in 0..(matrix.len() - sea_monster.len()) {
        for j in 0..(matrix[i].len() - sea_monster[0].len()) {
            if (0..sea_monster.len()).all(|y| {
                (0..sea_monster[y].len()).all(|x| {
                    sea_monster[y][x] != '#'
                        || sea_monster[y][x] == '#' && matrix[i + y][j + x] == Pixel::Hash
                })
            }) {
                count += 1;
            }
        }
    }
    count
}

fn print_tile(tile: &Tile) {
    for row in &tile.matrix {
        for x in row {
            print!("{}", x);
        }
        println!("");
    }
}

fn print_image(image: &Vec<Vec<Pixel>>) {
    for row in image {
        for p in row {
            match p {
                Pixel::Hash => print!("#"),
                Pixel::Dot => print!("."),
            }
        }
        println!("")
    }
}

fn print_map(map: &Map, width: usize) {
    for i in 0..width {
        for j in 0..width {
            print!("{} ", map.get(&(i as isize, j as isize)).unwrap().id);
        }
        println!("");
    }
}

fn get_position(map: &Map, tile_width: usize, pos: &Position) -> Pixel {
    let tile_width = tile_width as isize;
    let i = pos.0 / tile_width;
    let j = pos.1 / tile_width;
    let tile = map.get(&(i as isize, j as isize)).unwrap();
    tile.matrix[(pos.0 - i * tile_width) as usize][(pos.1 - j * tile_width) as usize]
}

fn print_map_content(map: &Map, width: usize, tile_width: usize) {
    for i in 0..(width * tile_width) {
        if i % tile_width == 0 {
            println!("")
        }
        println!(" ");
        for j in 0..(width * tile_width) {
            if j % tile_width == 0 {
                print!(" ")
            }
            print!(
                "{}",
                get_position(&map, tile_width, &(i as isize, j as isize))
            );
        }
    }
    println!("")
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut number_of_tiles = 0;
    let tiles: HashMap<usize, Tile> = parse_tiles(&input_str)
        .map(|tile| {
            number_of_tiles += 1;
            (tile.id, tile)
        })
        .collect();
    let mut edge_to_tile_id = HashMap::new();
    for (_id, tile) in &tiles {
        for t in transformations(tile.clone()) {
            edge_to_tile_id
                .entry(t.left)
                .or_insert_with(HashSet::new)
                .insert(t.id);
            edge_to_tile_id
                .entry(t.right)
                .or_insert_with(HashSet::new)
                .insert(t.id);
            edge_to_tile_id
                .entry(t.top)
                .or_insert_with(HashSet::new)
                .insert(t.id);
            edge_to_tile_id
                .entry(t.bottom)
                .or_insert_with(HashSet::new)
                .insert(t.id);
        }
    }

    let mut tile_to_tile: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (_id, tile) in &tiles {
        for t in transformations(tile.clone()) {
            for id in &edge_to_tile_id[&t.left] {
                if id != &t.id {
                    tile_to_tile
                        .entry(t.id)
                        .or_insert_with(HashSet::new)
                        .insert(*id);
                }
            }
        }
    }

    let (first_corner, first_neighbours) = tile_to_tile
        .iter()
        .filter(|(_id, value)| value.len() == 2)
        .next()
        .unwrap();

    // println!("{}", tiles.len());

    let width = (number_of_tiles as f64).sqrt() as usize;
    println!("{}: {}", width, number_of_tiles);

    let first_neighbours: Vec<&usize> = first_neighbours.iter().collect();

    println!(
        "{:?}",
        transformations(tiles.get(first_corner).unwrap().clone())
            .filter(|tile| {
                edge_to_tile_id[&tile.right]
                    .iter()
                    .find(|id| id == &first_neighbours[0])
                    .is_some()
                    && edge_to_tile_id[&tile.bottom]
                        .iter()
                        .find(|id| id == &first_neighbours[1])
                        .is_some()
            })
            .map(|x| x.id)
            .collect::<Vec<usize>>()
    );

    let map = transformations(tiles.get(first_corner).unwrap().clone())
        .filter(|tile| {
            edge_to_tile_id[&tile.right]
                .iter()
                .find(|id| id == &first_neighbours[0])
                .is_some()
                && edge_to_tile_id[&tile.bottom]
                    .iter()
                    .find(|id| id == &first_neighbours[1])
                    .is_some()
        })
        .find_map(|tile| {
            println!("Starting tile: {:?}", tile.id);
            print_tile(&tile);
            println!("{:?}", tile.left);
            println!("{:?}", tile.right);
            println!("{:?}", edge_to_tile_id[&tile.right]);
            let mut hs = HashSet::new();
            hs.insert(tile.id);
            println!(
                "{:?}",
                edge_to_tile_id[&tile.right]
                    .difference(&hs)
                    .copied()
                    .collect::<HashSet<usize>>()
            );
            let mut map = HashMap::new();
            map.insert((0, 0), tile.clone());
            let mut done = HashSet::new();
            done.insert(tile.id);
            if solve(
                &mut map,
                &tile_to_tile,
                &tiles,
                &mut done,
                width,
                1,
                tile.id,
            ) {
                Some(map)
            } else {
                None
            }
        })
        .unwrap();
    let first_corner = map.get(&(0, 0)).unwrap();
    let tile_width = first_corner.matrix.len();

    print_tile(&first_corner);

    print_map(&map, width);

    print_map_content(&map, width, tile_width);
    let image = convert_to_matrix(&map, width);
    println!("");
    print_image(&image);
    println!("");

    let hashes: usize = image
        .iter()
        .map(|row| row.iter().filter(|c| c == &&Pixel::Hash).count())
        .sum();

    let sea_monster_count = matrix_transformations(image)
        .find_map(|matrix| {
            let count = search(&matrix);
            if count > 0 {
                print_image(&matrix);
                Some(count)
            } else {
                None
            }
        })
        .unwrap();

    hashes - sea_monster_count * 15
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = include_bytes!("day20_test.txt");
        assert_eq!(star_one(Cursor::new(input)), 20899048083289);
    }

    #[test]
    fn test_matrix_flip() {
        {
            let input = vec![vec![Pixel::Hash, Pixel::Dot], vec![Pixel::Dot, Pixel::Hash]];
            let expected = vec![vec![Pixel::Dot, Pixel::Hash], vec![Pixel::Hash, Pixel::Dot]];
            assert_eq!(matrix_flip(&input, 0), expected);
            assert_eq!(matrix_flip(&input, 1), expected);
        }
        {
            let input = vec![
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
                vec![Pixel::Dot, Pixel::Hash, Pixel::Hash],
                vec![Pixel::Hash, Pixel::Dot, Pixel::Dot],
            ];
            let expected1 = vec![
                vec![Pixel::Hash, Pixel::Dot, Pixel::Dot],
                vec![Pixel::Dot, Pixel::Hash, Pixel::Hash],
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
            ];
            let expected2 = vec![
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
                vec![Pixel::Hash, Pixel::Hash, Pixel::Dot],
                vec![Pixel::Dot, Pixel::Dot, Pixel::Hash],
            ];
            assert_eq!(matrix_flip(&input, 0), expected1);
            assert_eq!(matrix_flip(&input, 1), expected2);
        }
    }

    #[test]
    fn test_matrix_rotate() {
        {
            let input = vec![vec![Pixel::Hash, Pixel::Dot], vec![Pixel::Dot, Pixel::Hash]];
            let expected = vec![vec![Pixel::Dot, Pixel::Hash], vec![Pixel::Hash, Pixel::Dot]];
            assert_eq!(matrix_rotate(&input), expected);
        }
        {
            let input = vec![
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
                vec![Pixel::Dot, Pixel::Hash, Pixel::Hash],
                vec![Pixel::Hash, Pixel::Dot, Pixel::Dot],
            ];
            let expected1 = vec![
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
                vec![Pixel::Dot, Pixel::Hash, Pixel::Dot],
                vec![Pixel::Dot, Pixel::Hash, Pixel::Hash],
            ];
            let expected2 = vec![
                vec![Pixel::Dot, Pixel::Dot, Pixel::Hash],
                vec![Pixel::Hash, Pixel::Hash, Pixel::Dot],
                vec![Pixel::Hash, Pixel::Dot, Pixel::Hash],
            ];
            let output = matrix_rotate(&input);
            assert_eq!(output, expected1);
            assert_eq!(matrix_rotate(&output), expected2);
        }
    }

    #[test]
    fn test_parse_tile() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###";
        let tile = input.parse::<Tile>().unwrap();
        assert_eq!(tile.id, 2311);
    }

    #[test]
    fn test_star_two() {
        let input = include_bytes!("day20_test.txt");
        assert_eq!(star_two(Cursor::new(input)), 273);
    }
}
