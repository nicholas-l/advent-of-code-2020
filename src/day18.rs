// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while_m_n},
//     character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
//     combinator::map_res,
//     sequence::{delimited, preceded, terminated, tuple},
//     IResult,
// };
use std::collections::VecDeque;
use std::io::BufRead;

// enum Expression {
//     Addition(Box<Expression>, Box<Expression>),
//     Multiply(Box<Expression>, Box<Expression>),
//     Value(usize),
//     Brackets(Box<Expression>),
// }

// impl Expression {
//     fn eval(&self) -> usize {
//         match self {
//             Expression::Addition(a, b) => a.eval() + b.eval(),
//             Expression::Multiply(left, right) => left.eval() + right.eval(),
//             Expression::Value(value) => *value,
//             Expression::Brackets(inner) => inner.eval(),
//         }
//     }
// }

// fn parse_number(i: &str) -> IResult<&str, Expression> {
//     map_res(digit1, |digit_str: &str| {
//         digit_str.parse::<usize>().map(Expression::Value)
//     })(i)
// }

// fn parse(input: &str) -> IResult<&str, Expression> {
//     preceded(
//         multispace0,
//         parse_number, //, parse_application, parse_if, parse_quote)),
//     )(input)
// }

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mult,
    Value(usize),
    LeftParen,
    RightParen,
}

fn eval_star_one(input: VecDeque<Op>) -> VecDeque<Op> {
    let parens_result = eval_parens(input, eval_star_one);
    eval_addition_multiply(parens_result)
}

fn eval_addition_multiply(mut input: VecDeque<Op>) -> VecDeque<Op> {
    let mut result = VecDeque::new();
    while let Some(c) = input.pop_front() {
        if c == Op::Add {
            let a = match result.pop_back() {
                Some(Op::Value(v)) => v,
                v => {
                    println!("Res: {:?}", input);
                    println!("Add: {:?}", result);
                    panic!("{:?}", v)
                }
            };
            // Get next char
            let b = match input.pop_front() {
                Some(Op::Value(v)) => v,
                _ => panic!(),
            };
            result.push_back(Op::Value(a + b));
        } else if c == Op::Mult {
            let a = match result.pop_back() {
                Some(Op::Value(v)) => v,
                v => panic!("{:?}", v),
            };
            // Get next char
            let b = match input.pop_front() {
                Some(Op::Value(v)) => v,
                x => panic!(format!("{:?}", x)),
            };
            result.push_back(Op::Value(a * b));
        } else {
            result.push_back(c);
        }
    }
    result
}

fn eval_star_two(input: VecDeque<Op>) -> VecDeque<Op> {
    let parens_result = eval_parens(input, eval_star_two);
    let add_result = eval_addition(parens_result);

    eval_multiply(add_result)
}

fn eval_parens(
    mut input: VecDeque<Op>,
    outer: impl Fn(VecDeque<Op>) -> VecDeque<Op>,
) -> VecDeque<Op> {
    let mut result = VecDeque::new();
    while let Some(e) = input.pop_front() {
        if e == Op::LeftParen {
            let mut left_brackets = 1;
            let mut sub = VecDeque::new();
            while left_brackets > 0 {
                let e2 = input.pop_front().unwrap();
                if e2 == Op::LeftParen {
                    left_brackets += 1;
                } else if e2 == Op::RightParen {
                    left_brackets -= 1;
                }
                sub.push_back(e2);
            }
            sub.pop_back(); // last should be a right paren.
            let res = outer(sub);
            result.extend(res);
        } else {
            result.push_back(e);
        }
    }
    result
}

fn eval_addition(mut input: VecDeque<Op>) -> VecDeque<Op> {
    let mut result = VecDeque::new();
    while let Some(c) = input.pop_front() {
        if c == Op::Add {
            let a = match result.pop_back() {
                Some(Op::Value(v)) => v,
                v => {
                    println!("Res: {:?}", input);
                    println!("Add: {:?}", result);
                    panic!("{:?}", v)
                }
            };
            // Get next char
            let b = match input.pop_front() {
                Some(Op::Value(v)) => v,
                _ => panic!(),
            };
            result.push_back(Op::Value(a + b));
        } else {
            result.push_back(c);
        }
    }
    result
}

fn eval_multiply(mut input: VecDeque<Op>) -> VecDeque<Op> {
    let mut result = VecDeque::new();
    while let Some(c) = input.pop_front() {
        if c == Op::Mult {
            let a = match result.pop_back() {
                Some(Op::Value(v)) => v,
                v => panic!("{:?}", v),
            };
            // Get next char
            let b = match input.pop_front() {
                Some(Op::Value(v)) => v,
                x => panic!(format!("{:?}", x)),
            };
            result.push_back(Op::Value(a * b));
        } else {
            result.push_back(c);
        }
    }
    result
}

pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    // let (input, expression) = parse(&input_str).unwrap();
    // println!("{}", input);
    // expression.eval()
    input_str
        .lines()
        .map(|line| {
            let chars: Vec<Op> = line
                .chars()
                .filter(|c| c != &' ')
                .map(|c| match c {
                    '+' => Op::Add,
                    '*' => Op::Mult,
                    c @ '0'..='9' => Op::Value(c.to_digit(10).unwrap() as usize),
                    '(' => Op::LeftParen,
                    ')' => Op::RightParen,
                    _ => panic!("basklf"),
                })
                .collect();
            let res = eval_star_one(chars.into());
            if res.len() != 1 {
                panic!("Not equal to 1");
            }
            match res[0] {
                Op::Value(x) => x,
                _ => panic!("blah"),
            }
        })
        .sum()
}

pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    // let (input, expression) = parse(&input_str).unwrap();
    // println!("{}", input);
    // expression.eval()
    input_str
        .lines()
        .map(|line| {
            let chars: Vec<Op> = line
                .chars()
                .filter(|c| c != &' ')
                .map(|c| match c {
                    '+' => Op::Add,
                    '*' => Op::Mult,
                    c @ '0'..='9' => Op::Value(c.to_digit(10).unwrap() as usize),
                    '(' => Op::LeftParen,
                    ')' => Op::RightParen,
                    _ => panic!("basklf"),
                })
                .collect();
            let res = eval_star_two(chars.into());
            if res.len() != 1 {
                panic!("Not equal to 1");
            }
            match res[0] {
                Op::Value(x) => x,
                _ => panic!("blah"),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        {
            let input = b"1 + 2 * 3 + 4 * 5 + 6";
            assert_eq!(star_one(Cursor::new(input)), 71);
        }
        {
            let input = b"2 * 3 + (4 * 5)";
            assert_eq!(star_one(Cursor::new(input)), 26);
        }
        {
            let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
            assert_eq!(star_one(Cursor::new(input)), 437);
        }
        {
            let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
            assert_eq!(star_one(Cursor::new(input)), 12240);
        }
        {
            let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
            assert_eq!(star_one(Cursor::new(input)), 13632);
        }
    }
    #[test]
    fn test_star_two() {
        {
            let input = b"1 + 2 * 3 + 4 * 5 + 6";
            assert_eq!(star_two(Cursor::new(input)), 231);
        }
        {
            let input = b"2 * 3 + (4 * 5)";
            assert_eq!(star_two(Cursor::new(input)), 46);
        }
        {
            let input = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
            assert_eq!(star_two(Cursor::new(input)), 1445);
        }
        {
            let input = b"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
            assert_eq!(star_two(Cursor::new(input)), 669060);
        }
        {
            let input = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
            assert_eq!(star_two(Cursor::new(input)), 23340);
        }
    }
}
