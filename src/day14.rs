use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mem\[(?P<index>\d{1,})\] = (?P<value>\d{1,})").unwrap();
}

#[derive(PartialEq, Copy, Clone)]
enum MaskPoint {
    X,
    One,
    Zero,
}

impl TryFrom<char> for MaskPoint {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(MaskPoint::X),
            '0' => Ok(MaskPoint::Zero),
            '1' => Ok(MaskPoint::One),
            _ => Err(format!("Could not match {} to MaskPoint", c)),
        }
    }
}

impl TryFrom<MaskPoint> for char {
    type Error = String;
    fn try_from(c: MaskPoint) -> Result<Self, Self::Error> {
        match c {
            MaskPoint::X => Ok('X'),
            MaskPoint::Zero => Ok('0'),
            MaskPoint::One => Ok('1'),
        }
    }
}

enum Instruction {
    Mask(Vec<MaskPoint>),
    Memory(usize, usize),
}

pub fn star_one(input: impl BufRead) -> usize {
    let mut instructions = parse_input(input).collect::<Vec<Instruction>>();

    let first = match instructions.remove(0) {
        Instruction::Mask(x) => x,
        _ => unreachable!(),
    };

    let hashmap = instructions
        .into_iter()
        .fold(
            (HashMap::new(), first),
            |(mut hm, mask), instruction| match instruction {
                Instruction::Mask(chars) => (hm, chars),
                Instruction::Memory(index, value) => {
                    let b: Vec<char> = (format!("{:036b}", value)).chars().rev().collect();
                    let binary_str = mask
                        .iter()
                        .rev()
                        .enumerate()
                        .map(|(i, c)| match (c, b.get(i)) {
                            (MaskPoint::X, Some(c)) => *c,
                            (MaskPoint::X, None) => '0',
                            (MaskPoint::One, _) => '1',
                            (MaskPoint::Zero, _) => '0',
                        })
                        .rev()
                        .collect::<String>();
                    let new_value = usize::from_str_radix(&binary_str, 2).unwrap();
                    hm.insert(index, new_value);
                    (hm, mask)
                }
            },
        )
        .0;
    hashmap.values().sum()
}

fn from_maskpoints(maskpoints: Vec<MaskPoint>) -> usize {
    usize::from_str_radix(
        &maskpoints
            .into_iter()
            .map(|x| match x {
                MaskPoint::X => 'X',
                MaskPoint::One => '1',
                MaskPoint::Zero => '0',
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

fn set_addresses(address: Vec<MaskPoint>, mut set: impl FnMut(usize)) {
    let mut stack = Vec::new();
    stack.push(address);

    while let Some(mut address) = stack.pop() {
        if let Some(pos) = address.iter().position(|&p| p == MaskPoint::X) {
            let mut left = address.clone();
            left[pos] = MaskPoint::One;
            stack.push(left);
            address[pos] = MaskPoint::Zero;
            stack.push(address);
        } else {
            set(from_maskpoints(address));
        }
    }
}

fn parse_input(input: impl BufRead) -> impl Iterator<Item = Instruction> {
    input.lines().filter_map(Result::ok).map(|line| {
        let firstchars: String = line.chars().take(3).collect();
        match firstchars.as_str() {
            "mas" => Instruction::Mask(
                line.chars()
                    .skip(7)
                    .map(MaskPoint::try_from)
                    .filter_map(Result::ok)
                    .collect(),
            ),
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
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut instructions = parse_input(input).collect::<Vec<Instruction>>();

    let first = match instructions.remove(0) {
        Instruction::Mask(x) => x
            .into_iter()
            .enumerate()
            .filter(|(_, x)| x != &MaskPoint::Zero)
            .collect::<HashMap<usize, MaskPoint>>(),
        _ => unreachable!(),
    };

    let hashmap = instructions
        .into_iter()
        .fold(
            (HashMap::new(), first),
            |(mut hm, mask), instruction| match instruction {
                Instruction::Mask(chars) => (
                    hm,
                    chars
                        .into_iter()
                        .enumerate()
                        .filter(|(_, x)| x != &MaskPoint::Zero)
                        .collect::<HashMap<usize, MaskPoint>>(),
                ),
                Instruction::Memory(index, value) => {
                    let adress_str = (format!("{:036b}", index))
                        .chars()
                        .enumerate()
                        .map(|(i, x)| *mask.get(&i).unwrap_or(&MaskPoint::try_from(x).unwrap()))
                        .collect();
                    set_addresses(adress_str, |new_index| {
                        hm.insert(new_index, value);
                    });
                    (hm, mask)
                }
            },
        )
        .0;
    hashmap.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

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
        let input = b"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(star_two(Cursor::new(input)), 208);
    }
}
