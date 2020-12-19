use std::{collections::HashMap, io::BufRead};

#[derive(Debug, PartialEq)]
enum Rule {
    Value(char),
    Ref(usize),
    Alt(Vec<Rule>),
    Multiple(Vec<Rule>),
}

fn parse_single(input: &str) -> Rule {
    if input.contains("\"") {
        let v: Vec<char> = input.chars().collect();
        if v.len() != 3 {
            panic!("Input length is not a char: {}", input)
        }
        return Rule::Value(v[1]);
    } else if let Ok(value) = usize::from_str_radix(input, 10) {
        return Rule::Ref(value);
    } else {
        panic!("Unable to parse");
    }
}

fn parse_rule(input: &[&str]) -> Rule {
    if input.contains(&"|") {
        Rule::Alt(
            input
                .split(|x| x == &"|")
                .map(|rule| parse_rule(rule))
                .collect(),
        )
    } else if input.len() == 1 {
        parse_single(input[0])
    } else {
        Rule::Multiple(input.iter().map(|r| parse_single(r)).collect())
    }
}
/// This function returns None if the rule cannot match the input or Some if it can.
/// The vector is the possible leftover input, which is important 
/// for backtracking when a branch does not complete.
/// None => Failed to parse the input.
/// Some(vec![]) => Completed successfully with no more input rest to parse.
/// Some(vec![['a'], ['a', 'b']]) => two different possibilities that are left to parsed
///     that have been successfully parsed.
/// To verify that a rule matches an input use `res.map(|r| r.iter().any(|r| r.len() == 0)).unwrap_or(false)`
fn match_rule<'a>(
    map: &HashMap<usize, Rule>,
    rule: &Rule,
    input: &'a [char],
) -> Option<Vec<&'a [char]>> {
    // println!("{:?}: {:?}", rule, input);
    match rule {
        Rule::Multiple(v) => {
            let remainder = &input[..];
            let mut new_remainders = vec![remainder];
            for rule in v {
                new_remainders = new_remainders
                    .into_iter()
                    .filter_map(|remainder| match_rule(map, rule, remainder))
                    .flatten()
                    .collect();
                if new_remainders.len() == 0 {
                    return None;
                }
            }
            Some(new_remainders)
        }
        Rule::Value(c) => {
            if input.len() > 0 && input[0] == *c {
                Some(vec![&input[1..]])
            } else {
                None
            }
        }
        Rule::Ref(i) => {
            let new_rule = map.get(&i).expect("Could not find rule in match rule");
            match_rule(map, new_rule, &input[..])
        }
        Rule::Alt(v) => {
            let res: Vec<&[char]> = v.iter()
                    .filter_map(|rule| match_rule(map, rule, &input[..]))
                    .flatten()
                    .collect();
            if res.len() == 0 {
                None
            } else {
                Some(res)
            }
        }
    }
}


fn parse_rule_line(line: &str) -> (usize, Rule) {
    let index = line.split(':').next().unwrap().parse::<usize>().unwrap();
    let rule_string: Vec<&str> = line
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let rule = parse_rule(&rule_string[..]);
    (index, rule)
}

fn parse_input(mut input: impl BufRead, overrides: Option<&str>) -> (String, HashMap<usize, Rule>) {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut sections = input_str.split("\n\n");
    let mut rules: HashMap<usize, Rule> = sections
        .next()
        .unwrap()
        .lines()
        .map(parse_rule_line)
        .collect();
    if let Some(overrides) = overrides {
        for line in overrides.lines() {
            let (index, rule) = parse_rule_line(line);
            rules.insert(index, rule);
        }
    }
    let values = sections.next().unwrap();
    (values.to_string(), rules)
}


#[allow(dead_code, unused_variables)]
pub fn star_one(input: impl BufRead) -> usize {
    let (values, rules) = parse_input(input, None);
    let rule0 = rules.get(&0).expect("Rule 0 not found");
    values
        .lines()
        .filter(|line| {
            let chars: Vec<char> = line.chars().collect();
            let res = match_rule(&rules, rule0, &chars[..]);
            res.map(|r| r.iter().any(|r| r.len() == 0)).unwrap_or(false)
        })
        .count()
}

#[allow(dead_code, unused_variables)]
pub fn star_two(input: impl BufRead) -> usize {
    let overrides = "8: 42 | 42 8
11: 42 31 | 42 11 31";
    let (values, rules) = parse_input(input, Some(overrides));

    let rule0 = rules.get(&0).expect("Rule 0 not found");

    values
        .lines()
        .filter(|line| {
            let chars: Vec<char> = line.trim().chars().collect();
            let res = match_rule(&rules, rule0, &chars[..]);
            // Check if any of the inputs left to parse is empty.
            res.map(|r| r.iter().any(|r| r.len() == 0)).unwrap_or(false)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const INPUT: &[u8] = b"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_star_one() {
        let input = b"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
        assert_eq!(star_one(Cursor::new(input)), 2);
    }

    #[test]
    fn test_match_rule_alt() {
        let input = b"0: 1 | 2
1: \"a\"
2: \"b\"

test";
        let (_, rules) = parse_input(Cursor::new(input), None);
        let test: Vec<char> = "a".chars().collect();
        assert_eq!(
            match_rule(&rules, rules.get(&0).unwrap(), &test[..]),
            Some(vec![&[] as &[char]])
        );
    }

    #[test]
    fn test_match_rule() {
        let overrides = "8: 42 | 42 8
11: 42 31 | 42 11 31";
        let (_, rules) = parse_input(Cursor::new(INPUT), Some(overrides));
        let test: Vec<char> = "babbbbaabbbbbabbbbbbaabaaabaaa".chars().collect();
        let output = match_rule(&rules, rules.get(&0).unwrap(), &test[..]);
        assert_eq!(output.map(|r| r.iter().any(|r| r.len() == 0)), Some(true));
    }

    #[test]
    fn test_parse_rule() {
        {
            let (index, rule) = parse_rule_line("8: 42 | 42 8");
            let expected = Rule::Alt(vec![
                Rule::Ref(42),
                Rule::Multiple(vec![Rule::Ref(42), Rule::Ref(8)]),
            ]);
            assert_eq!(index, 8);
            assert_eq!(rule, expected);
        }

        {
            let (index, rule) = parse_rule_line("11: 42 31 | 42 11 31");
            let expected = Rule::Alt(vec![
                Rule::Multiple(vec![Rule::Ref(42), Rule::Ref(31)]),
                Rule::Multiple(vec![Rule::Ref(42), Rule::Ref(11), Rule::Ref(31)]),
            ]);
            assert_eq!(index, 11);
            assert_eq!(rule, expected);
        }
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(Cursor::new(INPUT)), 12);
    }
}
