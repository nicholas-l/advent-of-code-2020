use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

// (?P<min>\w{1,}) bags contain 1 bright white bag(s)?, 2 muted yellow bags.

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<amount>\d{1,}) (?P<colour>.+) bag(s)?").unwrap();
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let hashmap = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| {
            let mut iter = x.splitn(2, " bags contain ");
            let parent = iter.next().unwrap();
            let children = iter
                .next()
                .expect(&format!("Expected {} (parent: {})", x, parent));
            (parent.to_string(), children.to_string())
        })
        .filter(|(parent, children)| children != "no other bags.")
        .fold(HashMap::new(), |mut hm, (parent, children)| {
            for child in children.split(",") {
                let captures = RE
                    .captures(&child)
                    .expect("Bad line that does not match regex.");

                hm.entry(captures.name("colour").unwrap().as_str().to_string())
                    .or_insert_with(Vec::new)
                    .push(parent.trim().to_string());
            }
            hm
        });
    let mut stack = Vec::new();
    stack.push("shiny gold");
    let mut visited = HashSet::new();
    while let Some(node) = stack.pop() {
        if !visited.contains(node) {
            visited.insert(node);
            if let Some(parents) = hashmap.get(node) {
                for parent in parents {
                    stack.push(parent)
                }
            }
        }
    }
    visited.len() - 1
}

fn get_number_of_bags(map: &HashMap<String, Vec<(String, usize)>>, id: &str) -> usize {
    match map.get(id) {
        None => 1,
        Some(children) => {
            1_usize
                + children
                    .iter()
                    .map(|(child, number)| number * get_number_of_bags(map, child))
                    .sum::<usize>()
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let hashmap: HashMap<_, Vec<(String, usize)>> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| {
            let mut iter = x.splitn(2, " bags contain ");
            let parent = iter.next().unwrap();
            let children = iter
                .next()
                .expect(&format!("Expected {} (parent: {})", x, parent));
            (parent.trim().to_string(), children.to_string())
        })
        .filter(|(parent, children)| children != "no other bags.")
        .map(|(parent, children)| {
            let children = children
                .split(",")
                .map(|child| {
                    let captures = RE
                        .captures(&child)
                        .expect("Bad line that does not match regex.");

                    (
                        captures.name("colour").unwrap().as_str().to_string(),
                        captures
                            .name("amount")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                    )

                    // .or_insert_with(Vec::new)
                    // .push(parent.trim().to_string());
                })
                .collect::<Vec<_>>();
            (parent, children)
        })
        .collect();
    get_number_of_bags(&hashmap, "shiny gold") - 1
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 4);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 32);
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(star_two(Cursor::new(input)), 126);
    }
}
