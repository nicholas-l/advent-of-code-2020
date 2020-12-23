use std::{collections::HashMap, io::BufRead};

use vec_arena::Arena;
// https://github.com/smol-rs/vec-arena/blob/master/examples/linked-list.rs
/// The null index, akin to null pointers.
///
/// Just like a null pointer indicates an address no object is ever stored at,
/// the null index indicates an index no object is ever stored at.
///
/// Number `!0` is the largest possible value representable by `usize`.
const NULL: usize = !0;

#[derive(Debug)]
struct Node<T> {
    /// Previous node in the list.
    prev: usize,

    /// Next node in the list.
    next: usize,

    /// Actual value stored in node.
    value: T,
}

struct List<T> {
    /// This is where nodes are stored.
    arena: Arena<Node<T>>,

    /// First node in the list.
    head: usize,

    /// Last node in the list.
    tail: usize,
}

impl<T: Copy> List<T> {
    /// Constructs a new, empty doubly linked list.
    fn new() -> Self {
        List {
            arena: Arena::new(),
            head: NULL,
            tail: NULL,
        }
    }

    /// Returns the number of elements in the list.
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.arena.len()
    }

    /// Links nodes `a` and `b` together, so that `a` comes before `b` in the list.
    fn link(&mut self, a: usize, b: usize) {
        if a != NULL {
            self.arena[a].next = b;
        }
        if b != NULL {
            self.arena[b].prev = a;
        }
    }

    /// Appends `value` to the back of the list.
    fn push_back(&mut self, value: T) -> usize {
        let node = self.arena.insert(Node {
            prev: NULL,
            next: NULL,
            value: value,
        });

        let tail = self.tail;
        self.link(tail, node);

        self.tail = node;
        if self.head == NULL {
            self.head = node;
        }
        node
    }

    /// Pops and returns the value at the front of the list.
    fn pop_front(&mut self) -> Option<T> {
        self.arena.remove(self.head).map(|node| {
            self.link(NULL, node.next);
            self.head = node.next;
            if node.next == NULL {
                self.tail = NULL;
            }
            node.value
        })
    }

    fn to_vec(&self) -> Vec<T> {
        let mut res = vec![];
        let mut current = self.arena.get(self.head).unwrap();
        res.push(current.value);
        while let Some(node) = self.arena.get(current.next) {
            res.push(node.value);
            current = node;
        }
        res
    }

    /// Removes the element specified by `index`.
    #[allow(dead_code)]
    fn remove(&mut self, index: usize) -> T {
        let node = self.arena.remove(index).unwrap();

        self.link(node.prev, node.next);
        if self.head == index {
            self.head = node.next;
        }
        if self.tail == index {
            self.tail = node.prev;
        }

        node.value
    }

    fn insert(&mut self, index: usize, value: T) -> usize {
        if index != self.tail {
            let node = self.arena.insert(Node {
                prev: NULL,
                next: NULL,
                value: value,
            });
            let after = self.arena.get(index).unwrap().next;
            self.link(index, node);
            self.link(node, after);
            node
        } else {
            self.push_back(value)
        }
    }
}

#[allow(dead_code)]
fn step(v: &mut Vec<usize>, current: usize, print: bool) -> usize {
    let current_value = v[current];
    let mut d1 = v[current] - 1;
    let len = v.len();

    let to_remove1 = v[(current + 1) % v.len()];
    let to_remove2 = v[(current + 2) % v.len()];
    let to_remove3 = v[(current + 3) % v.len()];

    let c1 = v.remove(v.iter().position(|x| x == &to_remove1).unwrap());
    let c2 = v.remove(v.iter().position(|x| x == &to_remove2).unwrap());
    let c3 = v.remove(v.iter().position(|x| x == &to_remove3).unwrap());
    let min = *v.iter().min().unwrap();

    if print {
        println!("pick up: {} {} {}", c1, c2, c3);
    }
    // Find destination cup
    let d = loop {
        if d1 < min {
            break v.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
        } else if let Some(p) = v.iter().position(|x| x == &d1) {
            break p;
        } else {
            d1 -= 1;
        }
    };
    if print {
        println!("{:?} - {}", v, d);
    }
    v.rotate_right(len - 3 - d - 1);
    if print {
        println!("{:?}", v);
    }
    // x x x 4 5 6 7 8
    //
    if print {
        println!("destination: {} ({})", v[d], d);
    }
    // Insert new cups
    v.push(c1);
    v.push(c2);
    v.push(c3);

    v.rotate_left(len - d - 1);

    v.iter().position(|x| x == &current_value).unwrap()
}

fn step_linked(v: &mut List<usize>, lookup: &HashMap<usize, usize>, max: usize) -> usize {
    let current_value = v.pop_front().unwrap();
    let mut d1 = current_value - 1;
    let c1 = v.pop_front().unwrap();
    let c2 = v.pop_front().unwrap();
    let c3 = v.pop_front().unwrap();

    // Find destination cup value
    let d = loop {
        // Assume we dont have anythin less that 1
        if d1 < 1 {
            d1 = max;
        }
        if d1 != c1 && d1 != c2 && d1 != c3 {
            break d1;
        } else {
            d1 -= 1;
        }
    };

    v.insert(*lookup.get(&d).unwrap(), c3);
    // dbg!(&v.to_vec());
    v.insert(*lookup.get(&d).unwrap(), c2);
    v.insert(*lookup.get(&d).unwrap(), c1);
    // dbg!(&other);

    v.push_back(current_value);

    1
}

