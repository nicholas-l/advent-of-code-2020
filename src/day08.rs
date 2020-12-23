use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Accumulate(isize),
    Jump(isize),
    NoOperation(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<&str> = s.split(" ").collect();
        match chars[0] {
            "acc" => {
                Ok(Instruction::Accumulate(chars[1].parse::<isize>().expect(
                    &format!("Could not convert {} to usize", chars[1]),
                )))
            }
            "jmp" => Ok(Instruction::Jump(chars[1].parse().unwrap())),
            "nop" => Ok(Instruction::NoOperation(chars[1].parse().unwrap())),
            _ => panic!(format!("Could not parse {} to Instruction", s)),
        }
    }
}

enum Error {
    InfiniteLoop(usize),
    PointerOverflow,
}

pub fn star_one(input: impl BufRead) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect();

    run(&instructions, instructions.len())
        .map_err(|err| match err {
            Error::InfiniteLoop(x) => x,
            _ => panic!(),
        })
        .unwrap_err()
}

fn run(instructions: &[Instruction], flip: usize) -> Result<usize, Error> {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut pointer = 0;
    let mut acc = 0_isize;
    loop {
        if pointer == instructions.len() {
            break Ok(acc as usize);
        }
        if pointer > instructions.len() {
            break Err(Error::PointerOverflow);
        }
        if visited.contains(&pointer) {
            break Err(Error::InfiniteLoop(acc as usize));
        } else {
            visited.insert(pointer);
            pointer = match (flip == pointer, &instructions[pointer]) {
                (false, Instruction::NoOperation(_)) => pointer + 1,
                (true, Instruction::NoOperation(value)) => (pointer as isize + value) as usize,
                (false, Instruction::Jump(value)) => (pointer as isize + value) as usize,
                (true, Instruction::Jump(_)) => pointer + 1,
                (_, Instruction::Accumulate(value)) => {
                    acc = acc + value;
                    pointer + 1
                }
            }
        }
    }
}

pub fn star_two(input: impl BufRead) -> usize {
    let instructions: Vec<Instruction> = input
        .lines()
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect();
    instructions
        .iter()
        .enumerate()
        // Remove acculumator instructions
        .filter(|(_index, instruction)| match instruction {
            Instruction::Accumulate(_) => false,
            _ => true,
        })
        .map(|x| x.0)
        .filter_map(|index| run(&instructions, index).ok())
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    const INPUT: &[u8] = b"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(Cursor::new(INPUT)), 5);
    }

    #[test]
    fn test_star_two() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";
        assert_eq!(star_two(Cursor::new(input)), 8);
    }
}
