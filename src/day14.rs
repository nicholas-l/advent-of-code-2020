use std::io::BufRead;
use std::collections::HashMap;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mem\[(?P<index>\d{1,})\] = (?P<value>\d{1,})").unwrap();
}

// enum MaskPoint {
//     Same,
//     One,
//     Zero,
// }

enum Instruction {
    Mask(Vec<char>),
    Memory(usize, usize),
}

#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let mut instructions = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let firstchars: String = line.chars().take(3).collect();
            match firstchars.as_str() {
                "mas" => Instruction::Mask(line.chars().skip(7).collect()),
                "mem" => {
                    let captures = RE
                        .captures(&line)
                        .expect(&format!("Could not match {} with regex", line));
                    let index = captures
                        .name("index")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    let value = captures
                        .name("value")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    Instruction::Memory(index, value)
                }
                _ => panic!(),
            }
        })
        .collect::<Vec<Instruction>>();

    let first = match instructions.remove(0) {
        Instruction::Mask(x) => x,
        _ => unreachable!(),
    };

    let hashmap = instructions
        .into_iter()
        .fold((HashMap::new(), first), |(mut hm, mask), instruction| match instruction {
            Instruction::Mask(chars) => (hm, chars),
            Instruction::Memory(index, value) => {
                let b: Vec<char> = (format!("{:b}", value)).chars().rev().collect();
                let binary_str = mask
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, c)| match (c, b.get(i)) {
                        ('X', Some(c)) => *c,
                        ('X', None) => '0',
                        ('1', _) => '1',
                        ('0', _) => '0',
                        _ => unreachable!(),
                    })
                    .rev()
                    .collect::<String>();
                let new_value = usize::from_str_radix(&binary_str, 2).unwrap();
                hm.insert(index, new_value);
                (hm, mask)
            }
        }).0;
        hashmap.values().sum()
        
}

fn get_addresses(mask: &Vec<char>, address: usize) -> Vec<usize> {
    // TODO maybe use minus and subtraction of base2 values.
    // Create lis tof masks
    let mut values = Vec::new();
    let b: Vec<char> = (format!("{:036b}", address)).chars().collect();
    values.push(b);
    
    println!("{}", mask.len());
    for i in 0..mask.len() {
        match mask[i] {
            'X' => {
                // copy values and update old[i] to 0 and new[i] to 1
                let new_values = values.clone();

                values.extend(new_values.into_iter().map(|mut opt|{
                    opt[i] = if opt[i] == '0' { '1' } else { '0' };
                    opt
                }));
                
            },
            '1' => {
                // set all values[i] to 1,
                for opt in &mut values {
                    opt[i] = '1';
                }
            }
            _ => {},
        }
    }

    values.into_iter().map(|value| {
        let s: String = value.into_iter().collect();
        usize::from_str_radix(&s, 2).unwrap()
    }).collect()
    
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let mut instructions = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let firstchars: String = line.chars().take(3).collect();
            match firstchars.as_str() {
                "mas" => Instruction::Mask(line.chars().skip(7).collect()),
                "mem" => {
                    let captures = RE
                        .captures(&line)
                        .expect(&format!("Could not match {} with regex", line));
                    let index = captures
                        .name("index")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    let value = captures
                        .name("value")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    Instruction::Memory(index, value)
                }
                _ => panic!(),
            }
        })
        .collect::<Vec<Instruction>>();

    let first = match instructions.remove(0) {
        Instruction::Mask(x) => x,
        _ => unreachable!(),
    };

    let hashmap = instructions
        .into_iter()
        .fold((HashMap::new(), first), |(mut hm, mask), instruction| match instruction {
            Instruction::Mask(chars) => (hm, chars),
            Instruction::Memory(index, value) => {
                for new_index in get_addresses(&mask, index) {
                    hm.insert(new_index, value);
                }
                (hm, mask)
            }
        }).0;
        hashmap.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_get_addresses() {
        let mask = "00000000000000000000000000000000X0XX".chars().collect();
        let mut addresses = get_addresses(&mask, 26);
        addresses.sort();
        assert_eq!(addresses, vec![16, 17, 18, 19, 24, 25, 26, 27]);
    }

    #[test]
    fn test_star_one() {
        let input = b"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(star_one(Cursor::new(input)), 165);
    }

    #[test]
    fn test_star_two() {
        let input = b"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(star_two(Cursor::new(input)), 208);
    }
}
