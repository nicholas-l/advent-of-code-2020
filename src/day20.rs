use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    io::BufRead,
};

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Tile (?P<id>\d+):").unwrap();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Pixel {
    Hash,
    Dot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    left: Vec<Pixel>,
    right: Vec<Pixel>,
    top: Vec<Pixel>,
    bottom: Vec<Pixel>,
}

impl Tile {
    fn rotate(&self) -> Tile {
        let mut top = self.left.clone();
        top.reverse();
        let mut bottom = self.right.clone();
        bottom.reverse();
        Tile {
            id: self.id,
            top,
            right: self.top.clone(),
            bottom,
            left: self.bottom.clone(),
        }
    }

    fn flip(&self, axis: usize) -> Tile {
        match axis {
            0 => {
                let mut right = self.right.clone();
                right.reverse();
                let mut left = self.left.clone();
                left.reverse();
                Tile {
                    id: self.id,
                    top: self.bottom.clone(),
                    bottom: self.top.clone(),
                    right,
                    left,
                }
            }
            1 => {
                let mut top = self.top.clone();
                top.reverse();
                let mut bottom = self.bottom.clone();
                bottom.reverse();
                Tile {
                    id: self.id,
                    top,
                    bottom,
                    right: self.left.clone(),
                    left: self.right.clone(),
                }
            }
            x => panic!("Could not rotate about axis {}", x),
        }
    }
}

type Position = (isize, isize);
type Map<'a> = HashMap<Position, &'a Tile>;
type Edge = Vec<Pixel>;

fn tile_rotations(tile: Tile) -> Vec<Tile> {
    let mut rotations = Vec::with_capacity(4);
    rotations.push(tile);
    for _ in 0..3 {
        rotations.push(rotations.last().unwrap().rotate())
    }
    rotations
}

fn get_surrounding_edges<'a>(map: &'a Map, pos: &Position) -> Vec<&'a Edge> {
    let mut v = Vec::new();
    // Left
    if let Some(x) = map.get(&(pos.0, pos.1 - 1)) {
        v.push(&x.right)
    }

    if let Some(x) = map.get(&(pos.0, pos.1 + 1)) {
        v.push(&x.left)
    }

    if let Some(x) = map.get(&(pos.0 - 1, pos.1)) {
        v.push(&x.bottom)
    }

    if let Some(x) = map.get(&(pos.0 + 1, pos.1)) {
        v.push(&x.top)
    }
    v
}

fn tile_flips(tile: Tile) -> Vec<Tile> {
    let mut flips = Vec::with_capacity(4);
    let flip_h = tile.flip(0);
    // let flip_v = tile.flip(1);
    // let flip_hv = flip_h.flip(1);
    flips.push(tile);
    flips.push(flip_h);
    // flips.push(flip_v);
    // flips.push(flip_hv);
    flips
}

fn transformations(tile: Tile) -> impl Iterator<Item = Tile> {
    tile_flips(tile).into_iter().flat_map(tile_rotations)
}

fn get_tiles_from_edge<'a>(
    edge_to_tile_id: &'a HashMap<&'a Edge, Vec<&Tile>>,
    edge: &Edge,
) -> &'a Vec<&'a Tile> {
    edge_to_tile_id.get(edge).unwrap()
}

fn get_next_positions<'a>(width: usize) -> impl Iterator<Item = Position> + 'a {
    (0..width).flat_map(move |i| (0..width).map(move |j| (i as isize, j as isize)))
}

fn get_neighbour_ids(map: &Map, pos: Position) -> Vec<usize> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|dir| map.get(&(pos.0 + dir.0, pos.1 + dir.1)))
        .map(|x| x.id)
        .collect()
}

fn solve<'a>(
    map: &mut Map<'a>,
    tile_to_tile: &'a HashMap<usize, HashSet<usize>>,
    tiles: &'a Vec<Tile>,
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
        let ids: HashSet<_> = get_neighbour_ids(&map, position)
            .iter()
            .map(|id| &tile_to_tile[id])
            .fold(None, |acc: Option<HashSet<_>>, neighbours| {
              if let Some(acc) = acc {
                Some(acc.intersection(neighbours).copied().collect::<HashSet<usize>>())
              } else {
                Some(neighbours.clone())
              }
            })
            .unwrap();
        tiles
            .iter()
            .filter(|tile| ids.contains(&tile.id))
            .any(|tile| {
                if !done.contains(&tile.id) {
                    // Potentially dont copy map instead add and remove.
                    map.insert(position, tile);
                    done.insert(tile.id);
                    if solve(map, tile_to_tile, tiles, done, width, depth + 1, extra) {
                        return true;
                    }
                    done.remove(&tile.id);
                    map.remove(&position);
                }
                false
            })
    }
}

