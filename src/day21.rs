use regex::Regex;
use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, HashMap, HashSet},
    io::BufRead,
};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?P<ingredients>[\w ]+) \(contains (?P<allergens>[\w ,]+)").unwrap();
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut a = HashSet::new();
    let mut i = HashSet::new();
    let data: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let captures = RE.captures(&line).unwrap();
            let allergens = captures["allergens"]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let ingredients = captures["ingredients"]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            (ingredients, allergens)
        })
        .collect();
    let mut hm: HashMap<String, HashSet<&String>> = HashMap::new();
    for (ingredients, allergens) in &data {
        for allergen in allergens {
            let hs: HashSet<_> = ingredients.iter().collect();
            match hm.entry(allergen.to_string()) {
                Entry::Occupied(mut e) => {
                    let v = e.get();
                    let v = v.intersection(&hs).copied().collect();
                    e.insert(v);
                }
                Entry::Vacant(o) => {
                    o.insert(hs);
                }
            }

            for ingredient in ingredients {
                i.insert(ingredient.to_string());
            }
            a.insert(allergen.to_string());
        }
        // println!("{:?}", hm);
    }
    let mut possible_labels: Vec<(String, HashSet<&String>)> = hm.into_iter().collect();
    possible_labels.sort_by_key(|x| Reverse(x.1.len()));

    let mut labels = HashMap::new();
    while !possible_labels.is_empty() {
        let (allergen, possible_ing) = possible_labels.pop().unwrap();
        if possible_ing.len() != 1 {
            panic!(
                "Possible ingredients are not 1!: {:?}: {:?} ({:?})",
                allergen, possible_ing, possible_labels
            );
        }
        let v: Vec<&String> = possible_ing.into_iter().collect();
        labels.insert(v[0], allergen);
        for x in &mut possible_labels {
            x.1.remove(v[0]);
        }
        possible_labels.sort_by_key(|x| Reverse(x.1.len()));
    }
    println!("{:?}", labels);
    let mut count = 0;
    for (ingredients, _allergens) in &data {
        for ingredient in ingredients {
            if !labels.contains_key(ingredient) {
                count += 1
            }
        }
    }

    count
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut a = HashSet::new();
    let mut i = HashSet::new();
    let data: Vec<(Vec<String>, Vec<String>)> = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let captures = RE.captures(&line).unwrap();
            let allergens = captures["allergens"]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            let ingredients = captures["ingredients"]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            (ingredients, allergens)
        })
        .collect();
    let mut hm: HashMap<String, HashSet<&String>> = HashMap::new();
    for (ingredients, allergens) in &data {
        for allergen in allergens {
            let hs: HashSet<_> = ingredients.iter().collect();
            match hm.entry(allergen.to_string()) {
                Entry::Occupied(mut e) => {
                    let v = e.get();
                    let v = v.intersection(&hs).copied().collect();
                    e.insert(v);
                }
                Entry::Vacant(o) => {
                    o.insert(hs);
                }
            }

            for ingredient in ingredients {
                i.insert(ingredient.to_string());
            }
            a.insert(allergen.to_string());
        }
        // println!("{:?}", hm);
    }
    let mut possible_labels: Vec<(String, HashSet<&String>)> = hm.into_iter().collect();
    possible_labels.sort_by_key(|x| Reverse(x.1.len()));

    let mut labels = HashMap::new();
    while !possible_labels.is_empty() {
        let (allergen, possible_ing) = possible_labels.pop().unwrap();
        if possible_ing.len() != 1 {
            panic!("Possible ingredients are not 1!");
        }
        let v: Vec<&String> = possible_ing.into_iter().collect();
        labels.insert(v[0], allergen);
        for x in &mut possible_labels {
            x.1.remove(v[0]);
        }
        possible_labels.sort_by_key(|x| Reverse(x.1.len()));
    }
    println!("{:?}", labels);
    let mut count = 0;
    for (ingredients, _allergens) in &data {
        for ingredient in ingredients {
            if !labels.contains_key(ingredient) {
                count += 1;
            }
        }
    }

    let mut label_list: Vec<(&&String, &String)> = labels.iter().collect();

    label_list.sort_by_key(|x| x.1);

    println!(
        "{}",
        label_list
            .into_iter()
            .map(|x| *x.0)
            .cloned()
            .collect::<Vec<String>>()
            .join(",")
    );

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(star_one(Cursor::new(input)), 5);
    }

    #[test]
    fn test_star_two() {
        let input = b"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(star_two(Cursor::new(input)), 5);
    }
}
