use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::BufRead;

pub fn star_one(input: impl BufRead) -> usize {
    let mut data: Vec<usize> = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().parse().unwrap())
        .collect();
    data.sort();
    let target = data.iter().max().unwrap() + 3;
    let mut current = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 0;

    while current != target {
        current = if let Ok(_i) = data.binary_search(&(current + 1)) {
            diff_1 += 1;
            current + 1
        } else if let Ok(_i) = data.binary_search(&(current + 2)) {
            current + 2
        } else if let Ok(_i) = data.binary_search(&(current + 3)) {
            diff_3 += 1;
            current + 3
        } else {
            // We have reached the target.
            return diff_1 * (diff_3 + 1);
        }
    }
    diff_1 * (diff_3 + 1)
}

fn number_to_target(cache: &HashMap<usize, usize>, current: usize, target: usize) -> usize {
    // If we have reached the target there is only 1 path way.
    if current == target {
        return 1;
    }
    // Get the number of pathways to target by looking up the cache of possible next steps
    cache.get(&(current + 1)).unwrap_or(&0)
        + cache.get(&(current + 2)).unwrap_or(&0)
        + cache.get(&(current + 3)).unwrap_or(&0)
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut data: Vec<usize> = input
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().parse().unwrap())
        .collect();
    data.sort();
    let target = data.iter().max().unwrap() + 3;

    // We need to get process the largest values first.
    let mut stack = BinaryHeap::new();
    let mut cache = HashMap::new();
    let mut done = HashSet::new();
    // Work backwards from target so we can cache the number of paths that eventually
    // lead to the target.
    stack.push(target);

    while let Some(current) = stack.pop() {
        if !done.contains(&current) {
            done.insert(current);
            let number_of_paths = number_to_target(&cache, current, target);
            cache.insert(current, number_of_paths);
            if let Ok(_i) = data.binary_search(&(current.saturating_sub(1))) {
                stack.push(current - 1);
            }
            if let Ok(_i) = data.binary_search(&(current.saturating_sub(2))) {
                stack.push(current - 2);
            }

            if let Ok(_i) = data.binary_search(&(current.saturating_sub(3))) {
                stack.push(current - 3);
            }
        }
    }

    number_to_target(&cache, 0, target)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    use std::io::Cursor;

    #[test]
    fn test_star_one_a() {
        let input: &[u8; 27] = b"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(star_one(Cursor::new(input)), 35);
    }

    #[test]
    fn test_star_one_b() {
        let input = b"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(star_one(Cursor::new(input)), 220);
    }

    #[test]
    fn test_star_two_a() {
        let input: &[u8; 27] = b"16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(star_two(Cursor::new(input)), 8);
    }

    #[test]
    fn test_star_two_b() {
        let input = b"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(star_two(Cursor::new(input)), 19208);
    }
}