#[allow(dead_code, unused_variables)]
pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut number_of_tiles = 0;
    let mut tiles : Vec<Tile> = input_str
        .split("\n\n")
        .map(|tile_str| {
            let mut lines = tile_str.lines();
            let captures = RE
                .captures(&lines.next().unwrap())
                .expect("Bad line that does not match regex.");
            let id = captures["id"].parse::<usize>().unwrap();
            let matrix: Vec<Vec<Pixel>> = lines
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Pixel::Hash,
                            '.' => Pixel::Dot,
                            x => panic!("Unknown character: {}", x),
                        })
                        .collect()
                })
                .collect();
            number_of_tiles += 1;
            let top = matrix[0].clone();
            let bottom = matrix.last().unwrap().clone();
            let left = matrix.iter().map(|r| r[0]).collect();
            let right = matrix.iter().map(|r| *r.last().unwrap()).collect();
            Tile {
                id,
                left,
                right,
                top,
                bottom,
            }
        })
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
        .filter(|(key, value)| value.len() == 2)
        .map(|x| *x.0)
        .product();
}

#[allow(dead_code, unused_variables)]
pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut number_of_tiles = 0;
    let mut tiles: Vec<Tile> = input_str
        .split("\n\n")
        .map(|tile_str| {
            let mut lines = tile_str.lines();
            let captures = RE
                .captures(&lines.next().unwrap())
                .expect("Bad line that does not match regex.");
            let id = captures["id"].parse::<usize>().unwrap();
            let matrix: Vec<Vec<Pixel>> = lines
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '#' => Pixel::Hash,
                            '.' => Pixel::Dot,
                            x => panic!("Unknown character: {}", x),
                        })
                        .collect()
                })
                .collect();
            number_of_tiles += 1;
            let top = matrix[0].clone();
            let bottom = matrix.last().unwrap().clone();
            let left = matrix.iter().map(|r| r[0]).collect();
            let right = matrix.iter().map(|r| *r.last().unwrap()).collect();
            Tile {
                id,
                left,
                right,
                top,
                bottom,
            }
        })
        .flat_map(|tile| transformations(tile))
        .collect();
    tiles.dedup();
    let mut edge_to_tile = HashMap::new();
    for t in &tiles {
        edge_to_tile.entry(&t.left).or_insert_with(Vec::new).push(t);
        edge_to_tile
            .entry(&t.right)
            .or_insert_with(Vec::new)
            .push(t);
        edge_to_tile.entry(&t.top).or_insert_with(Vec::new).push(t);
        edge_to_tile
            .entry(&t.bottom)
            .or_insert_with(Vec::new)
            .push(t);
    }

    let mut tile_tile: HashMap<usize, HashSet<usize>> = HashMap::new();

    for tile in &tiles {
        for x in &edge_to_tile[&tile.left] {
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

    let (first_corner, first_neighbours) = tile_tile
        .iter()
        .filter(|(key, value)| value.len() == 2)
        .next()
        .unwrap();

    // println!("{}", tiles.len());

    let width = (number_of_tiles as f64).sqrt() as usize;
    println!("{}: {}", width, number_of_tiles);

    let first_neighbours: Vec<&usize> = first_neighbours.iter().collect();

    let map = tiles
        .iter()
        .filter(|tile| tile.id == *first_corner)
        .filter(|tile| {
            edge_to_tile[&tile.right]
                .iter()
                .find(|x| x.id == *first_neighbours[0])
                .is_some()
                && edge_to_tile[&tile.bottom]
                    .iter()
                    .find(|x| x.id == *first_neighbours[1])
                    .is_some()
        })
        .find_map(|tile| {
            let mut map = HashMap::new();
            map.insert((0, 0), tile);
            let mut done = HashSet::new();
            done.insert(tile.id);
            if solve(&mut map, &tile_tile, &tiles, &mut done, width, 1, tile.id) {
                Some(map)
            } else {
                None
            }
        })
        .unwrap();

    println!("{:?}", map);
    map.get(&(0, 0)).unwrap().id
        * map.get(&(0, width as isize - 1)).unwrap().id
        * map.get(&(width as isize - 1, 0)).unwrap().id
        * map
            .get(&(width as isize - 1, width as isize - 1))
            .unwrap()
            .id
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
    fn test_star_two() {
        let input = include_bytes!("day20_test.txt");
        assert_eq!(star_two(Cursor::new(input)), 273);
    }
}