pub fn star_one(input: impl BufRead) -> usize {
    let v: Vec<usize> = input
        .bytes()
        .map(|x| (x.unwrap() - b'0') as usize)
        .collect();

    let max = *v.iter().max().unwrap();
    let (mut list, lookup) = create_inputs(v);
    for round in 0..100 {
        println!("-- move {} --", round + 1);
        // println!(
        //     "cups: {}",
        //     v.iter()
        //         .enumerate()
        //         .map(|(i, x)| if i == current_index {
        //             format!("({}) ", x)
        //         } else {
        //             format!("{} ", x)
        //         })
        //         .collect::<String>()
        // );
        println!("{:?}", list.to_vec());
        step_linked(&mut list, &lookup, max);
    }
    println!("-- final --");
    println!("{:?}", list.to_vec());
    // println!(
    //     "cups:  {}",
    //     v.iter()
    //         .enumerate()
    //         .map(|(i, x)| if i == current_index {
    //             format!("({}) ", x)
    //         } else {
    //             format!("{} ", x)
    //         })
    //         .collect::<String>()
    // );
    let mut res = vec![];
    let mut current = list.arena.get(*lookup.get(&1).unwrap()).unwrap();
    dbg!(current);
    while let Some(node) = list.arena.get(current.next) {
        // dbg!(node);
        res.push(node.value);
        current = node;
        if current.next == NULL {
            break;
        }
        if current.value == 1 {
            break;
        }
    }

    println!("first");
    dbg!(&res);
    dbg!(&current);
    let mut current = list.arena.get(list.head).unwrap();
    if current.value != 1 {
        res.push(current.value);
        while let Some(node) = list.arena.get(current.next) {
            if node.value == 1 {
                break;
            }
            res.push(node.value);
            current = node;
        }
    }

    println!("first");

    res.iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn create_inputs(cups: Vec<usize>) -> (List<usize>, HashMap<usize, usize>) {
    let mut list = List::new();
    let mut lookup = HashMap::new();
    for x in cups {
        lookup.insert(x, list.push_back(x));
    }
    (list, lookup)
}

pub fn star_two(input: impl BufRead) -> usize {
    let mut cups: Vec<usize> = input
        .bytes()
        .map(|x| (x.unwrap() - b'0') as usize)
        .collect();
    let max = *cups.iter().max().unwrap() + 1;
    let upper = 1_000_000;
    cups.extend(max..=upper);

    assert!(cups.contains(&1_000_000));

    let (mut list, lookup) = create_inputs(cups);

    let number_of_rounds = 10_000_000;

    let print_every = 10_000;
    for round in 0..number_of_rounds {
        step_linked(&mut list, &lookup, 1_000_000);
        if round % print_every == 0 {
            println!("{}", round as f64 * 100f64 / number_of_rounds as f64);
        }
    }
    // println!("-- final --");
    // println!(
    //     "cups:  {}",
    //     cups.iter()
    //         .enumerate()
    //         .map(|(i, x)| if i == current_index {
    //             format!("({}) ", x)
    //         } else {
    //             format!("{} ", x)
    //         })
    //         .collect::<String>()
    // );
    println!("Done, now to find 1");
    while let Some(x) = list.pop_front() {
        if x == 1 {
            break;
        }
    }
    let cup1 = list.pop_front().unwrap();
    let cup2 = list.pop_front().unwrap();
    cup1 * cup2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_step() {
        let mut v = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut current = 0;

        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 8, 9, 1, 5, 4, 6, 7]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 5, 4, 6, 7, 8, 9, 1]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![7, 2, 5, 8, 9, 1, 3, 4, 6]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![3, 2, 5, 8, 4, 6, 7, 9, 1]);

        current += 1;
        let current_index = step(&mut v, current, true);
        if (current as isize - current_index as isize) < 0 {
            v.rotate_left(current_index - current);
        } else {
            v.rotate_right(current - current_index);
        }
        assert_eq!(v, vec![9, 2, 5, 8, 4, 1, 3, 6, 7]);
    }

    #[test]
    fn test_star_one() {
        let input = b"389125467";
        assert_eq!(star_one(Cursor::new(input)), 67384529);
    }

    #[test]
    fn test_step_linked() {
        // let (list, lookup) = create_inputs(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        // step_linked(&mut list, lookup, 9);
        // let expected = vec![2, 8, 9, 1, 5, 4, 6, 7, 3].into_iter().collect();
        // assert_eq!(list, expected);

        // step_linked(&mut v, 9);
        // let mut expected2 = vec![5, 4, 6, 7, 8, 9, 1, 3, 2].into_iter().collect();
        // assert_eq!(v, expected2);

        // step_linked(&mut expected2, 9);
        // let mut expected3 = vec![8, 9, 1, 3, 4, 6, 7, 2, 5].into_iter().collect();
        // assert_eq!(expected2, expected3);

        // step_linked(&mut expected3, 9);
        // let mut expected4 = vec![4, 6, 7, 9, 1, 3, 2, 5, 8].into_iter().collect();
        // assert_eq!(expected3, expected4);

        // step_linked(&mut expected4, 9);
        // assert_eq!(
        //     expected4,
        //     vec![1, 3, 6, 7, 9, 2, 5, 8, 4].into_iter().collect()
        // );
    }

    #[test]
    fn test_star_two() {
        let input = b"389125467";
        assert_eq!(star_two(Cursor::new(input)), 149245887792);
    }
}
