use regex::Regex;
use std::io::BufRead;
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"(?P<field>[\w ]+): (?P<min1>\d{1,})-(?P<max1>\d{1,}) or (?P<min2>\d{1,})-(?P<max2>\d{1,})"
    )
    .unwrap();
}

fn get_valid_numbers(input: &str) -> HashSet<usize> {
    input
        .lines()
        .flat_map(|line| {
            let captures = RE
                .captures(&line)
                .expect("Bad line that does not match regex.");
            let min1 = captures["min1"].parse::<usize>().unwrap();
            let max1 = captures["max1"].parse::<usize>().unwrap();
            let min2 = captures["min2"].parse::<usize>().unwrap();
            let max2 = captures["max2"].parse::<usize>().unwrap();
            (min1..=max1).chain(min2..=max2)
        })
        .collect()
}

#[allow(dead_code, unused_variables)]
pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut sections = input_str.split("\n\n");

    let valid_numbers: HashSet<usize> = get_valid_numbers(sections.next().unwrap());

    let _my_ticket = sections.next().unwrap();
    let nearby_tickets = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|line| line.split(',').map(|x| x.parse::<usize>().unwrap()));
    nearby_tickets
        .filter(|x| !valid_numbers.contains(x))
        .sum()
}

fn get_field_validation(input: &str) -> HashMap<String, (usize, usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let captures = RE
                .captures(&line)
                .expect("Bad line that does not match regex.");
            let field_name = &captures["field"];
            let min1 = captures["min1"].parse::<usize>().unwrap();
            let max1 = captures["max1"].parse::<usize>().unwrap();
            let min2 = captures["min2"].parse::<usize>().unwrap();
            let max2 = captures["max2"].parse::<usize>().unwrap();
            (field_name.to_string(), (min1, max1, min2, max2))
        })
        .collect()
}

fn validate_field(x: usize, (min1, max1, min2, max2): (usize, usize, usize, usize)) -> bool {
    (x >= min1 && x <= max1) || (x >= min2 && x <= max2)
}

#[allow(dead_code, unused_variables)]
pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut sections = input_str.split("\n\n");

    let field_type_str = sections.next().unwrap();

    let valid_numbers: HashSet<usize> = get_valid_numbers(field_type_str);
    let field_validation: HashMap<String, (usize, usize, usize, usize)> =
        get_field_validation(field_type_str);

    let my_ticket = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();
    let valid_nearby_tickets: Vec<Vec<usize>> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|ticket| ticket.iter().all(|x| valid_numbers.contains(x)))
        .collect();
    let number_of_fields = valid_nearby_tickets[0].len();
    let field_data: Vec<Vec<usize>> = (0..number_of_fields)
        .map(|i| {
            valid_nearby_tickets
                .iter()
                .map(|ticket| ticket[i])
                .collect()
        })
        .collect();

    let mut possible_fields: Vec<(&String, HashSet<usize>)> = field_validation
        .iter()
        .map(|(key, value)| {
            (
                key,
                (0..number_of_fields)
                    .filter(|i| field_data[*i].iter().all(|&x| validate_field(x, *value)))
                    .collect::<HashSet<usize>>(),
            )
        })
        .collect();

    possible_fields.sort_by_key(|x| Reverse(x.1.len()));


    let mut field_names = vec![""; number_of_fields];

    while let Some(x) = possible_fields.pop() {
        if x.1.len() != 1 {
            panic!("Not equal to 1: {:?}", x.1);
        }
        let pos = x.1.iter().next().unwrap();
        for field in &mut possible_fields {
            field.1.remove(pos);
        }
        field_names[*pos] = x.0;
    }

    field_names
        .into_iter()
        .enumerate()
        .filter(|(i, name)| name.starts_with("departure"))
        .map(|(i, name)| my_ticket[i])
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(star_one(Cursor::new(input)), 71);
    }

    #[test]
    #[ignore]
    fn test_star_two() {
        let input = b"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(star_two(Cursor::new(input)), 175594);
    }
}
