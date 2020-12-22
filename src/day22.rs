use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

#[allow(dead_code, unused_variables)]
pub fn star_one(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut players: Vec<VecDeque<usize>> = input_str
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|x| {
                    x.parse::<usize>()
                        .expect(&format!("Could not parse card: {}", x))
                })
                .collect()
        })
        .collect();
    let mut player2 = players.pop().unwrap();
    let mut player1 = players.pop().unwrap();
    while player2.len() > 0 && player1.len() > 0 {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        match (p1, p2) {
            (a, b) if a > b => {
                player1.push_back(a);
                player1.push_back(b);
            }
            (a, b) if a < b => {
                player2.push_back(b);
                player2.push_back(a);
            }
            _ => panic!("Cards are equal"),
        }
    }

    if player1.len() > 0 {
        println!("Player1");
        println!("{:?}", player1);
        player1
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * x)
            .sum()
    } else {
        println!("{:?}", player2);
        player2
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * x)
            .sum()
    }
}

fn get_hash(player1: &VecDeque<usize>, player2: &VecDeque<usize>) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    player1.hash(&mut hasher);
    player2.hash(&mut hasher);
    hasher.finish()
}

fn play_star_two(
    mut player1: VecDeque<usize>,
    mut player2: VecDeque<usize>,
    game: usize,
) -> (usize, VecDeque<usize>) {
    let mut played = HashSet::new();
    loop {
        let game_hash = get_hash(&player1, &player2);
        if played.contains(&game_hash) {
            break (1, player1);
        } else if player2.len() == 0 {
            break (1, player1);
        } else if player1.len() == 0 {
            break (2, player2);
        }

        played.insert(game_hash);

        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        let winner = if player1.len() >= p1 && player2.len() >= p2 {
            let player1 = player1.iter().take(p1).copied().collect();
            let player2 = player2.iter().take(p2).copied().collect();
            play_star_two(player1, player2, game + 1).0
        } else if p1 > p2 {
            1
        } else if p2 > p1 {
            2
        } else {
            panic!("Cards are equal")
        };

        if winner == 1 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
}

#[allow(dead_code, unused_variables)]
pub fn star_two(mut input: impl BufRead) -> usize {
    let mut input_str = String::new();
    input
        .read_to_string(&mut input_str)
        .expect("Could not read all of string");
    let mut players: Vec<VecDeque<usize>> = input_str
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|x| {
                    x.parse::<usize>()
                        .expect(&format!("Could not parse card: {}", x))
                })
                .collect()
        })
        .collect();
    let player2 = players.pop().unwrap();
    let player1 = players.pop().unwrap();

    let (winner, cards) = play_star_two(player1, player2, 1);
    cards
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i + 1) * x)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_star_one() {
        let input = b"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(star_one(Cursor::new(input)), 306);
    }

    #[test]
    fn test_infinite_start_two() {
        let input = b"Player 1:
43
19

Player 2:
2
29
14";
        assert_eq!(star_two(Cursor::new(input)), 105);
    }

    #[test]
    fn test_star_two() {
        let input = b"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        assert_eq!(star_two(Cursor::new(input)), 291);
    }
}
